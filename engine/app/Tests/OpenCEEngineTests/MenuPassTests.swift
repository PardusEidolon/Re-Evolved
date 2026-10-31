import Metal
import OpenCEEngineCore
@testable import OpenCEEngineRender
import XCTest

/// The Halo UI plugin's scene drawn by the app: the design's screen fitted
/// to the target, its boxes' edges and corners, and its lines laid out in
/// the faces' own advances.
final class MenuPassTests: XCTestCase {
    func device() throws -> MTLDevice {
        try XCTUnwrap(MTLCreateSystemDefaultDevice(), "no Metal device")
    }

    func testTheDesignIsFittedAndCentred() {
        let full = MenuDesign.fit(width: 3840, height: 2160)
        XCTAssertEqual(full.scale, 1)
        XCTAssertEqual(full.x, 0)
        XCTAssertEqual(full.y, 0)
        XCTAssertEqual(MenuDesign.fit(width: 1920, height: 1080).scale, 0.5)
        // (5:4: the design across the width, centred down it)
        let square = MenuDesign.fit(width: 1280, height: 1024)
        XCTAssertEqual(square.scale, 1280.0 / 3840.0, accuracy: 1e-6)
        XCTAssertEqual(square.x, 0)
        XCTAssertEqual(square.y, (1024 - 720) / 2, accuracy: 1e-3)
        // (21:9: down the height, centred across)
        let wide = MenuDesign.fit(width: 2560, height: 1080)
        XCTAssertEqual(wide.scale, 0.5)
        XCTAssertEqual(wide.x, (2560 - 1920) / 2)
    }

    func testTheFacesAreFound() throws {
        for face in [MenuFace.title, .heavy, .bold, .semibold] {
            XCTAssertNotNil(MenuFonts.url(face), MenuFonts.file(face))
        }
        let glyphs = try MenuGlyphs(url: try XCTUnwrap(MenuFonts.url(.heavy)), pixelSize: 54, device: try device())
        XCTAssertEqual(glyphs.texture.pixelFormat, .rg8Unorm)
        XCTAssertTrue(glyphs.has("A"))
        XCTAssertFalse(glyphs.has("漢"))
        // (proportional: an I narrower than an M, and twice the size twice the width)
        XCTAssertLessThan(glyphs.width("I"), glyphs.width("M"))
        let twice = try MenuGlyphs(url: try XCTUnwrap(MenuFonts.url(.heavy)), pixelSize: 108, device: try device())
        XCTAssertEqual(twice.width("CAMPAIGN") / glyphs.width("CAMPAIGN"), 2, accuracy: 0.05)
        XCTAssertThrowsError(
            try MenuGlyphs(url: try XCTUnwrap(MenuFonts.url(.heavy)), pixelSize: 0, device: try device()))
    }

    func testTheSceneIsLaidOutInTheTargetsPixels() throws {
        let pass = try MenuPass(device: try device(), outputFormat: .bgra8Unorm)
        let scene = MenuScene(
            shapes: [
                MenuShape(
                    rect: [240, 330, 750, 1260], fill: [0, 0, 0, 0.5], border: 9, borderColor: [1, 1, 1, 1],
                    radii: [54, 0, 0, 54])
            ],
            texts: [
                MenuText(
                    text: "CAMPAIGN", x: 1920, y: 1185, size: 108, face: .heavy, color: [1, 1, 1, 1],
                    glow: [0.4, 0.75, 1, 0.8], align: .center),
                MenuText(
                    text: "OpenCE", x: 3744, y: 2052, size: 36, face: .semibold, color: [1, 1, 1, 1],
                    glow: [0, 0, 0, 0], align: .right),
            ])
        let shapes = pass.shapeInstances(scene, width: 1920, height: 1080)
        XCTAssertEqual(shapes.count, 1)
        XCTAssertEqual(shapes[0].rect, SIMD4(120, 165, 495, 795))
        XCTAssertEqual(shapes[0].radii, SIMD4(27, 0, 0, 27))
        XCTAssertEqual(shapes[0].border.x, 4.5)

        let runs = try pass.glyphInstances(scene, width: 1920, height: 1080)
        // (CAMPAIGN's glow under its ink, then OpenCE's ink)
        XCTAssertEqual(runs.map { $0.1.count }, [8, 8, 6])
        XCTAssertEqual(runs[0].1.map(\.channel.x), Array(repeating: 1, count: 8))
        XCTAssertEqual(runs[1].1.map(\.channel.x), Array(repeating: 0, count: 8))
        XCTAssertEqual(runs[1].0.pixelSize, 54)
        XCTAssertEqual(runs[2].0.pixelSize, 18)
        // centred: as far from the middle on the left as on the right
        let ink = runs[1].1
        let left = try XCTUnwrap(ink.map(\.rect.x).min()), right = try XCTUnwrap(ink.map(\.rect.z).max())
        XCTAssertEqual((left + right) / 2, 960, accuracy: 6)
        // right-aligned: its last glyph ends at its x, give or take its padding
        let small = runs[2].1
        let end = try XCTUnwrap(small.map(\.rect.z).max())
        XCTAssertEqual(end, 1872, accuracy: 8)
        // the lines' glyphs within their lines
        for quad in ink {
            XCTAssertGreaterThan(quad.rect.w, 1185 / 2 - 10)
            XCTAssertLessThan(quad.rect.y, (1185 + 108) / 2 + 10)
        }
        // nothing for an empty or zero-sized scene
        XCTAssertTrue(pass.shapeInstances(MenuScene(shapes: [], texts: []), width: 640, height: 480).isEmpty)
        let flat = MenuScene(
            shapes: [MenuShape(rect: [0, 0, 0, 10], fill: [1, 1, 1, 1], border: 0, borderColor: [], radii: [])],
            texts: [])
        XCTAssertTrue(pass.shapeInstances(flat, width: 640, height: 480).isEmpty)
    }

