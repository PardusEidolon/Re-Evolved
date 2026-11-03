// The window's input handed to game.wasm (GameInput): keys with their
// character and modifiers, the mouse's buttons and modifier keys as they
// change among the held keys, its motion, and the wheel in notches.
// Headless: no window is opened.
import OpenCEEngineKit
import XCTest

/// What the game was handed, in order.
private final class Recorded: GameControls {
    var calls: [String] = []

    func macKey(code: UInt16, character: UInt8, control: Bool, shift: Bool, alt: Bool, down: Bool) {
        calls.append("key \(code) \(character) \(control) \(shift) \(alt) \(down)")
    }
    func mouseButton(button: UInt8, down: Bool) { calls.append("button \(button) \(down)") }
    func mouseMotion(dx: Float, dy: Float) { calls.append("motion \(dx) \(dy)") }
    func mouseWheel(notches: Float) { calls.append("wheel \(notches)") }
    func mousePosition(x: Float, y: Float) { calls.append("at \(x) \(y)") }
}

final class GameInputTests: XCTestCase {
    func testTheFreeMouseInTheMenusIsWhereItIsAndItsPressesThere() {
        let game = Recorded()
        let input = GameInput(game: game)
        input.point(x: 100, y: 50)
        input.press(button: 1, down: true, x: 120, y: 60)
        input.press(button: 1, down: false, x: 121, y: 61)
        XCTAssertEqual(
            game.calls, ["at 100.0 50.0", "at 120.0 60.0", "button 1 true", "at 121.0 61.0", "button 1 false"])
    }

    func testAKeyGoesWithItsCharacterAndModifiers() {
        let game = Recorded()
        var input = GameInput(game: game)
        input.key(code: 13, characters: "W", control: false, shift: true, alt: false, down: true)
        input.key(code: 13, characters: "", control: false, shift: false, alt: false, down: false)
        // (a character past ASCII has none for the game)
        input.key(code: 0, characters: "é", control: false, shift: false, alt: true, down: true)
        XCTAssertEqual(
            game.calls,
            ["key 13 87 false true false true", "key 13 0 false false false false", "key 0 0 false false true true"])
    }

    func testTheHeldButtonsAndModifiersGoAsTheyChange() {
        let game = Recorded()
        var input = GameInput(game: game)
        // left button (0x100) and left Shift (56) pressed, then the right (0x101) too
        input.held([0x100, 56], motion: (dx: 0, dy: 0))
        input.held([0x100, 0x101, 56], motion: (dx: 3, dy: -2))
        input.held([], motion: (dx: 0, dy: 0))
        XCTAssertEqual(
            game.calls,
            [
                "key 56 0 false false false true", "button 1 true", "button 3 true", "motion 3.0 -2.0",
                "key 56 0 false false false false", "button 1 false", "button 3 false",
            ])
    }

    func testTheWheelTurnsInNotches() {
        let game = Recorded()
        var input = GameInput(game: game)
        // a wheel's line is a notch; a trackpad's pixels gather to one
        input.scroll(delta: 1, precise: false)
        input.scroll(delta: 20, precise: true)
        input.scroll(delta: 30, precise: true)
        XCTAssertEqual(game.calls, ["wheel 1.0", "wheel 1.0"])
    }
}
