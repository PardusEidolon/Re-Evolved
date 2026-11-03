// The window's input handed to game.wasm: each key with its character and
// the modifiers held (the game's own keyboard reads them, as the
// original's DirectInput and WM_CHAR did), the held mouse buttons and
// modifier keys as they change, the mouse's motion, and the wheel in
// notches, and the free mouse's place while the menus are up. WasmGame
// takes them through GameControls.
import OpenCEEngineCore

/// What the game takes of the input.
public protocol GameControls: AnyObject {
    func macKey(code: UInt16, character: UInt8, control: Bool, shift: Bool, alt: Bool, down: Bool)
    func mouseButton(button: UInt8, down: Bool)
    func mouseMotion(dx: Float, dy: Float)
    func mouseWheel(notches: Float)
    func mousePosition(x: Float, y: Float)
}

extension WasmGame: GameControls {}

public struct GameInput {
    /// a trackpad's pixels of scrolling a wheel's notch
    public static let pixelsANotch = 40.0

    private let game: GameControls
    private var held: Set<UInt16> = []
    private var scrolled = 0.0

    public init(game: GameControls) {
        self.game = game
    }

    /// A key going down or up, with the characters it types (the first,
    /// if ASCII, is the game's) and the modifiers held.
    public mutating func key(code: UInt16, characters: String, control: Bool, shift: Bool, alt: Bool, down: Bool) {
        let character = characters.unicodeScalars.first.flatMap { $0.isASCII ? UInt8($0.value) : nil } ?? 0
        game.macKey(code: code, character: character, control: control, shift: shift, alt: alt, down: down)
    }

    /// The keys the window holds (the mouse's buttons from 0x100, and the
    /// modifier keys) and the mouse's motion since the last frame: each
    /// change goes to the game, in the codes' order, then the motion.
    public mutating func held(_ codes: Set<UInt16>, motion: (dx: Double, dy: Double)) {
        for code in held.symmetricDifference(codes).sorted() {
            let down = codes.contains(code)
            if let button = Self.button(code) {
                game.mouseButton(button: button, down: down)
            } else {
                game.macKey(code: code, character: 0, control: false, shift: false, alt: false, down: down)
            }
        }
        held = codes
        if motion.dx != 0 || motion.dy != 0 {
            game.mouseMotion(dx: Float(motion.dx), dy: Float(motion.dy))
        }
    }

    /// The free mouse at `x` and `y`, the drawable's pixels from its top
    /// left: the menus' pointer while the window leaves the mouse free.
    public func point(x: Double, y: Double) {
        game.mousePosition(x: Float(x), y: Float(y))
    }

    /// A button of the free mouse going down or up at `x` and `y`: the
    /// pointer goes there first, so the press falls where it was made.
    public func press(button: UInt8, down: Bool, x: Double, y: Double) {
        point(x: x, y: y)
        game.mouseButton(button: button, down: down)
    }

    /// The wheel turning: a wheel's lines are notches; a trackpad's pixels
    /// gather until they make one.
    public mutating func scroll(delta: Double, precise: Bool) {
        guard precise else {
            if delta != 0 { game.mouseWheel(notches: Float(delta)) }
            return
        }
        scrolled += delta
        let notches = (scrolled / Self.pixelsANotch).rounded(.towardZero)
        if notches != 0 {
            scrolled -= notches * Self.pixelsANotch
            game.mouseWheel(notches: Float(notches))
        }
    }

    /// The game's button for the window's code of a mouse button: 1 the
    /// left, 2 the middle, 3 the right, 4 and 5 the side ones.
    static func button(_ code: UInt16) -> UInt8? {
        switch code {
        case 0x100: 1
        case 0x101: 3
        case 0x102: 2
        case 0x103: 4
        case 0x104: 5
        default: nil
        }
    }
}
