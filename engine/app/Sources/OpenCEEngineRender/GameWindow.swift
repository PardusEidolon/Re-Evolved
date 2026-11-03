// What the game's passes on colour texture 0 draw to (GameRenderer): the
// game's window (a CAMetalLayer's drawables), or a texture off the screen
// for the tests and headless runs.

import Foundation
import Metal
import QuartzCore

/// A frame of the window, drawn by the passes on colour 0 until Present.
public struct WindowFrame {
    public let texture: MTLTexture
    let drawable: MTLDrawable?
}

public protocol RenderWindow: AnyObject, Sendable {
    /// The frame to draw next, or nil when there is none to have.
    func nextFrame() -> WindowFrame?
    /// Shows `frame` once `commands` have run (before they are committed).
    func present(_ frame: WindowFrame, in commands: MTLCommandBuffer)
}

/// The game's window: its view's CAMetalLayer, BGRA8, once the game has
/// opened it (the game makes its textures before).
public final class LayerWindow: RenderWindow, @unchecked Sendable {
    private let lock = NSLock()
    private var shown: CAMetalLayer?

    public init(layer: CAMetalLayer? = nil) {
        shown = layer
    }

    /// The layer drawn to (set on the main thread, read on the game's).
    public var layer: CAMetalLayer? {
        get {
            lock.lock()
            defer { lock.unlock() }
            return shown
        }
        set {
            lock.lock()
            defer { lock.unlock() }
            shown = newValue
        }
    }

    public func nextFrame() -> WindowFrame? {
        // (blocks while the layer's drawables are all in flight, which keeps
        // the game no more than their number of frames ahead)
        guard let drawable = layer?.nextDrawable() else { return nil }
        return WindowFrame(texture: drawable.texture, drawable: drawable)
    }

    public func present(_ frame: WindowFrame, in commands: MTLCommandBuffer) {
        if let drawable = frame.drawable {
            commands.present(drawable)
        }
    }
}

/// A window off the screen: one BGRA8 texture, which each frame draws over,
/// and a count of the frames presented; the frame presented `capture`th is
/// also copied aside, as it was shown, for the golden frames.
public final class OffscreenWindow: RenderWindow, @unchecked Sendable {
    public let texture: MTLTexture
    /// the copy of the frame presented `capture`th, once it is
    public let captured: MTLTexture?
    private let capture: Int?
    private let lock = NSLock()
    private var presentedFrames = 0
    private var didCapture = false
    private let device: MTLDevice

    /// Asked at each frame presented, with the count of frames presented
    /// so far: where to hand that frame, once its commands have finished
    /// (BGRA, as the window holds it), or nil to let it go. The map
    /// audit's frames are taken through it.
    public var snapshot: ((Int) -> ((CapturedFrame) -> Void)?)? {
        get {
            lock.lock()
            defer { lock.unlock() }
            return snapshotHandler
        }
        set {
            lock.lock()
            defer { lock.unlock() }
            snapshotHandler = newValue
        }
    }
    private var snapshotHandler: ((Int) -> ((CapturedFrame) -> Void)?)?

    public init?(device: MTLDevice, width: Int, height: Int, capture: Int? = nil) {
        self.device = device
        let descriptor = MTLTextureDescriptor.texture2DDescriptor(
            pixelFormat: .bgra8Unorm, width: width,
            height: height, mipmapped: false)
        descriptor.usage = [.renderTarget, .shaderRead]
        descriptor.storageMode = .shared
        guard let texture = device.makeTexture(descriptor: descriptor) else { return nil }
        self.texture = texture
        self.capture = capture
        if capture != nil {
            guard let copy = device.makeTexture(descriptor: descriptor) else { return nil }
            captured = copy
        } else {
            captured = nil
        }
    }

    public func nextFrame() -> WindowFrame? {
        WindowFrame(texture: texture, drawable: nil)
    }

    public func present(_ frame: WindowFrame, in commands: MTLCommandBuffer) {
        lock.lock()
        defer { lock.unlock() }
        presentedFrames += 1
        // (copied in the frame's own commands, so the next frame cannot draw
        // over it first)
        if presentedFrames == capture, let captured, let blit = commands.timedBlitEncoder("game capture") {
            blit.copy(from: frame.texture, to: captured)
            blit.endEncoding()
            didCapture = true
        }
        if let take = snapshotHandler?(presentedFrames) {
            Self.snapshot(frame.texture, on: device, in: commands, to: take)
        }
    }

    /// `texture` copied aside in `commands`, and handed to `take` once they
    /// have finished.
    private static func snapshot(
        _ texture: MTLTexture, on device: MTLDevice, in commands: MTLCommandBuffer,
        to take: @escaping (CapturedFrame) -> Void
    ) {
        let descriptor = MTLTextureDescriptor.texture2DDescriptor(
            pixelFormat: texture.pixelFormat, width: texture.width, height: texture.height, mipmapped: false)
        descriptor.storageMode = .shared
        guard let copy = device.makeTexture(descriptor: descriptor),
            let blit = commands.timedBlitEncoder("game snapshot")
        else { return }
        blit.copy(from: texture, to: copy)
        blit.endEncoding()
        commands.addCompletedHandler { _ in
            let (width, height) = (copy.width, copy.height)
            var bytes = [UInt8](repeating: 0, count: width * height * 4)
            copy.getBytes(
                &bytes, bytesPerRow: width * 4, from: MTLRegionMake2D(0, 0, width, height), mipmapLevel: 0)
            take(CapturedFrame(width: width, height: height, bytes: bytes))
        }
    }

    /// Whether the frame to capture has been presented (and copied once its
    /// commands finish).
    public var hasCaptured: Bool {
        lock.lock()
        defer { lock.unlock() }
        return didCapture
    }

    /// The frames presented so far.
    public var presented: Int {
        lock.lock()
        defer { lock.unlock() }
        return presentedFrames
    }

    /// The texture's pixels, RGBA, rows from the top (once the renderer has
    /// finished drawing them).
    public func pixels() -> [[UInt8]] {
        Self.pixels(of: texture)
    }

    /// The captured frame (once the renderer has finished), or nil before
    /// it is presented.
    public func capturedFrame() -> CapturedFrame? {
        guard hasCaptured, let captured else { return nil }
        let (width, height) = (captured.width, captured.height)
        var bytes = [UInt8](repeating: 0, count: width * height * 4)
        captured.getBytes(&bytes, bytesPerRow: width * 4, from: MTLRegionMake2D(0, 0, width, height), mipmapLevel: 0)
        return CapturedFrame(width: width, height: height, bytes: bytes)
    }

    private static func pixels(of texture: MTLTexture) -> [[UInt8]] {
        let (width, height) = (texture.width, texture.height)
        var bytes = [UInt8](repeating: 0, count: width * height * 4)
        texture.getBytes(&bytes, bytesPerRow: width * 4, from: MTLRegionMake2D(0, 0, width, height), mipmapLevel: 0)
        return stride(from: 0, to: bytes.count, by: 4).map { [bytes[$0 + 2], bytes[$0 + 1], bytes[$0], bytes[$0 + 3]] }
    }
}
