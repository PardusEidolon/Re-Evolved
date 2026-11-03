// What ray traced shadows and reflections cost on this Mac's GPU, measured
// before either is drawn (lighting_original Phase 5: "where the M1 Max's
// budget allows"). A hybrid renderer rasterizes its positions and normals and
// traces only the secondary rays, so only those are timed: one shadow ray a
// pixel toward the sun (any hit), and one reflection ray a pixel (closest
// hit), against a terrain of a structure BSP's size with boxes standing on
// it. The first test checks the rays answer rightly on a scene small enough
// to know; the second times them at 3840×2160 and prints the numbers.

import Metal
import simd
import XCTest

private let kernels = """
    #include <metal_stdlib>
    #include <metal_raytracing>
    using namespace metal;
    using namespace metal::raytracing;

    struct View { float3 eye; float3 forward; float3 right; float3 up; float3 sun; uint orthographic; };

    // the rasterized pass's stand-in: each pixel's first hit, and its normal
    kernel void positions(primitive_acceleration_structure scene [[buffer(0)]],
                          constant View &view [[buffer(1)]],
                          device const packed_float3 *vertices [[buffer(2)]],
                          texture2d<float, access::write> position [[texture(0)]],
                          texture2d<float, access::write> normal [[texture(1)]],
                          uint2 at [[thread_position_in_grid]]) {
        float2 size = float2(position.get_width(), position.get_height());
        if (at.x >= uint(size.x) || at.y >= uint(size.y)) return;
        float2 uv = (float2(at) + 0.5) / size * 2.0 - 1.0;
        ray r;
        if (view.orthographic) {
            r.origin = view.eye + view.right * uv.x + view.up * uv.y;
            r.direction = view.forward;
        } else {
            r.origin = view.eye;
            r.direction = normalize(view.forward + view.right * uv.x * size.x / size.y - view.up * uv.y);
        }
        r.min_distance = 0.0;
        r.max_distance = 1.0e4;
        intersector<triangle_data> trace;
        intersection_result<triangle_data> hit = trace.intersect(r, scene);
        if (hit.type == intersection_type::none) {
            position.write(float4(0.0), at);
            return;
        }
        uint first = hit.primitive_id * 3;
        float3 a = vertices[first], b = vertices[first + 1], c = vertices[first + 2];
        float3 n = normalize(cross(b - a, c - a));
        if (dot(n, r.direction) > 0.0) n = -n;
        position.write(float4(r.origin + r.direction * hit.distance, 1.0), at);
        normal.write(float4(n, 0.0), at);
    }

    // 1 where the sun is hidden from the pixel
    kernel void shadows(primitive_acceleration_structure scene [[buffer(0)]],
                        constant View &view [[buffer(1)]],
                        texture2d<float, access::read> position [[texture(0)]],
                        texture2d<float, access::read> normal [[texture(1)]],
                        texture2d<float, access::write> shadowed [[texture(2)]],
                        uint2 at [[thread_position_in_grid]]) {
        if (at.x >= position.get_width() || at.y >= position.get_height()) return;
        float4 p = position.read(at);
        if (p.w == 0.0) { shadowed.write(float4(0.0), at); return; }
        ray r;
        r.origin = p.xyz + normal.read(at).xyz * 1.0e-3;
        r.direction = view.sun;
        r.min_distance = 0.0;
        r.max_distance = 1.0e4;
        intersector<triangle_data> trace;
        trace.accept_any_intersection(true);
        intersection_result<triangle_data> hit = trace.intersect(r, scene);
        shadowed.write(float4(hit.type == intersection_type::none ? 0.0 : 1.0), at);
    }

    // the distance to what the pixel reflects, 0 for the sky
    kernel void reflections(primitive_acceleration_structure scene [[buffer(0)]],
                            constant View &view [[buffer(1)]],
                            texture2d<float, access::read> position [[texture(0)]],
                            texture2d<float, access::read> normal [[texture(1)]],
                            texture2d<float, access::write> reflected [[texture(2)]],
                            uint2 at [[thread_position_in_grid]]) {
        if (at.x >= position.get_width() || at.y >= position.get_height()) return;
        float4 p = position.read(at);
        if (p.w == 0.0) { reflected.write(float4(0.0), at); return; }
        float3 n = normal.read(at).xyz;
        ray r;
        r.origin = p.xyz + n * 1.0e-3;
        r.direction = reflect(view.orthographic ? view.forward : normalize(p.xyz - view.eye), n);
        r.min_distance = 0.0;
        r.max_distance = 1.0e4;
        intersector<triangle_data> trace;
        intersection_result<triangle_data> hit = trace.intersect(r, scene);
        reflected.write(float4(hit.type == intersection_type::none ? 0.0 : hit.distance), at);
    }
    """

