// What the developer shell's overlay gets back from the engine it runs
// lines through: the running game's console (WasmGame.ask), or a test's
// fake.
import Foundation
import OpenCEEngineCore

/// A line's reply: whether it ran, its text, and its data as JSON
/// (`{"clear": true}` clears the scrollback).
public struct ShellOutput: Equatable, Sendable {
    public var ok: Bool
    public var text: String
    public var json: String
    public init(ok: Bool, text: String, json: String = "null") {
        self.ok = ok
        self.text = text
        self.json = json
    }
}

/// A watched expression and its value now.
public struct WatchValue: Equatable, Sendable {
    public var expression: String
    public var value: String
    public var ok: Bool
    public init(expression: String, value: String, ok: Bool) {
        self.expression = expression
        self.value = value
        self.ok = ok
    }
}

/// A line of the engine's log, numbered in the order written.
public struct LogEntry: Equatable, Sendable {
    public var sequence: UInt64
    public var level: String
    public var target: String
    public var text: String
    public init(sequence: UInt64, level: String, target: String, text: String) {
        self.level = level
        self.target = target
        self.text = text
        self.sequence = sequence
    }
}

/// The running game as the shell's engine: each line goes to the game's
/// own console (the in-process console that `debug.telnet_console`
/// turns on), which has no completions, watches or log to give.
public final class GameShellEngine: ShellEngine {
    private let game: WasmGame
    /// How long a line may wait for the game's reply.
    private let milliseconds: UInt32

    public init(game: WasmGame, milliseconds: UInt32 = 500) {
        self.game = game
        self.milliseconds = milliseconds
    }

    public func runLine(_ line: String) -> ShellOutput {
        guard let reply = game.ask(line: line, milliseconds: milliseconds) else {
            return ShellOutput(ok: false, text: "the console is not listening")
        }
        return ShellOutput(ok: true, text: reply)
    }

    public func completions(_ line: String) -> [String] { [] }
    public func watches() -> [WatchValue] { [] }
    public func logLines(since sequence: UInt64, max: UInt32) -> [LogEntry] { [] }
}
