// The Metal binary archive of the game's render pipelines: each pipeline
// the game asks for is compiled for the GPU once, kept in the archive at
// `url`, and found there on the next run, so the game never waits on
// Metal's compiler while it plays. A pipeline is looked for with
// `failOnBinaryArchiveMiss`; one the archive lacks is a miss, made in the
// ordinary way and added. The archive is written back (to a temporary file,
// then moved over the old) when something was added.

import Foundation
import Metal

public final class PipelineArchive: @unchecked Sendable {
    /// Where the archive is kept.
    public let url: URL
    /// The archive Metal looks pipelines up in.
    public let metal: MTLBinaryArchive
    private let lock = NSLock()
    private var added = 0

    /// Opens the archive at `url` (a new, empty one if there is none there,
    /// or the one there cannot be read).
    public init?(device: MTLDevice, url: URL) {
        let descriptor = MTLBinaryArchiveDescriptor()
        if FileManager.default.fileExists(atPath: url.path) {
            descriptor.url = url
        }
        if let archive = try? device.makeBinaryArchive(descriptor: descriptor) {
            metal = archive
        } else {
            // (an archive Metal will not read, from another OS or GPU, is replaced)
            guard let archive = try? device.makeBinaryArchive(descriptor: MTLBinaryArchiveDescriptor()) else {
                return nil
            }
            metal = archive
        }
        self.url = url
    }

    /// The pipelines added since the archive was opened or last saved.
    public var unsaved: Int {
        lock.lock()
        defer { lock.unlock() }
        return added
    }

    /// Makes the pipeline `descriptor` describes from the archive; nil if
    /// the archive lacks it.
    func find(_ device: MTLDevice, _ descriptor: MTLRenderPipelineDescriptor) -> MTLRenderPipelineState? {
        descriptor.binaryArchives = [metal]
        defer { descriptor.binaryArchives = nil }
        return try? device.makeRenderPipelineState(
            descriptor: descriptor, options: .failOnBinaryArchiveMiss, reflection: nil)
    }

    /// Compiles the pipeline `descriptor` describes into the archive.
    func add(_ descriptor: MTLRenderPipelineDescriptor) throws {
        lock.lock()
        defer { lock.unlock() }
        try metal.addRenderPipelineFunctions(descriptor: descriptor)
        added += 1
    }

    /// Writes the archive to its `url` if anything was added.
    public func save() throws {
        lock.lock()
        defer { lock.unlock() }
        guard added > 0 else { return }
        let files = FileManager.default
        try files.createDirectory(at: url.deletingLastPathComponent(), withIntermediateDirectories: true)
        let temporary = url.deletingLastPathComponent()
            .appendingPathComponent(".\(url.lastPathComponent).\(UUID().uuidString)")
        try metal.serialize(to: temporary)
        if files.fileExists(atPath: url.path) {
            _ = try files.replaceItemAt(url, withItemAt: temporary)
        } else {
            try files.moveItem(at: temporary, to: url)
        }
        added = 0
    }
}
