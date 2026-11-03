// game.wasm run headless for the app (needs assets/maps and a built
// game.wasm): Blood Gulch loaded and drawn for some seconds into a window
// off the screen, through WasmGame and GameRenderer.

import Foundation
import Metal
import OpenCEEngineCore
import OpenCEEngineRender
import XCTest

/// The repository's root, five folders above this file.
let repositoryRoot = URL(fileURLWithPath: #filePath).deletingLastPathComponent()
    .deletingLastPathComponent().deletingLastPathComponent().deletingLastPathComponent()
    .deletingLastPathComponent()

final class GameRunTests: XCTestCase {
    func testBloodGulchIsDrawnHeadless() throws {
        let root = repositoryRoot
        let wasm = root.appendingPathComponent("build/engine/wasm/game.wasm")
        let map = root.appendingPathComponent("assets/maps/bloodgulch.map")
        let files = FileManager.default
        guard files.fileExists(atPath: wasm.path) else { throw XCTSkip("no build/engine/wasm/game.wasm") }
        guard files.fileExists(atPath: map.path) else { throw XCTSkip("no assets/maps/bloodgulch.map") }
        let saves = files.temporaryDirectory.appendingPathComponent("opence-game-run-\(UUID().uuidString)")
        try files.createDirectory(at: saves, withIntermediateDirectories: true)
        defer { try? files.removeItem(at: saves) }

        let device = try XCTUnwrap(MTLCreateSystemDefaultDevice())
        let window = try XCTUnwrap(OffscreenWindow(device: device, width: 640, height: 480))
        let renderer = try XCTUnwrap(GameRenderer(device: device, window: window))
        let game = try WasmGame(
            options: WasmGameOptions(
                wasm: wasm.path, data: root.appendingPathComponent("assets").path, saves: saves.path,
                cache: root.appendingPathComponent("build/engine/wasm/cache").path, width: 640, height: 480,
                map: "bloodgulch", seconds: 30, settings: [], audit: nil))
        try game.start(target: renderer)
        let end = game.wait()
        renderer.finish()

        print(
            "game run: \(end.batches) batches, \(end.commands) commands, \(end.refused) refused by the game's "
                + "check, \(renderer.statistics.refused) by Metal, \(window.presented) frames presented")
        XCTAssertNil(end.error)
        XCTAssertTrue(end.timedOut)
        XCTAssertEqual(end.refused, 0, end.refusals.joined(separator: "\n"))
        XCTAssertGreaterThan(window.presented, 0)
        // (the level drawn: more than one colour on the screen)
        let pixels = window.pixels()
        XCTAssertGreaterThan(Set(pixels.map { Array($0.prefix(3)) }).count, 16)
        // (to look at it: the last frame as a PPM where OPENCE_GAME_RUN_FRAME names, kept on this machine)
        if let path = ProcessInfo.processInfo.environment["OPENCE_GAME_RUN_FRAME"] {
            let header = Data("P6\n\(window.texture.width) \(window.texture.height)\n255\n".utf8)
            try (header + Data(pixels.flatMap { $0.prefix(3) })).write(to: URL(fileURLWithPath: path))
        }
    }

    /// Blood Gulch heard headless: its mixed sound pulled from game.wasm's speakers as the audio device
    /// would pull it, a 48th of a second at a time, and not silent once the level plays.
    func testBloodGulchIsHeardHeadless() throws {
        let root = repositoryRoot
        let wasm = root.appendingPathComponent("build/engine/wasm/game.wasm")
        let files = FileManager.default
        guard files.fileExists(atPath: wasm.path) else { throw XCTSkip("no build/engine/wasm/game.wasm") }
        guard files.fileExists(atPath: root.appendingPathComponent("assets/maps/bloodgulch.map").path) else {
            throw XCTSkip("no assets/maps/bloodgulch.map")
        }
        let saves = files.temporaryDirectory.appendingPathComponent("opence-game-sound-\(UUID().uuidString)")
        try files.createDirectory(at: saves, withIntermediateDirectories: true)
        defer { try? files.removeItem(at: saves) }
        let device = try XCTUnwrap(MTLCreateSystemDefaultDevice())
        let window = try XCTUnwrap(OffscreenWindow(device: device, width: 320, height: 240))
        let renderer = try XCTUnwrap(GameRenderer(device: device, window: window))
        let game = try WasmGame(
            options: WasmGameOptions(
                wasm: wasm.path, data: root.appendingPathComponent("assets").path, saves: saves.path,
                cache: root.appendingPathComponent("build/engine/wasm/cache").path, width: 320, height: 240,
                map: "bloodgulch", seconds: 25, settings: [], audit: nil))
        let speakers = game.speakers()
        XCTAssertEqual(speakers.sampleRate(), 48_000)
        try game.start(target: renderer)
        var heard = 0
        var loudest: Float = 0
        let started = Date()
        while Date().timeIntervalSince(started) < 22 {
            let samples = speakers.pull(frames: 1000)
            XCTAssertEqual(samples.count, 2000)
            if game.ticks() > 0 {
                heard += samples.filter { $0 != 0 }.count
                loudest = max(loudest, samples.map(abs).max() ?? 0)
            }
            Thread.sleep(forTimeInterval: 1.0 / 48)
        }
        let end = game.wait()
        renderer.finish()
        print("game sound: \(heard) samples heard in play, loudest \(loudest)")
        XCTAssertNil(end.error)
        XCTAssertGreaterThan(game.ticks(), 0, "the game never played")
        XCTAssertGreaterThan(heard, 48_000, "under half a second of sound in play")
        XCTAssertLessThanOrEqual(loudest, 1.5)
    }

    /// The pipeline archive across two runs of Blood Gulch: the first compiles each pipeline into it, the
    /// second, in play, finds every one there and never waits on Metal's compiler.
    func testASecondRunOfBloodGulchCompilesNoPipelineInPlay() throws {
        let root = repositoryRoot
        let wasm = root.appendingPathComponent("build/engine/wasm/game.wasm")
        let files = FileManager.default
        guard files.fileExists(atPath: wasm.path) else { throw XCTSkip("no build/engine/wasm/game.wasm") }
        guard files.fileExists(atPath: root.appendingPathComponent("assets/maps/bloodgulch.map").path) else {
            throw XCTSkip("no assets/maps/bloodgulch.map")
        }
        let folder = files.temporaryDirectory.appendingPathComponent("opence-archive-run-\(UUID().uuidString)")
        try files.createDirectory(at: folder, withIntermediateDirectories: true)
        defer { try? files.removeItem(at: folder) }
        let url = folder.appendingPathComponent("pipelines.metallib")
        let device = try XCTUnwrap(MTLCreateSystemDefaultDevice())
        var runs: [GameRenderer.Statistics] = []
        for _ in 0..<2 {
            let window = try XCTUnwrap(OffscreenWindow(device: device, width: 640, height: 480))
            let renderer = try XCTUnwrap(GameRenderer(device: device, window: window))
            let game = try WasmGame(
                options: WasmGameOptions(
                    wasm: wasm.path, data: root.appendingPathComponent("assets").path, saves: folder.path,
                    cache: root.appendingPathComponent("build/engine/wasm/cache").path, width: 640, height: 480,
                    map: "bloodgulch", seconds: 25, settings: [], audit: nil))
            renderer.archive = try XCTUnwrap(PipelineArchive(device: device, url: url))
            renderer.playing = { game.ticks() > 0 }
            try game.start(target: renderer)
            let end = game.wait()
            renderer.finish()
            XCTAssertNil(end.error)
            XCTAssertGreaterThan(game.ticks(), 0, "the game never played")
            runs.append(renderer.statistics)
        }
        print(
            "pipeline archive: first run \(runs[0].pipelinesMade) compiled (\(runs[0].pipelinesMissedInPlay) in play), "
                + "second \(runs[1].pipelinesArchived) archived, \(runs[1].pipelinesMissedInPlay) compiled in play")
        XCTAssertGreaterThan(runs[0].pipelinesMade, 0)
        XCTAssertGreaterThan(runs[1].pipelinesArchived, 0)
        XCTAssertEqual(runs[1].pipelinesMissedInPlay, 0)
    }

    /// Blood Gulch's golden frame through game.wasm: frame 1500 at 640×480 from the VM's camera (camera.txt,
    /// loaded each second through the console as the VM's network_test_camera did), the game's time stepped a
    /// 60th of a second a frame (debug.frame_clock). Written to build/engine/golden, never approved here, and
    /// measured against the VM's reference.
    func testBloodGulchGoldenFrameIsDrawnThroughWasm() throws {
        let root = repositoryRoot
        let wasm = root.appendingPathComponent("build/engine/wasm/game.wasm")
        let maps = root.appendingPathComponent("assets/maps")
        let files = FileManager.default
        guard files.fileExists(atPath: wasm.path) else { throw XCTSkip("no build/engine/wasm/game.wasm") }
        guard files.fileExists(atPath: maps.appendingPathComponent("bloodgulch.map").path) else {
            throw XCTSkip("no assets/maps/bloodgulch.map")
        }
        let folder = files.temporaryDirectory.appendingPathComponent("opence-golden-\(UUID().uuidString)")
        let data = folder.appendingPathComponent("data")
        let saves = folder.appendingPathComponent("saves")
        try files.createDirectory(at: data, withIntermediateDirectories: true)
        try files.createDirectory(at: saves, withIntermediateDirectories: true)
        defer { try? files.removeItem(at: folder) }
        // (the maps linked, not copied; the camera the VM's frame was taken from)
        try files.createSymbolicLink(at: data.appendingPathComponent("maps"), withDestinationURL: maps)
        try Data("95.0 -150.0 4.0\n-0.6 0.8 -0.15\n0 0 1\n1.22\n".utf8)
            .write(to: data.appendingPathComponent("camera.txt"))

        let frame = 1500
        let device = try XCTUnwrap(MTLCreateSystemDefaultDevice())
        let window = try XCTUnwrap(OffscreenWindow(device: device, width: 640, height: 480, capture: frame))
        let renderer = try XCTUnwrap(GameRenderer(device: device, window: window))
        let game = try WasmGame(
            options: WasmGameOptions(
                wasm: wasm.path, data: data.path, saves: saves.path,
                cache: root.appendingPathComponent("build/engine/wasm/cache").path, width: 640, height: 480,
                map: "bloodgulch", seconds: 240,
                settings: ["debug.frame_clock=true", "debug.telnet_console=true"], audit: nil))
        try game.start(target: renderer)
        let camera = Thread {
            while !window.hasCaptured {
                _ = game.ask(line: "camera_control true", milliseconds: 200)
                if let answer = game.ask(line: "debug_camera_load", milliseconds: 200), !answer.isEmpty {
                    print("debug_camera_load: \(answer)")
                }
                Thread.sleep(forTimeInterval: 1)
            }
        }
        camera.start()
        let end = game.wait()
        renderer.finish()

        XCTAssertNil(end.error)
        XCTAssertEqual(end.refused, 0, end.refusals.joined(separator: "\n"))
        let drawn = try XCTUnwrap(window.capturedFrame(), "only \(window.presented) frames were presented")
        let golden = root.appendingPathComponent("build/engine/golden")
        try files.createDirectory(at: golden, withIntermediateDirectories: true)
        try drawn.writePNG(to: golden.appendingPathComponent("wasm_bloodgulch_640x480.png"))
        // (the level drawn, the sky and grass where the VM's frame has them)
        XCTAssertGreaterThan(drawn.distinctColors, 64)
        let sky = drawn.rgb(x: 20, y: 10)
        let grass = drawn.rgb(x: 150, y: 320)
        print("golden frame: sky \(sky), grass \(grass)")
        let reference = root.appendingPathComponent("build/engine/golden/vm_bloodgulch_640x480.png")
        if let vm = CapturedFrame.readPNG(at: reference), let (largest, share) = drawn.difference(from: vm) {
            print("golden frame against the VM's: largest \(largest), \(share * 100)% differ by more than 8")
            // By region: the level (the ground below the horizon, the cliff at the right) is the VM's; the sky's
            // clouds scroll with the time, and the VM's frame shows a hint ("Hold BACK for score") this one has
            // not, so those two are measured and printed, not held.
            let regions: [(String, Range<Int>, Range<Int>, Double?)] = [
                ("ground", 0..<640, 240..<480, 0.01), ("cliff", 370..<640, 0..<160, 0.01),
                ("sky", 0..<640, 0..<90, nil), ("hint", 0..<260, 90..<125, nil),
            ]
            for (name, columns, rows, limit) in regions {
                let region = try XCTUnwrap(drawn.difference(from: vm, columns: columns, rows: rows))
                print("golden frame's \(name): largest \(region.largest), \(region.share * 100)% differ by more than 8")
                if let limit { XCTAssertLessThanOrEqual(region.share, limit, name) }
            }
        }
    }
}