/// the view the kernels read, laid out as MSL's `View` (float3s at 16 bytes)
private struct View {
    var eye: SIMD4<Float>
    var forward: SIMD4<Float>
    var right: SIMD4<Float>
    var up: SIMD4<Float>
    var sun: SIMD4<Float>
    var orthographic: UInt32
    var padding: (UInt32, UInt32, UInt32) = (0, 0, 0)
}

/// one triangle's three corners
private typealias Triangle = (SIMD3<Float>, SIMD3<Float>, SIMD3<Float>)

private struct Tracer {
    let device: MTLDevice
    let queue: MTLCommandQueue
    let positions: MTLComputePipelineState
    let shadows: MTLComputePipelineState
    let reflections: MTLComputePipelineState

    init?() {
        guard let device = MTLCreateSystemDefaultDevice(), device.supportsRaytracing,
            let queue = device.makeCommandQueue(),
            let library = try? device.makeLibrary(source: kernels, options: nil),
            let positions = library.makeFunction(name: "positions"),
            let shadows = library.makeFunction(name: "shadows"),
            let reflections = library.makeFunction(name: "reflections"),
            let made = try? [positions, shadows, reflections].map({
                try device.makeComputePipelineState(function: $0)
            })
        else { return nil }
        self.device = device
        self.queue = queue
        self.positions = made[0]
        self.shadows = made[1]
        self.reflections = made[2]
    }

    /// the vertices, packed three floats each, and the scene built from them
    func scene(_ triangles: [Triangle]) -> (MTLBuffer, MTLAccelerationStructure)? {
        let floats = triangles.flatMap { [$0.0, $0.1, $0.2] }.flatMap { [$0.x, $0.y, $0.z] }
        guard
            let vertices = device.makeBuffer(
                bytes: floats, length: floats.count * 4, options: .storageModeShared)
        else { return nil }
        let geometry = MTLAccelerationStructureTriangleGeometryDescriptor()
        geometry.vertexBuffer = vertices
        geometry.vertexStride = 12
        geometry.vertexFormat = .float3
        geometry.triangleCount = triangles.count
        geometry.opaque = true
        let descriptor = MTLPrimitiveAccelerationStructureDescriptor()
        descriptor.geometryDescriptors = [geometry]
        let sizes = device.accelerationStructureSizes(descriptor: descriptor)
        guard let built = device.makeAccelerationStructure(size: sizes.accelerationStructureSize),
            let scratch = device.makeBuffer(length: sizes.buildScratchBufferSize, options: .storageModePrivate),
            let commands = queue.makeCommandBuffer(),
            let encoder = commands.makeAccelerationStructureCommandEncoder()
        else { return nil }
        encoder.build(
            accelerationStructure: built, descriptor: descriptor,
            scratchBuffer: scratch, scratchBufferOffset: 0)
        encoder.endEncoding()
        commands.commit()
        commands.waitUntilCompleted()
        return (vertices, built)
    }

    func texture(_ width: Int, _ height: Int, _ format: MTLPixelFormat) -> MTLTexture? {
        let descriptor = MTLTextureDescriptor.texture2DDescriptor(
            pixelFormat: format, width: width, height: height, mipmapped: false)
        descriptor.usage = [.shaderRead, .shaderWrite]
        descriptor.storageMode = .shared
        return device.makeTexture(descriptor: descriptor)
    }

    /// runs `pipeline` once over `textures`, and the GPU's milliseconds for it
    @discardableResult
    func run(
        _ pipeline: MTLComputePipelineState, _ scene: MTLAccelerationStructure, _ vertices: MTLBuffer,
        _ view: View, _ textures: [MTLTexture]
    ) -> Double? {
        guard let commands = queue.makeCommandBuffer(), let encoder = commands.makeComputeCommandEncoder()
        else { return nil }
        var view = view
        encoder.setComputePipelineState(pipeline)
        encoder.setAccelerationStructure(scene, bufferIndex: 0)
        encoder.setBytes(&view, length: MemoryLayout<View>.stride, index: 1)
        encoder.setBuffer(vertices, offset: 0, index: 2)
        encoder.useResource(vertices, usage: .read)
        encoder.setTextures(textures, range: 0..<textures.count)
        encoder.dispatchThreads(
            MTLSize(width: textures[0].width, height: textures[0].height, depth: 1),
            threadsPerThreadgroup: MTLSize(width: 8, height: 8, depth: 1))
        encoder.endEncoding()
        commands.commit()
        commands.waitUntilCompleted()
        guard commands.status == .completed else { return nil }
        return (commands.gpuEndTime - commands.gpuStartTime) * 1000
    }
}

