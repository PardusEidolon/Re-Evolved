// The game's render batches on Metal (GameRenderer): each frame's commands
// carried out in order, the bytes its uploads name taken from the frame one
// after another, and the textures it reads back answered.

import Foundation
import Metal
import OpenCEEngineCore
import OpenCEEngineRender
import XCTest

final class GameRendererTests: XCTestCase {
    var device: MTLDevice!
    var window: OffscreenWindow!
    var renderer: GameRenderer!

    override func setUpWithError() throws {
        device = try XCTUnwrap(MTLCreateSystemDefaultDevice())
        window = try XCTUnwrap(OffscreenWindow(device: device, width: 4, height: 4))
        renderer = try XCTUnwrap(GameRenderer(device: device, window: window))
    }

    /// a 2D RGBA8 texture drawn to and read back
    func target(_ id: UInt32, width: UInt32 = 2, height: UInt32 = 2) -> Command {
        .createTexture(
            id: id, kind: .plane, format: .rgba8, width: width, height: height, depth: 1, levels: 1,
            sampled: true, renderTarget: true, samples: 1)
    }

    func pass(_ color: UInt32, clear: (Float, Float, Float, Float) = (0, 0, 0, 0)) -> Command {
        .beginPass(
            color: color, depth: 0, colorLoad: .clear, depthLoad: .dontCare, stencilLoad: .dontCare,
            clearRed: clear.0, clearGreen: clear.1, clearBlue: clear.2, clearAlpha: clear.3, clearDepth: 1,
            clearStencil: 0, resolve: 0, visibility: 0, width: 0, height: 0)
    }

    func read(_ id: UInt32, to address: UInt32, width: UInt32 = 2, height: UInt32 = 2) -> Command {
        .readTexture(
            id: id, level: 0, slice: 0, x: 0, y: 0, width: width, height: height, address: address,
            bytesPerRow: width * 4, length: width * height * 4)
    }

    func testAClearedTextureIsReadBackWhereItsCommandNames() {
        let readbacks = renderer.submit(
            frame: RenderFrame(
                commands: [
                    target(1), pass(1, clear: (1, 0, 0, 1)), .endPass, read(1, to: 0x800),
                ], data: Data()))
        XCTAssertEqual(
            readbacks,
            [
                RenderReadback(
                    address: 0x800,
                    bytes: Data(
                        [UInt8](repeating: 0, count: 4).enumerated().flatMap {
                            _ in [255, 0, 0, 255] as [UInt8]
                        }))
            ])
        XCTAssertEqual(renderer.statistics.refused, 0)
    }

    func testEachUploadTakesItsLengthOfTheFrameSBytesInTurn() {
        let texels: [UInt8] = Array(1...16)
        let readbacks = renderer.submit(
            frame: RenderFrame(
                commands: [
                    .createBuffer(id: 1, length: 4),
                    // (the buffer's four bytes come first, then the texture's)
                    .uploadBuffer(id: 1, offset: 0, address: 0x100, length: 4),
                    target(2),
                    .uploadTexture(
                        id: 2, level: 0, slice: 0, x: 0, y: 0, z: 0, width: 2, height: 2, depth: 1,
                        address: 0x200, bytesPerRow: 8, bytesPerImage: 16, length: 16),
                    read(2, to: 0x900),
                ], data: Data([9, 9, 9, 9] + texels)))
        XCTAssertEqual(readbacks.map(\.address), [0x900])
        XCTAssertEqual(readbacks.first.map { [UInt8]($0.bytes) }, texels)
    }

    /// a triangle over the whole target, in one colour
    static let shader = """
        #include <metal_stdlib>
        using namespace metal;
        struct Out { float4 position [[position]]; };
        vertex Out vertex_main(uint id [[vertex_id]]) {
            float2 corner = float2((id << 1) & 2, id & 2);
            return Out { float4(corner * 2 - 1, 0, 1) };
        }
        fragment float4 fragment_main(Out in [[stage_in]]) { return float4(0, 1, 0, 1); }
        """

