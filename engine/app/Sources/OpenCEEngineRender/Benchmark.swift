// The benchmark: one of the engine's benchmark paths (devgulch, Blood
// Gulch, Blood Gulch with sixteen players: crates/ffi/src/benchmark.rs)
// played and drawn offscreen for a set time, as fast as the GPU goes, with
// up to three frames in flight as the window has. Each frame asks its
// caller for the world at that time. A frame's time is the time between
// the ends of two frames' GPU work, so it counts whichever of the CPU and
// the GPU is slower.

import Foundation
import Metal
import OpenCEEngineCore
import QuartzCore
import simd

/// The benchmark's frame times and what they were drawn at.
public struct BenchmarkResult: Sendable {
    public let width: Int
    public let height: Int
    public let seconds: Double
    /// Each frame's time, in milliseconds.
    public let frameTimes: [Double]
    /// Each frame's GPU time (from its first command to its last), in milliseconds.
    public let gpuTimes: [Double]
    /// Each frame's CPU time, in milliseconds, where it is measured apart
    /// (the game's: `game(...)`); none otherwise.
    public var cpuTimes: [Double] = []

    /// The game's frames (game.wasm through GameRenderer): each the slower of
    /// its CPU frame (the game's own, less its throttle and sleeps:
    /// `GameEnd.frameMs`) and its GPU work, as they would be with the frames
    /// pipelined. The lists are aligned at their ends: the GPU's begins with
    /// the load's frames, and the CPU's with the first tick.
    public static func game(
        width: Int, height: Int, seconds: Double, cpuTimes: [Double], gpuTimes: [Double]
    ) -> BenchmarkResult {
        let count = min(cpuTimes.count, gpuTimes.count)
        let cpu = Array(cpuTimes.suffix(count))
        let gpu = Array(gpuTimes.suffix(count))
        var result = BenchmarkResult(
            width: width, height: height, seconds: seconds, frameTimes: zip(cpu, gpu).map { max($0, $1) },
            gpuTimes: gpu)
        result.cpuTimes = cpu
        return result
    }

    /// The `percentile` (0 to 100) of `times`, by the nearest rank.
    public static func percentile(_ percentile: Double, of times: [Double]) -> Double {
        guard !times.isEmpty else { return 0 }
        let sorted = times.sorted()
        let rank = Int((percentile / 100 * Double(sorted.count)).rounded(.up)) - 1
        return sorted[min(max(rank, 0), sorted.count - 1)]
    }

    public var p50: Double { Self.percentile(50, of: frameTimes) }
    public var p99: Double { Self.percentile(99, of: frameTimes) }
    public var gpuP50: Double { Self.percentile(50, of: gpuTimes) }
    public var gpuP99: Double { Self.percentile(99, of: gpuTimes) }
    public var cpuP50: Double { Self.percentile(50, of: cpuTimes) }
    public var cpuP99: Double { Self.percentile(99, of: cpuTimes) }

    /// The frame times' report (the engine's: crates/ffi/src/benchmark.rs),
    /// whose hitches are frames over twice `targetMs`.
    public func report(targetMs: Double) -> BenchmarkReport {
        benchmarkReport(frameTimes: frameTimes, targetMs: targetMs)
    }

    /// The result of `path` as JSON, for tools/test_engine_benchmark.py.
    public func json(path: String, targetMs: Double) -> String {
        func number(_ value: Double) -> String { String(format: "%.3f", value) }
        let report = report(targetMs: targetMs)
        return "{\"path\": \"\(path)\", \"width\": \(width), \"height\": \(height), "
            + "\"seconds\": \(number(seconds)), \"frames\": \(report.frames), "
            + "\"mean_ms\": \(number(report.meanMs)), \"p50_ms\": \(number(report.p50Ms)), "
            + "\"p99_ms\": \(number(report.p99Ms)), \"p999_ms\": \(number(report.p999Ms)), "
            + "\"hitches\": \(report.hitches), \"pacing_ms\": \(number(report.pacingMs)), "
            + "\"gpu_p50_ms\": \(number(gpuP50)), \"gpu_p99_ms\": \(number(gpuP99))"
            + (cpuTimes.isEmpty ? "" : ", \"cpu_p50_ms\": \(number(cpuP50)), \"cpu_p99_ms\": \(number(cpuP99))") + "}"
    }
}