/// the floats of a one-channel float texture
private func floats(_ texture: MTLTexture) -> [Float] {
    var out = [Float](repeating: 0, count: texture.width * texture.height)
    out.withUnsafeMutableBytes { bytes in
        if let base = bytes.baseAddress {
            texture.getBytes(
                base, bytesPerRow: texture.width * 4,
                from: MTLRegionMake2D(0, 0, texture.width, texture.height), mipmapLevel: 0)
        }
    }
    return out
}

/// a square from (x0, y0) to (x1, y1) at height z, as two triangles
private func square(_ x0: Float, _ y0: Float, _ x1: Float, _ y1: Float, _ z: Float) -> [Triangle] {
    let (a, b, c, d) = (SIMD3(x0, y0, z), SIMD3(x1, y0, z), SIMD3(x1, y1, z), SIMD3(x0, y1, z))
    return [(a, b, c), (a, c, d)]
}

/// a terrain of `cells`×`cells` squares over 1000 units, with `boxes`
/// boxes standing on it: the size of a structure BSP and its scenery
private func terrain(cells: Int, boxes: Int) -> [Triangle] {
    func height(_ x: Float, _ y: Float) -> Float {
        8 * sin(x * 0.011) * cos(y * 0.007) + 2 * sin(x * 0.05 + y * 0.03)
    }
    var triangles: [Triangle] = []
    triangles.reserveCapacity(cells * cells * 2 + boxes * 12)
    let step = 1000 / Float(cells)
    for row in 0..<cells {
        for column in 0..<cells {
            let (x0, y0) = (Float(column) * step, Float(row) * step)
            let (x1, y1) = (x0 + step, y0 + step)
            let a = SIMD3(x0, y0, height(x0, y0)), b = SIMD3(x1, y0, height(x1, y0))
            let c = SIMD3(x1, y1, height(x1, y1)), d = SIMD3(x0, y1, height(x0, y1))
            triangles += [(a, b, c), (a, c, d)]
        }
    }
    // (boxes in a fixed pattern, so each run measures the same scene)
    for index in 0..<boxes {
        let x = Float((index * 7919) % 997), y = Float((index * 104_729) % 991)
        let (x1, y1, z0) = (x + 6, y + 6, height(x, y) - 1)
        let z1 = z0 + 4 + Float(index % 5) * 3
        let corners = [SIMD3(x, y, z0), SIMD3(x1, y, z0), SIMD3(x1, y1, z0), SIMD3(x, y1, z0)]
        let tops = corners.map { SIMD3($0.x, $0.y, z1) }
        triangles += [(tops[0], tops[1], tops[2]), (tops[0], tops[2], tops[3])]
        for side in 0..<4 {
            let next = (side + 1) % 4
            triangles += [(corners[side], corners[next], tops[next]), (corners[side], tops[next], tops[side])]
        }
        triangles += [(corners[0], corners[2], corners[1]), (corners[0], corners[3], corners[2])]
    }
    return triangles
}