    func testAPipelineMadeFromTheFrameSLibraryDraws() {
        let source = Data(Self.shader.utf8)
        let blend = Blend(
            enabled: false, sourceColor: .one, destinationColor: .zero, colorOperation: .add,
            sourceAlpha: .one, destinationAlpha: .zero, alphaOperation: .add)
        let readbacks = renderer.submit(
            frame: RenderFrame(
                commands: [
                    .createLibrary(id: 1, address: 0x100, length: UInt32(source.count)),
                    .createPipeline(
                        id: 2, vertexLibrary: 1, fragmentLibrary: 1, colorFormat: .rgba8, depthFormat: nil,
                        samples: 1, blend: blend, writeMask: 15, attributes: [], layouts: []),
                    target(3), pass(3), .setPipeline(id: 2),
                    .draw(primitive: .triangle, first: 0, vertices: 3, instances: 1),
                    .endPass, read(3, to: 0x800),
                ], data: source))
        XCTAssertEqual(
            readbacks.first.map { [UInt8]($0.bytes) },
            Array([[UInt8]](repeating: [0, 255, 0, 255], count: 4).joined()))
        XCTAssertEqual(renderer.statistics.librariesCompiled, 1)
        XCTAssertEqual(renderer.statistics.pipelinesMade, 1)
    }

    func testACountingVisibilityDrawIsReadBackFromItsBuffer() {
        let source = Data(Self.shader.utf8)
        let blend = Blend(
            enabled: false, sourceColor: .one, destinationColor: .zero, colorOperation: .add,
            sourceAlpha: .one, destinationAlpha: .zero, alphaOperation: .add)
        let readbacks = renderer.submit(
            frame: RenderFrame(
                commands: [
                    .createLibrary(id: 1, address: 0x100, length: UInt32(source.count)),
                    .createPipeline(
                        id: 2, vertexLibrary: 1, fragmentLibrary: 1, colorFormat: .rgba8, depthFormat: nil,
                        samples: 1, blend: blend, writeMask: 15, attributes: [], layouts: []),
                    target(3), .createBuffer(id: 4, length: 16),
                    .beginPass(
                        color: 3, depth: 0, colorLoad: .clear, depthLoad: .dontCare, stencilLoad: .dontCare,
                        clearRed: 0, clearGreen: 0, clearBlue: 0, clearAlpha: 0, clearDepth: 1,
                        clearStencil: 0, resolve: 0, visibility: 4, width: 0, height: 0),
                    .setPipeline(id: 2), .setVisibility(mode: .counting, offset: 8),
                    .draw(primitive: .triangle, first: 0, vertices: 3, instances: 1),
                    .setVisibility(mode: .disabled, offset: 0), .endPass,
                    .readBuffer(id: 4, offset: 8, address: 0x800, length: 8),
                ], data: source))
        // (the triangle covers the 2x2 target: four samples passed)
        XCTAssertEqual(readbacks.first?.address, 0x800)
        XCTAssertEqual(
            readbacks.first.map { $0.bytes.withUnsafeBytes { $0.loadUnaligned(as: UInt64.self) } }, 4)
    }

    /// the frame that makes the one-colour pipeline, its blending on or off, and draws with it
    func drawFrame(blending: Bool) -> RenderFrame {
        let source = Data(Self.shader.utf8)
        let blend = Blend(
            enabled: blending, sourceColor: .one, destinationColor: .zero, colorOperation: .add,
            sourceAlpha: .one, destinationAlpha: .zero, alphaOperation: .add)
        return RenderFrame(
            commands: [
                .createLibrary(id: 1, address: 0x100, length: UInt32(source.count)),
                .createPipeline(
                    id: 2, vertexLibrary: 1, fragmentLibrary: 1, colorFormat: .rgba8, depthFormat: nil,
                    samples: 1, blend: blend, writeMask: 15, attributes: [], layouts: []),
                target(3), pass(3), .setPipeline(id: 2),
                .draw(primitive: .triangle, first: 0, vertices: 3, instances: 1), .endPass,
            ], data: source)
    }

