// The game's benchmark (game.wasm through GameRenderer): each frame is the
// slower of the game's CPU frame and its GPU work, the two lists aligned at
// their ends (the GPU's has the load's frames before them).

import OpenCEEngineRender
import XCTest

final class GameBenchmarkTests: XCTestCase {
    func testAFrameIsTheSlowerOfItsCPUAndGPUTimes() {
        let result = BenchmarkResult.game(
            width: 3840, height: 2160, seconds: 60, cpuTimes: [1, 9, 2], gpuTimes: [30, 4, 5, 6])
        XCTAssertEqual(result.frameTimes, [4, 9, 6])
        XCTAssertEqual(result.cpuTimes, [1, 9, 2])
        // (the load's frame left out of the GPU's too)
        XCTAssertEqual(result.gpuTimes, [4, 5, 6])
        XCTAssertEqual(result.p99, 9)
    }

    func testNoFramesMakeNoNumbers() {
        let result = BenchmarkResult.game(width: 640, height: 480, seconds: 1, cpuTimes: [], gpuTimes: [3])
        XCTAssertTrue(result.frameTimes.isEmpty)
        XCTAssertEqual(result.p99, 0)
    }
}