final class RayBudgetTests: XCTestCase {
    func testTheRaysFindTheShadowAndTheReflectionOfASceneThatIsKnown() throws {
        let tracer = try XCTUnwrap(Tracer(), "needs a Metal device that ray traces")
        // a floor at 0 over [0, 4]², and a roof at 1 over its left quarter,
        // seen straight down from above with the sun overhead
        let (vertices, scene) = try XCTUnwrap(tracer.scene(square(0, 0, 4, 4, 0) + square(0, 0, 1, 4, 1)))
        let view = View(
            eye: [2, 2, 10, 0], forward: [0, 0, -1, 0], right: [2, 0, 0, 0], up: [0, 2, 0, 0],
            sun: [0, 0, 1, 0], orthographic: 1)
        let (width, height) = (16, 16)
        let position = try XCTUnwrap(tracer.texture(width, height, .rgba32Float))
        let normal = try XCTUnwrap(tracer.texture(width, height, .rgba32Float))
        let shadowed = try XCTUnwrap(tracer.texture(width, height, .r32Float))
        let reflected = try XCTUnwrap(tracer.texture(width, height, .r32Float))
        XCTAssertNotNil(tracer.run(tracer.positions, scene, vertices, view, [position, normal]))
        XCTAssertNotNil(tracer.run(tracer.shadows, scene, vertices, view, [position, normal, shadowed]))
        XCTAssertNotNil(tracer.run(tracer.reflections, scene, vertices, view, [position, normal, reflected]))
        // (the roof is seen, not the floor under it, so nothing is shadowed:
        // the sun is straight up from both)
        XCTAssertEqual(floats(shadowed).filter { $0 > 0 }.count, 0)
        // seen from below the roof's height, the floor under it is in shadow
        let low = View(
            eye: [2, 2, 0.5, 0], forward: [0, 0, -1, 0], right: [2, 0, 0, 0], up: [0, 2, 0, 0],
            sun: [0, 0, 1, 0], orthographic: 1)
        XCTAssertNotNil(tracer.run(tracer.positions, scene, vertices, low, [position, normal]))
        XCTAssertNotNil(tracer.run(tracer.shadows, scene, vertices, low, [position, normal, shadowed]))
        XCTAssertNotNil(tracer.run(tracer.reflections, scene, vertices, low, [position, normal, reflected]))
        XCTAssertEqual(floats(shadowed).filter { $0 > 0 }.count, width * height / 4)
        // and the floor there reflects the roof, 1 above it; the rest, the sky
        let distances = floats(reflected)
        XCTAssertEqual(distances.filter { abs($0 - 1) < 0.01 }.count, width * height / 4)
        XCTAssertEqual(distances.filter { $0 == 0 }.count, width * height * 3 / 4)
    }

    func testTheRaysCostAt4K() throws {
        let tracer = try XCTUnwrap(Tracer(), "needs a Metal device that ray traces")
        // a player's eye looking across the terrain, the sun low in the west
        let sun = simd.normalize(SIMD3<Float>(-0.6, 0.2, 0.5))
        let view = View(
            eye: [100, 100, 14, 0], forward: [0.7, 0.7, -0.15, 0], right: [0.4, -0.4, 0, 0],
            up: [0.05, 0.05, 0.45, 0], sun: SIMD4(sun, 0), orthographic: 0)
        // (at 4K, and at half of it each way, as traced at a quarter of the
        // pixels and filtered up)
        for (width, height, cells, boxes) in [
            (3840, 2160, 156, 400), (3840, 2160, 312, 1600),
            (1920, 1080, 156, 400), (1920, 1080, 312, 1600),
        ] {
            let position = try XCTUnwrap(tracer.texture(width, height, .rgba32Float))
            let normal = try XCTUnwrap(tracer.texture(width, height, .rgba32Float))
            let shadowed = try XCTUnwrap(tracer.texture(width, height, .r32Float))
            let reflected = try XCTUnwrap(tracer.texture(width, height, .r32Float))
            let triangles = terrain(cells: cells, boxes: boxes)
            let (vertices, scene) = try XCTUnwrap(tracer.scene(triangles))
            XCTAssertNotNil(tracer.run(tracer.positions, scene, vertices, view, [position, normal]))
            var shadowTimes: [Double] = []
            var reflectionTimes: [Double] = []
            for _ in 0..<40 {
                shadowTimes.append(
                    try XCTUnwrap(
                        tracer.run(tracer.shadows, scene, vertices, view, [position, normal, shadowed])))
                reflectionTimes.append(
                    try XCTUnwrap(
                        tracer.run(tracer.reflections, scene, vertices, view, [position, normal, reflected])))
            }
            // (the first runs warm the caches up; the rest are measured)
            let shadows = Array(shadowTimes.dropFirst(5)).sorted()
            let reflections = Array(reflectionTimes.dropFirst(5)).sorted()
            let lit = floats(shadowed).filter { $0 > 0 }.count
            let reflecting = floats(reflected).filter { $0 > 0 }.count
            print(
                String(
                    format: "ray budget: %d triangles at %dx%d: shadows p50 %.3f p99 %.3f ms (%.1f%% shadowed), "
                        + "reflections p50 %.3f p99 %.3f ms (%.1f%% hit)",
                    triangles.count, width, height, shadows[shadows.count / 2], shadows[shadows.count - 1],
                    Double(lit) * 100 / Double(width * height), reflections[reflections.count / 2],
                    reflections[reflections.count - 1], Double(reflecting) * 100 / Double(width * height)))
            XCTAssertGreaterThan(lit, 0, "nothing in the terrain casts a shadow")
            XCTAssertGreaterThan(reflecting, 0, "nothing in the terrain is reflected")
        }
    }
}
