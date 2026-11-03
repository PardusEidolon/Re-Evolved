// The developer shell over the game: ~ (the key left of 1) opens it, as
// does the control socket's `shell` request; the game keeps playing under
// it. Its lines go to the game's own console. The main thread runs it and
// lays out what it draws; the renderer draws that over each frame on the
// game's thread, so the two meet at a locked copy of the items.
import AppKit
import Metal
import OpenCEEngineCore
import OpenCEEngineKit
import OpenCEEngineRender
import OpenCEEngineWindow

/// The shell's items for the renderer's thread.
final class ShellFrame: @unchecked Sendable {
    private let lock = NSLock()
    private var items: [TextItem] = []

    func set(_ items: [TextItem]) { lock.withLock { self.items = items } }
    func take() -> [TextItem] { lock.withLock { items } }
}

@MainActor
final class GameShell {
    let overlay: ShellOverlay
    private let textPass: TextPass
    private let frame = ShellFrame()

    /// The shell for `game`, drawn by `renderer`; nil (and told) when its
    /// font or its pass cannot be had.
    init?(game: WasmGame, renderer: GameRenderer, device: MTLDevice, scale: Double) {
        guard let font = GlyphAtlas.url() else {
            FileHandle.standardError.write(Data("OpenCE Engine: the shell has no font\n".utf8))
            return nil
        }
        do {
            let atlas = try GlyphAtlas(url: font, scale: scale, device: device)
            textPass = try TextPass(device: device, atlas: atlas, outputFormat: .bgra8Unorm)
        } catch {
            FileHandle.standardError.write(Data("OpenCE Engine: the shell's text: \(error)\n".utf8))
            return nil
        }
        overlay = ShellOverlay(
            engine: GameShellEngine(game: game), pasteboard: SystemPasteboard(), historyFile: ShellHistory.file)
        let (frame, textPass) = (frame, textPass)
        renderer.overlay = { commands, target in
            textPass.encode(frame.take(), into: commands, target: target)
        }
    }

    /// The cell's height in pixels, for a wheel's lines.
    var cellHeight: Double { Double(ShellMetrics(atlas: textPass.atlas).cellHeight) }

    /// A key press or release: true when it is the shell's.
    func key(_ event: NSEvent) -> Bool {
        guard overlay.isOpen || event.keyCode == ShellKey.toggleCode else { return false }
        if event.type == .keyUp || (event.isARepeat && !overlay.isOpen) { return overlay.isOpen }
        guard let key = ShellKey(event) else { return overlay.isOpen }
        return overlay.handle(key) || overlay.isOpen
    }

    /// The pointer: the shell's while it is open.
    func pointer(_ phase: EngineView.PointerPhase, x: Double, y: Double) -> Bool {
        guard overlay.isOpen else { return false }
        switch phase {
        case .down: overlay.pointer(.down, x: x, y: y)
        case .dragged: overlay.pointer(.dragged, x: x, y: y)
        case .up: overlay.pointer(.up, x: x, y: y)
        case .moved, .rightDown: break
        }
        return true
    }

    /// The wheel: the shell's while it is open (a notch three rows).
    func scroll(delta: Double, precise: Bool) -> Bool {
        overlay.scrollWheel(pixels: precise ? delta : delta * 3 * cellHeight)
    }

    /// Each poll: the socket's request, the slide, and what it draws over
    /// a frame `width` by `height` pixels.
    func update(request: Bool?, seconds: Double, width: Int, height: Int) {
        switch request {
        case true?: overlay.open()
        case false?: overlay.close()
        case nil: break
        }
        overlay.update(seconds: seconds, now: ProcessInfo.processInfo.systemUptime)
        frame.set(
            overlay.isVisible
                ? overlay.items(width: width, height: height, metrics: ShellMetrics(atlas: textPass.atlas)) : [])
    }
}

/// The shell's copying and pasting, with the system's clipboard.
final class SystemPasteboard: ShellPasteboard {
    func copy(_ text: String) {
        NSPasteboard.general.clearContents()
        NSPasteboard.general.setString(text, forType: .string)
    }

    func paste() -> String? {
        NSPasteboard.general.string(forType: .string)
    }
}

extension ShellKey {
    /// The shell's key for an AppKit key press.
    init?(_ event: NSEvent) {
        let flags = event.modifierFlags
        var modifiers: Modifiers = []
        if flags.contains(.shift) { modifiers.insert(.shift) }
        if flags.contains(.control) { modifiers.insert(.control) }
        if flags.contains(.option) { modifiers.insert(.option) }
        if flags.contains(.command) { modifiers.insert(.command) }
        self.init(
            code: event.keyCode, characters: event.characters ?? "",
            ignoringModifiers: event.charactersIgnoringModifiers ?? "", modifiers: modifiers)
    }
}
