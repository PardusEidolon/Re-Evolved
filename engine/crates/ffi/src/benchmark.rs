//! The benchmark's report.
//!
//! The app plays game.wasm for `--benchmark` seconds and times its frames;
//! the report here turns their times into the mean, the percentiles, the
//! hitches and the pacing, and finds the regressions against a run kept
//! before.

/// A run's frame times, summed up.
#[derive(Clone, Debug, PartialEq, uniffi::Record)]
pub struct BenchmarkReport {
    /// The frames counted.
    pub frames: u32,
    /// The mean frame time, in milliseconds.
    pub mean_ms: f64,
    /// The median.
    pub p50_ms: f64,
    /// The 99th percentile.
    pub p99_ms: f64,
    /// The 99.9th percentile.
    pub p999_ms: f64,
    /// The frames over twice the target.
    pub hitches: u32,
    /// The frame pacing: the root mean square of the change from one
    /// frame's time to the next's, in milliseconds (0 when every frame
    /// takes as long as the one before).
    pub pacing_ms: f64,
}

/// The `percentile` (0 to 100) of `sorted`, by the nearest rank, as the
/// app's benchmark has always taken it.
fn percentile(percentile: f64, sorted: &[f64]) -> f64 {
    let Some(&last) = sorted.last() else {
        return 0.0;
    };
    // (less a hair, as 99.9% of 1000 is a hair over 999 in floating point)
    let rank = libm::ceil(percentile / 100.0 * sorted.len() as f64 - 1e-9) as usize;
    sorted.get(rank.saturating_sub(1)).copied().unwrap_or(last)
}

/// The report of `frame_times` (milliseconds, in the order drawn), whose
/// hitches are the frames over twice `target_ms`.
#[uniffi::export]
pub fn benchmark_report(frame_times: Vec<f64>, target_ms: f64) -> BenchmarkReport {
    let times: Vec<f64> = frame_times.into_iter().filter(|t| t.is_finite()).collect();
    let mut sorted = times.clone();
    sorted.sort_by(f64::total_cmp);
    let count = times.len();
    let mean = if count == 0 {
        0.0
    } else {
        times.iter().sum::<f64>() / count as f64
    };
    let changes: Vec<f64> = times.windows(2).map(|pair| pair[1] - pair[0]).collect();
    let pacing = if changes.is_empty() {
        0.0
    } else {
        libm::sqrt(changes.iter().map(|change| change * change).sum::<f64>() / changes.len() as f64)
    };
    BenchmarkReport {
        frames: u32::try_from(count).unwrap_or(u32::MAX),
        mean_ms: mean,
        p50_ms: percentile(50.0, &sorted),
        p99_ms: percentile(99.0, &sorted),
        p999_ms: percentile(99.9, &sorted),
        hitches: u32::try_from(times.iter().filter(|&&t| t > 2.0 * target_ms).count())
            .unwrap_or(u32::MAX),
        pacing_ms: pacing,
    }
}

/// The share a measure may grow by from the run kept before.
pub const REGRESSION: f64 = 0.05;

/// What `current` lost against `baseline`: each of the mean, the p99 and
/// the p99.9 more than 5% slower, and any hitch more, one line each.
#[uniffi::export]
pub fn benchmark_regressions(baseline: BenchmarkReport, current: BenchmarkReport) -> Vec<String> {
    let mut lost = Vec::new();
    for (name, before, now) in [
        ("mean", baseline.mean_ms, current.mean_ms),
        ("p99", baseline.p99_ms, current.p99_ms),
        ("p99.9", baseline.p999_ms, current.p999_ms),
    ] {
        if now > before * (1.0 + REGRESSION) {
            lost.push(format!(
                "{name} {now:.3} ms is {:.1}% over {before:.3} ms",
                (now / before - 1.0) * 100.0
            ));
        }
    }
    if current.hitches > baseline.hitches {
        lost.push(format!(
            "{} hitches, {} before",
            current.hitches, baseline.hitches
        ));
    }
    lost
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_report_has_the_mean_percentiles_hitches_and_pacing() {
        // (999 frames of 8 ms, one of 20: a hitch at a target of 8.33)
        let mut times = vec![8.0; 999];
        times.insert(500, 20.0);
        let report = benchmark_report(times, 8.33);
        assert_eq!(report.frames, 1000);
        assert!((report.mean_ms - 8.012).abs() < 1e-9, "{}", report.mean_ms);
        assert_eq!(report.p50_ms, 8.0);
        assert_eq!(report.p99_ms, 8.0);
        assert_eq!(report.p999_ms, 8.0);
        assert_eq!(report.hitches, 1);
        // (two changes of 12 over 999)
        assert!((report.pacing_ms - libm::sqrt(288.0 / 999.0)).abs() < 1e-9);
        let steady = benchmark_report(vec![5.0; 100], 8.33);
        assert_eq!(steady.pacing_ms, 0.0);
        assert_eq!(steady.hitches, 0);
        assert_eq!(benchmark_report(Vec::new(), 8.33).frames, 0);
    }

    #[test]
    fn the_p999_finds_the_slowest_of_a_thousand() {
        let mut times: Vec<f64> = (1..=1000).map(f64::from).collect();
        times.reverse();
        let report = benchmark_report(times, 1000.0);
        assert_eq!(report.p50_ms, 500.0);
        assert_eq!(report.p99_ms, 990.0);
        assert_eq!(report.p999_ms, 999.0);
    }

    #[test]
    fn a_regression_over_five_percent_fails() {
        let baseline = benchmark_report(vec![8.0; 100], 8.33);
        let within = benchmark_report(vec![8.0 * 1.04; 100], 8.33);
        assert!(benchmark_regressions(baseline.clone(), within).is_empty());
        let over = benchmark_report(vec![8.0 * 1.06; 100], 8.33);
        let lost = benchmark_regressions(baseline.clone(), over);
        assert_eq!(lost.len(), 3, "{lost:?}");
        assert!(lost[0].starts_with("mean"), "{lost:?}");
        let mut hitched = vec![8.0; 100];
        hitched[50] = 30.0;
        let lost = benchmark_regressions(baseline, benchmark_report(hitched, 8.33));
        assert!(lost.iter().any(|line| line.contains("hitches")), "{lost:?}");
    }
}