    func testThePassDrawsBoxesAndText() throws {
        let device = try device()
        let queue = try XCTUnwrap(device.makeCommandQueue())
        let pass = try MenuPass(device: device, outputFormat: .bgra8Unorm)
        let (width, height) = (384, 216)
        let descriptor = MTLTextureDescriptor.texture2DDescriptor(
            pixelFormat: .bgra8Unorm, width: width, height: height, mipmapped: false)
        descriptor.usage = [.renderTarget, .shaderRead]
        descriptor.storageMode = .shared
        let target = try XCTUnwrap(device.makeTexture(descriptor: descriptor))

        func draw(_ scene: MenuScene) throws -> [UInt8] {
            let commands = try XCTUnwrap(queue.makeCommandBuffer())
            let clear = MTLRenderPassDescriptor()
            clear.colorAttachments[0].texture = target
            clear.colorAttachments[0].loadAction = .clear
            clear.colorAttachments[0].clearColor = MTLClearColor(red: 0, green: 0, blue: 0, alpha: 1)
            clear.colorAttachments[0].storeAction = .store
            commands.makeRenderCommandEncoder(descriptor: clear)?.endEncoding()
            try pass.encode(scene, into: commands, target: target)
            commands.commit()
            commands.waitUntilCompleted()
            var bytes = [UInt8](repeating: 0, count: width * height * 4)
            target.getBytes(
                &bytes, bytesPerRow: width * 4, from: MTLRegionMake2D(0, 0, width, height), mipmapLevel: 0)
            return bytes
        }
        // (bgra)
        func pixel(_ bytes: [UInt8], _ x: Int, _ y: Int) -> SIMD4<UInt8> {
            let at = (y * width + x) * 4
            return SIMD4(bytes[at + 2], bytes[at + 1], bytes[at], bytes[at + 3])
        }
        func lit(_ bytes: [UInt8], x: Range<Int>, y: Range<Int>) -> Int {
            var count = 0
            for row in y {
                for column in x where bytes[(row * width + column) * 4 + 1] > 64 { count += 1 }
            }
            return count
        }

        _ = try draw(MenuScene(shapes: [], texts: []))
        XCTAssertEqual(pass.lastDrawn.shapes, 0)
        XCTAssertEqual(pass.lastDrawn.glyphs, 0)

        // at a tenth of the design: a box 40×20 at 100, 50, its border 1
        // pixel of green, its fill red, its rounded corner left unfilled
        let boxed = try draw(
            MenuScene(
                shapes: [
                    MenuShape(
                        rect: [1000, 500, 400, 200], fill: [1, 0, 0, 1], border: 10, borderColor: [0, 1, 0, 1],
                        radii: [80, 0, 0, 0])
                ], texts: []))
        XCTAssertEqual(pass.lastDrawn.shapes, 1)
        XCTAssertEqual(pixel(boxed, 120, 60), SIMD4(255, 0, 0, 255), "the fill")
        XCTAssertEqual(pixel(boxed, 139, 60).y, 255, "the border")
        XCTAssertEqual(pixel(boxed, 99, 60), SIMD4(0, 0, 0, 255), "outside")
        XCTAssertEqual(pixel(boxed, 100, 50), SIMD4(0, 0, 0, 255), "the rounded corner")
        XCTAssertEqual(pixel(boxed, 139, 69).y, 255, "the square corner")

        // a line: ink where it is, a glow around it, nothing far off
        let line = MenuText(
            text: "MULTIPLAYER", x: 1920, y: 1000, size: 160, face: .heavy, color: [1, 1, 1, 1],
            glow: [0, 1, 0, 1], align: .center)
        var plain = line
        plain.glow = [0, 0, 0, 0]
        let unglowed = try draw(MenuScene(shapes: [], texts: [plain]))
        XCTAssertEqual(pass.lastDrawn.glyphs, 11)
        XCTAssertGreaterThan(lit(unglowed, x: 100..<284, y: 100..<116), 100)
        XCTAssertEqual(lit(unglowed, x: 0..<width, y: 0..<90), 0)
        let glowed = try draw(MenuScene(shapes: [], texts: [line]))
        XCTAssertEqual(pass.lastDrawn.glyphs, 22)
        XCTAssertGreaterThan(lit(glowed, x: 0..<width, y: 0..<height), lit(unglowed, x: 0..<width, y: 0..<height))
    }
}
