// The game's sounds: an AVAudioSourceNode pulls game.wasm's own mix
// (GameSpeakers), interleaved stereo, on the audio thread. Without an
// output device the game is silent.
import AVFoundation
import Foundation
import OpenCEEngineCore

public final class SoundOutput {
    private let engine = AVAudioEngine()
    private var node: AVAudioSourceNode?

    /// An output of game.wasm's mix; it starts playing at once.
    public convenience init(speakers: GameSpeakers) {
        self.init(rate: Double(speakers.sampleRate())) { speakers.pull(frames: $0) }
    }

    /// An output of what `pull` gives for so many frames, at `rate`.
    public init(rate: Double, pull: @escaping (UInt32) -> [Float]) {
        guard let format = AVAudioFormat(standardFormatWithSampleRate: rate, channels: 2) else { return }
        let node = AVAudioSourceNode(format: format) { _, _, frameCount, buffers -> OSStatus in
            let mixed = pull(frameCount)
            let channels = UnsafeMutableAudioBufferListPointer(buffers)
            // (the standard format is deinterleaved: one buffer a channel)
            for (channel, buffer) in channels.enumerated() {
                guard let samples = buffer.mData?.assumingMemoryBound(to: Float.self) else { continue }
                for frame in 0..<Int(frameCount) {
                    let index = frame * 2 + min(channel, 1)
                    samples[frame] = index < mixed.count ? mixed[index] : 0
                }
            }
            return noErr
        }
        engine.attach(node)
        engine.connect(node, to: engine.mainMixerNode, format: format)
        self.node = node
        try? engine.start()
    }

    /// Whether it is playing.
    public var isPlaying: Bool { engine.isRunning }

    /// Stops it.
    public func stop() {
        engine.stop()
    }
}
