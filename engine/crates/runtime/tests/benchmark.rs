//! The benchmark's numbers (`opence_runtime::benchmark`): the CPU frame and
//! the simulation tick, measured from the game's own profile sections.

use opence_runtime::benchmark::{frames, percentile, report};
use opence_runtime::Span;

fn span(name: &str, start_ns: u64, end_ns: u64) -> Span {
    Span {
        name: name.to_owned(),
        start_ns,
        end_ns,
    }
}

/// a frame of the main loop starting at `start`: input, a tick, the
/// render, a throttle of `throttle` and the present
fn frame(start: u64, work: u64, throttle: u64) -> Vec<Span> {
    vec![
        span("input", start, start + 10),
        span("game_tick.cheats_enforce", start + 20, start + 30),
        span("game_tick.player_effect", start + 40, start + 50),
        span("throttle", start + work, start + work + throttle),
        span(
            "present",
            start + work + throttle,
            start + work + throttle + 5,
        ),
    ]
}

#[test]
fn the_nearest_rank_percentile_of_a_sorted_list() {
    let values: Vec<u64> = (1..=100).collect();
    assert_eq!(percentile(&values, 50), Some(50));
    assert_eq!(percentile(&values, 99), Some(99));
    assert_eq!(percentile(&values, 100), Some(100));
    assert_eq!(percentile(&[7], 99), Some(7));
    assert_eq!(percentile(&[], 99), None);
}

#[test]
fn a_frame_is_from_one_input_to_the_next_less_its_throttle() {
    let mut spans = Vec::new();
    spans.extend(frame(0, 1_000, 500));
    spans.extend(frame(2_000, 3_000, 100));
    spans.extend(frame(6_000, 1_000, 0));
    let report = report(&spans);
    let frames = report.frame.as_ref().map(|summary| summary.count);
    // (the last frame has no next input, so it is not counted)
    assert_eq!(frames, Some(2));
    // 2000 - 500 and 4000 - 100
    assert_eq!(
        report.frame.map(|summary| (summary.p50_ns, summary.max_ns)),
        Some((1_500, 3_900))
    );
}

#[test]
fn a_tick_is_from_its_first_step_s_entry_to_its_last_step_s_exit() {
    let mut spans = frame(0, 1_000, 0);
    // a frame with two ticks
    spans.push(span("game_tick.cheats_enforce", 100, 110));
    spans.push(span("game_tick.player_effect", 300, 400));
    // a last step with no first: not a tick
    spans.push(span("game_tick.player_effect", 900, 950));
    spans.sort_by_key(|span| span.end_ns);
    let report = report(&spans);
    let ticks = report
        .tick
        .map(|summary| (summary.count, summary.p50_ns, summary.max_ns));
    assert_eq!(ticks, Some((2, 30, 300)));
}

#[test]
fn frames_before_the_first_tick_are_the_load_and_are_left_out() {
    // a long load, then the first tick and two frames
    let mut spans = vec![span("input", 0, 10), span("throttle", 20, 30)];
    spans.extend(frame(1_000_000, 1_000, 0));
    spans.extend(frame(1_002_000, 1_000, 0));
    spans.extend(frame(1_004_000, 1_000, 0));
    let report = report(&spans);
    assert_eq!(
        report.frame.map(|summary| (summary.count, summary.max_ns)),
        Some((2, 2_000))
    );
}

#[test]
fn no_spans_make_no_numbers() {
    let report = report(&[]);
    assert!(report.frame.is_none() && report.tick.is_none());
}

#[test]
fn each_section_s_spread_is_kept_by_its_name() {
    let spans = [
        span("present", 0, 10),
        span("present", 20, 50),
        span("input", 0, 4),
    ];
    let sections = opence_runtime::benchmark::sections(&spans);
    let present = sections
        .get("present")
        .map(|summary| (summary.count, summary.max_ns));
    assert_eq!(present, Some((2, 30)));
    let names: Vec<&str> = sections.keys().map(String::as_str).collect();
    assert_eq!(names, ["input", "present"]);
}

#[test]
fn the_first_thread_s_sleeps_are_left_out_of_its_frame_once() {
    let mut spans = frame(0, 1_000, 200);
    spans.extend(frame(2_000, 1_000, 0));
    // the stock game's 30 Hz sleep, and one inside the throttle
    spans.push(span(opence_runtime::SLEEP, 100, 600));
    spans.push(span(opence_runtime::SLEEP, 1_050, 1_150));
    let report = report(&spans);
    // 2000 - 500 slept - 200 throttled (the sleep inside it once)
    assert_eq!(report.frame.map(|summary| summary.max_ns), Some(1_300));
}

#[test]
fn each_frame_s_length_is_kept_in_order() {
    let mut spans = Vec::new();
    spans.extend(frame(0, 1_000, 500));
    spans.extend(frame(2_000, 3_000, 100));
    spans.extend(frame(6_000, 1_000, 0));
    // (in the order they ran, as the renderer's GPU times are)
    assert_eq!(frames(&spans), [1_500, 3_900]);
    assert!(frames(&[]).is_empty());
}