    func testAPipelineCompiledOnceIsFoundInTheArchiveOnTheNextRun() throws {
        let folder = FileManager.default.temporaryDirectory.appendingPathComponent(
            "opence-archive-\(UUID().uuidString)")
        defer { try? FileManager.default.removeItem(at: folder) }
        let url = folder.appendingPathComponent("pipelines.metallib")
        // the first run compiles the pipeline into the archive, and writes it
        renderer.archive = try XCTUnwrap(PipelineArchive(device: device, url: url))
        _ = renderer.submit(frame: drawFrame(blending: false))
        renderer.finish()
        XCTAssertEqual(renderer.statistics.pipelinesArchived, 0)
        XCTAssertTrue(FileManager.default.fileExists(atPath: url.path))
        // the next, in play from the start, finds it there; a pipeline it never made is a miss
        let next = try XCTUnwrap(GameRenderer(device: device, window: window))
        next.archive = try XCTUnwrap(PipelineArchive(device: device, url: url))
        next.playing = { true }
        _ = next.submit(frame: drawFrame(blending: false))
        XCTAssertEqual(next.statistics.pipelinesArchived, 1)
        XCTAssertEqual(next.statistics.pipelinesMissedInPlay, 0)
        _ = next.submit(frame: drawFrame(blending: true))
        XCTAssertEqual(next.statistics.pipelinesMissedInPlay, 1)
        XCTAssertEqual(next.statistics.refused, 0)
    }

    func testALibraryThatDoesNotCompileIsRefusedAndWhatFollowsStillRuns() {
        let broken = Data("not metal".utf8)
        let readbacks = renderer.submit(
            frame: RenderFrame(
                commands: [
                    .createLibrary(id: 1, address: 0x100, length: UInt32(broken.count)),
                    target(2), pass(2, clear: (0, 0, 1, 1)), .endPass, read(2, to: 0x800, width: 1, height: 1),
                ], data: broken))
        XCTAssertEqual(renderer.statistics.refused, 1)
        XCTAssertEqual(readbacks.first.map { [UInt8]($0.bytes) }, [0, 0, 255, 255])
    }

    func testAPassOnColourZeroDrawsTheWindowAndPresentShowsIt() {
        _ = renderer.submit(
            frame: RenderFrame(
                commands: [pass(0, clear: (0, 0, 1, 1)), .endPass, .present],
                data: Data()))
        renderer.finish()
        XCTAssertEqual(window.presented, 1)
        XCTAssertEqual(window.pixels().first, [0, 0, 255, 255])
    }

    func testASnapshotTakesTheFramesItIsAskedForAsTheyWereShown() {
        // (the second and fourth frames asked for; each a colour of its own)
        let lock = NSLock()
        var taken: [(Int, [UInt8])] = []
        window.snapshot = { presented in
            guard presented % 2 == 0 else { return nil }
            return { frame in
                lock.lock()
                taken.append((presented, Array(frame.bytes.prefix(4))))
                lock.unlock()
            }
        }
        let colours: [(Float, Float, Float, Float)] = [(1, 0, 0, 1), (0, 1, 0, 1), (0, 0, 1, 1), (1, 1, 1, 1)]
        for colour in colours {
            _ = renderer.submit(
                frame: RenderFrame(commands: [pass(0, clear: colour), .endPass, .present], data: Data()))
        }
        renderer.finish()
        lock.lock()
        defer { lock.unlock() }
        // (BGRA, as the window holds them)
        XCTAssertEqual(taken.map(\.0), [2, 4])
        XCTAssertEqual(taken.map(\.1), [[0, 255, 0, 255], [255, 255, 255, 255]])
    }

    func testTheGameSObjectsLastFromFrameToFrame() {
        _ = renderer.submit(frame: RenderFrame(commands: [target(1)], data: Data()))
        let readbacks = renderer.submit(
            frame: RenderFrame(
                commands: [
                    pass(1, clear: (1, 1, 1, 1)), .endPass, read(1, to: 0x800, width: 1, height: 1),
                ], data: Data()))
        XCTAssertEqual(readbacks.first.map { [UInt8]($0.bytes) }, [255, 255, 255, 255])
        XCTAssertEqual(renderer.statistics.batches, 2)
    }
}
