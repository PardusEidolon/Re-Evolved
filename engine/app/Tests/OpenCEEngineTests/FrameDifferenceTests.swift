// A frame's difference from another, over the whole frame or a region of it
// (a golden frame is measured by region: the level's, its sky's, its HUD's).

@testable import OpenCEEngineRender
import XCTest

final class FrameDifferenceTests: XCTestCase {
    /// A 4×2 frame of one grey, with the texel at (x, y) set to `value`.
    private func frame(changing x: Int = -1, _ y: Int = -1, to value: UInt8 = 0) -> CapturedFrame {
        var bytes = [UInt8](repeating: 100, count: 4 * 2 * 4)
        if x >= 0 {
            let at = (y * 4 + x) * 4
            bytes[at] = value
            bytes[at + 1] = value
            bytes[at + 2] = value
        }
        return CapturedFrame(width: 4, height: 2, bytes: bytes)
    }

    func testARegionCountsOnlyItsOwnTexels() throws {
        let changed = frame(changing: 3, 1, to: 160)
        let whole = try XCTUnwrap(changed.difference(from: frame()))
        XCTAssertEqual(whole.largest, 60)
        XCTAssertEqual(whole.share, 1.0 / 8.0)
        // (the left half has no change; the right column's two texels, one)
        let left = try XCTUnwrap(changed.difference(from: frame(), columns: 0..<2, rows: 0..<2))
        XCTAssertEqual(left.largest, 0)
        XCTAssertEqual(left.share, 0)
        let right = try XCTUnwrap(changed.difference(from: frame(), columns: 3..<4, rows: 0..<2))
        XCTAssertEqual(right.largest, 60)
        XCTAssertEqual(right.share, 0.5)
    }

    func testARegionOutsideTheFrameHasNoDifference() {
        XCTAssertNil(frame().difference(from: frame(), columns: 2..<5, rows: 0..<2))
    }
}
