// The Halo UI plugin's menus (engine/crates/halo_ui), drawn over the frame
// the game has drawn: the scene's rounded rectangles as distance fields
// and its lines in Overpass and OpenCE (port/assets/fonts), each face
// rasterised by CoreText at the size the line is drawn at, so the text is
// as sharp as the display. The scene is in the design's 3840×2160 units,
// scaled to fit the target and centred.
import CoreGraphics
import CoreText
import Foundation
import Metal
import OpenCEEngineCore

/// The design's screen the scene is laid out on.
public enum MenuDesign {
    public static let width: Float = 3840
    public static let height: Float = 2160

    /// The scale and offset putting the design's screen in a target
    /// `width` by `height` pixels: fit, and centred.
    public static func fit(width: Int, height: Int) -> (scale: Float, x: Float, y: Float) {
        let scale = min(Float(width) / Self.width, Float(height) / Self.height)
        return (scale, (Float(width) - Self.width * scale) / 2, (Float(height) - Self.height * scale) / 2)
    }
}

/// A face's file: Overpass at 900 and 750, and OpenCE. (The prototypes'
/// Semibold is Overpass 600, which the repository does not have; 750
/// stands in for it.)
public enum MenuFonts {
    public static func file(_ face: MenuFace) -> String {
        switch face {
        case .title: return "OpenCE-Regular.ttf"
        case .heavy: return "Overpass-900.ttf"
        case .bold, .semibold: return "Overpass-750.ttf"
        }
    }

    /// `face`'s file, where GlyphAtlas finds the shell's.
    public static func url(_ face: MenuFace, bundle: Bundle = .main) -> URL? {
        guard let mono = GlyphAtlas.url(bundle: bundle) else { return nil }
        let url = mono.deletingLastPathComponent().appendingPathComponent(file(face))
        return FileManager.default.fileExists(atPath: url.path) ? url : nil
    }
}

/// One face at one pixel size: its glyphs' coverage in the red channel and
/// a blurred copy, the glow, in the green, packed in rows.
public final class MenuGlyphs {
    struct Glyph {
        /// left, top, right, bottom in the atlas, in pixels
        let box: SIMD4<Float>
        /// from the pen at the baseline to the box's top left
        let offset: SIMD2<Float>
        let advance: Float
    }

    public let texture: MTLTexture
    public let pixelSize: Int
    /// from the baseline up and down, in pixels
    public let ascent: Float
    public let descent: Float
    let glyphs: [Unicode.Scalar: Glyph]
    /// how far the glow reaches past the ink, in pixels
    let spread: Int

    public init(url: URL, pixelSize: Int, device: MTLDevice) throws {
        guard let descriptors = CTFontManagerCreateFontDescriptorsFromURL(url as CFURL) as? [CTFontDescriptor],
            let descriptor = descriptors.first
        else { throw TextError.font("no font in \(url.path)") }
        guard pixelSize > 0, pixelSize <= 512 else { throw TextError.font("size \(pixelSize)") }
        let font = CTFontCreateWithFontDescriptor(descriptor, CGFloat(pixelSize), nil)
        self.pixelSize = pixelSize
        ascent = Float(CTFontGetAscent(font))
        descent = Float(CTFontGetDescent(font))
        let spread = max(1, pixelSize / 5)
        self.spread = spread

        var scalars: [Unicode.Scalar] = []
        var codes: [CGGlyph] = []
        for value in 0x20...0x7E {
            guard let scalar = Unicode.Scalar(value) else { continue }
            var units = Array(String(scalar).utf16)
            var glyph = [CGGlyph](repeating: 0, count: units.count)
            if CTFontGetGlyphsForCharacters(font, &units, &glyph, units.count) {
                scalars.append(scalar)
                codes.append(glyph[0])
            }
        }
        guard scalars.contains("M") else { throw TextError.font("no glyphs in \(url.lastPathComponent)") }
        var bounds = [CGRect](repeating: .zero, count: codes.count)
        var advances = [CGSize](repeating: .zero, count: codes.count)
        CTFontGetBoundingRectsForGlyphs(font, .horizontal, codes, &bounds, codes.count)
        CTFontGetAdvancesForGlyphs(font, .horizontal, codes, &advances, codes.count)

        // (shelves: each glyph's box padded by the glow's spread)
        let width = 2048, pad = spread + 1
        var placed: [(x: Int, y: Int, w: Int, h: Int)] = []
        var x = 0, y = 0, shelf = 0
        for bound in bounds {
            let w = Int(bound.width.rounded(.up)) + 2 * pad + 1, h = Int(bound.height.rounded(.up)) + 2 * pad + 1
            if x + w > width {
                (x, y, shelf) = (0, y + shelf, 0)
            }
            placed.append((x, y, w, h))
            x += w
            shelf = max(shelf, h)
        }
        let height = max(1, y + shelf)

        var ink = [UInt8](repeating: 0, count: width * height)
        try ink.withUnsafeMutableBytes { raw in
            guard
                let context = CGContext(
                    data: raw.baseAddress, width: width, height: height, bitsPerComponent: 8,
                    bytesPerRow: width, space: CGColorSpaceCreateDeviceGray(),
                    bitmapInfo: CGImageAlphaInfo.none.rawValue)
            else { throw TextError.resources("a bitmap of \(width)x\(height)") }
            context.setAllowsFontSmoothing(false)
            context.setShouldAntialias(true)
            context.setFillColor(gray: 1, alpha: 1)
            for (index, code) in codes.enumerated() {
                let box = placed[index], bound = bounds[index]
                // (a bitmap context's y is up from its bottom)
                var position = CGPoint(
                    x: CGFloat(box.x + pad) - bound.minX.rounded(.down),
                    y: CGFloat(height - box.y - box.h + pad) - bound.minY.rounded(.down))
                var glyph = code
                CTFontDrawGlyphs(font, &glyph, &position, 1, context)
            }
        }
        let glow = Self.blur(ink, width: width, height: height, radius: max(1, spread / 2))

        var glyphs: [Unicode.Scalar: Glyph] = [:]
        for (index, scalar) in scalars.enumerated() {
            let box = placed[index], bound = bounds[index]
            glyphs[scalar] = Glyph(
                box: SIMD4(Float(box.x), Float(box.y), Float(box.x + box.w), Float(box.y + box.h)),
                // (the pen was put pad - minX right of the box's left, and
                // its baseline pad + minY above the box's bottom)
                offset: SIMD2(
                    Float(bound.minX.rounded(.down)) - Float(pad),
                    -Float(box.h - pad) - Float(bound.minY.rounded(.down))),
                advance: Float(advances[index].width))
        }
        self.glyphs = glyphs

        var pixels = [UInt8](repeating: 0, count: width * height * 2)
        for index in 0..<(width * height) {
            pixels[index * 2] = ink[index]
            pixels[index * 2 + 1] = glow[index]
        }
        let textureDescriptor = MTLTextureDescriptor.texture2DDescriptor(
            pixelFormat: .rg8Unorm, width: width, height: height, mipmapped: false)
        textureDescriptor.usage = .shaderRead
        guard let texture = device.makeTexture(descriptor: textureDescriptor) else {
            throw TextError.resources("the menus' atlas")
        }
        pixels.withUnsafeBytes { raw in
            if let base = raw.baseAddress {
                texture.replace(
                    region: MTLRegionMake2D(0, 0, width, height), mipmapLevel: 0, withBytes: base,
                    bytesPerRow: width * 2)
            }
        }
        self.texture = texture
    }

    /// Three box blurs of `radius` across and down, about a Gaussian.
    static func blur(_ source: [UInt8], width: Int, height: Int, radius: Int) -> [UInt8] {
        var values = source.map { Float($0) }
        var scratch = values
        let span = Float(2 * radius + 1)
        for _ in 0..<3 {
            for row in 0..<height {
                let base = row * width
                var sum: Float = 0
                for x in -radius...radius { sum += values[base + min(max(x, 0), width - 1)] }
                for x in 0..<width {
                    scratch[base + x] = sum / span
                    sum += values[base + min(x + radius + 1, width - 1)] - values[base + max(x - radius, 0)]
                }
            }
            for column in 0..<width {
                var sum: Float = 0
                for y in -radius...radius { sum += scratch[min(max(y, 0), height - 1) * width + column] }
                for y in 0..<height {
                    values[y * width + column] = sum / span
                    sum +=
                        scratch[min(y + radius + 1, height - 1) * width + column]
                        - scratch[max(y - radius, 0) * width + column]
                }
            }
        }
        // (the glow brighter than the blur, so that it shows past thin strokes)
        return values.map { UInt8(min(255, $0 * 2.5)) }
    }

    func glyph(_ scalar: Unicode.Scalar) -> Glyph? { glyphs[scalar] ?? glyphs["?"] }

    /// How wide `text` is, in pixels.
    public func width(_ text: String) -> Float {
        text.unicodeScalars.reduce(0) { $0 + (glyph($1)?.advance ?? 0) }
    }

    /// Whether `scalar` has ink (for tests).
    public func has(_ scalar: Unicode.Scalar) -> Bool { glyphs[scalar] != nil }
}

/// A rounded rectangle, in the target's pixels.
struct MenuShapeInstance {
    /// left, top, right, bottom
    var rect: SIMD4<Float>
    /// top left, top right, bottom right, bottom left
    var radii: SIMD4<Float>
    var fill: SIMD4<Float>
    var borderColor: SIMD4<Float>
    /// the border's width, 1 if it is added to the frame rather than laid
    /// over it, then padding
    var border: SIMD4<Float>
}

/// A glyph's quad, in the target's pixels.
struct MenuGlyphInstance {
    var rect: SIMD4<Float>
    var uv: SIMD4<Float>
    var color: SIMD4<Float>
    /// 0 the ink, 1 the glow, then padding
    var channel: SIMD4<Float>
}

/// Draws a `MenuScene` over a finished frame.
public final class MenuPass {
    private let device: MTLDevice
    private var shapes: MTLRenderPipelineState
    private var glyphs: MTLRenderPipelineState
    private let sampler: MTLSamplerState
    private let urls: [MenuFace: URL]
    /// each face's atlases, by pixel size
    private var atlases: [MenuFace: [Int: MenuGlyphs]] = [:]
    private var buffers: [MTLBuffer?] = [nil, nil, nil]
    /// the commands that last read each buffer
    private var readers: [MTLCommandBuffer?] = [nil, nil, nil]
    private var frame = 0
    /// The shapes and glyph quads drawn in the last frame, for tests.
    public private(set) var lastDrawn = (shapes: 0, glyphs: 0)

    public static let shaderSource = """
        #include <metal_stdlib>
        using namespace metal;

        struct ShapeInstance {
            float4 rect;
            float4 radii;
            float4 fill;
            float4 border_color;
            float4 border;
        };

        struct GlyphInstance {
            float4 rect;
            float4 uv;
            float4 color;
            float4 channel;
        };

        struct ShapeVarying {
            float4 position [[position]];
            uint instance [[flat]];
        };

        struct GlyphVarying {
            float4 position [[position]];
            float2 uv;
            float4 color [[flat]];
            float channel [[flat]];
        };

        static float4 corner_position(float4 rect, uint vertex_id, float2 target) {
            const uint corners[6] = {0, 1, 2, 2, 1, 3};
            uint corner = corners[vertex_id];
            float2 pixel = float2((corner & 1) ? rect.z : rect.x, (corner & 2) ? rect.w : rect.y);
            return float4(pixel.x / target.x * 2 - 1, 1 - pixel.y / target.y * 2, 0, 1);
        }

        vertex ShapeVarying menu_shape_vertex(uint vertex_id [[vertex_id]], uint instance [[instance_id]],
                                              const device ShapeInstance *shapes [[buffer(0)]],
                                              constant float2 &target [[buffer(1)]]) {
            // (a pixel past each edge, for the edge's coverage)
            float4 rect = shapes[instance].rect + float4(-1, -1, 1, 1);
            ShapeVarying out;
            out.position = corner_position(rect, vertex_id, target);
            out.instance = instance;
            return out;
        }

        // a rounded box's signed distance, its corners' radii as the
        // quarter of the box `p` is in picks
        static float rounded_box(float2 p, float2 half_size, float4 radii) {
            float r = p.x < 0 ? (p.y < 0 ? radii.x : radii.w) : (p.y < 0 ? radii.y : radii.z);
            r = min(r, min(half_size.x, half_size.y));
            float2 q = abs(p) - half_size + r;
            return length(max(q, 0.0)) + min(max(q.x, q.y), 0.0) - r;
        }

        // premultiplied out
        fragment float4 menu_shape_fragment(ShapeVarying in [[stage_in]],
                                            const device ShapeInstance *shapes [[buffer(0)]]) {
            ShapeInstance shape = shapes[in.instance];
            float2 centre = (shape.rect.xy + shape.rect.zw) / 2;
            float2 half_size = (shape.rect.zw - shape.rect.xy) / 2;
            float d = rounded_box(in.position.xy - centre, half_size, shape.radii);
            float coverage = clamp(0.5 - d, 0.0, 1.0);
            float border = shape.border.x;
            float4 color = shape.fill;
            if (border > 0) {
                float inside = clamp(0.5 - (d + border), 0.0, 1.0);
                // (premultiplied before mixing, so a clear fill keeps its border's colour)
                float4 fill = float4(shape.fill.rgb * shape.fill.a, shape.fill.a);
                float4 edge = float4(shape.border_color.rgb * shape.border_color.a, shape.border_color.a);
                color = mix(edge, fill, inside) * coverage;
            } else {
                color = float4(color.rgb * color.a, color.a) * coverage;
            }
            // (added: with no alpha out the blend keeps all of the frame under it)
            return shape.border.y > 0 ? float4(color.rgb, 0) : color;
        }

        vertex GlyphVarying menu_glyph_vertex(uint vertex_id [[vertex_id]], uint instance [[instance_id]],
                                              const device GlyphInstance *glyphs [[buffer(0)]],
                                              constant float2 &target [[buffer(1)]]) {
            const uint corners[6] = {0, 1, 2, 2, 1, 3};
            uint corner = corners[vertex_id];
            GlyphInstance glyph = glyphs[instance];
            GlyphVarying out;
            out.position = corner_position(glyph.rect, vertex_id, target);
            out.uv = float2((corner & 1) ? glyph.uv.z : glyph.uv.x, (corner & 2) ? glyph.uv.w : glyph.uv.y);
            out.color = glyph.color;
            out.channel = glyph.channel.x;
            return out;
        }

        fragment float4 menu_glyph_fragment(GlyphVarying in [[stage_in]], texture2d<float> atlas [[texture(0)]],
                                            sampler linear [[sampler(0)]]) {
            float2 texel = atlas.sample(linear, in.uv).rg;
            float alpha = (in.channel > 0.5 ? texel.g : texel.r) * in.color.a;
            return float4(in.color.rgb * alpha, alpha);
        }
        """

    /// The pass, its faces found by `MenuFonts`; it throws when one is
    /// missing or its shaders do not build.
    public init(device: MTLDevice, outputFormat: MTLPixelFormat, bundle: Bundle = .main) throws {
        self.device = device
        var urls: [MenuFace: URL] = [:]
        for face in [MenuFace.title, .heavy, .bold, .semibold] {
            guard let url = MenuFonts.url(face, bundle: bundle) else {
                throw TextError.font("no \(MenuFonts.file(face))")
            }
            urls[face] = url
        }
        self.urls = urls
        do {
            let library = try device.makeLibrary(source: Self.shaderSource, options: GameRenderer.libraryOptions)
            shapes = try Self.pipeline(device, library, "menu_shape", outputFormat)
            glyphs = try Self.pipeline(device, library, "menu_glyph", outputFormat)
        } catch let error as TextError {
            throw error
        } catch {
            throw TextError.resources("the menus' shaders: \(error)")
        }
        let samplerDescriptor = MTLSamplerDescriptor()
        samplerDescriptor.minFilter = .linear
        samplerDescriptor.magFilter = .linear
        samplerDescriptor.sAddressMode = .clampToEdge
        samplerDescriptor.tAddressMode = .clampToEdge
        guard let sampler = device.makeSamplerState(descriptor: samplerDescriptor) else {
            throw TextError.resources("the menus' sampler")
        }
        self.sampler = sampler
    }

    private static func pipeline(
        _ device: MTLDevice, _ library: MTLLibrary, _ name: String, _ outputFormat: MTLPixelFormat
    ) throws -> MTLRenderPipelineState {
        guard let vertex = library.makeFunction(name: "\(name)_vertex"),
            let fragment = library.makeFunction(name: "\(name)_fragment")
        else { throw TextError.resources("\(name)'s functions") }
        let descriptor = MTLRenderPipelineDescriptor()
        descriptor.vertexFunction = vertex
        descriptor.fragmentFunction = fragment
        guard let color = descriptor.colorAttachments[0] else { throw TextError.resources("attachment") }
        color.pixelFormat = outputFormat
        color.isBlendingEnabled = true
        color.sourceRGBBlendFactor = .one
        color.destinationRGBBlendFactor = .oneMinusSourceAlpha
        color.sourceAlphaBlendFactor = .one
        color.destinationAlphaBlendFactor = .oneMinusSourceAlpha
        return try device.makeRenderPipelineState(descriptor: descriptor)
    }

    /// `face` at `pixelSize`, rasterised the first time it is asked for.
    public func atlas(_ face: MenuFace, pixelSize: Int) throws -> MenuGlyphs {
        if let atlas = atlases[face]?[pixelSize] { return atlas }
        guard let url = urls[face] else { throw TextError.font("no \(MenuFonts.file(face))") }
        let atlas = try MenuGlyphs(url: url, pixelSize: pixelSize, device: device)
        atlases[face, default: [:]][pixelSize] = atlas
        return atlas
    }

    /// The scene's shapes in a target `width` by `height`.
    func shapeInstances(_ scene: MenuScene, width: Int, height: Int) -> [MenuShapeInstance] {
        let (scale, left, top) = MenuDesign.fit(width: width, height: height)
        return scene.shapes.compactMap { shape in
            guard shape.rect.count == 4, shape.rect[2] > 0, shape.rect[3] > 0 else { return nil }
            let x = left + shape.rect[0] * scale, y = top + shape.rect[1] * scale
            return MenuShapeInstance(
                rect: SIMD4(x, y, x + shape.rect[2] * scale, y + shape.rect[3] * scale),
                radii: Self.vector(shape.radii) * scale, fill: Self.vector(shape.fill),
                borderColor: Self.vector(shape.borderColor),
                border: SIMD4(shape.border > 0 ? max(1, shape.border * scale) : 0, shape.additive ? 1 : 0, 0, 0))
        }
    }

    /// The scene's glyphs in a target `width` by `height`: each text's
    /// glow under its ink.
    func glyphInstances(_ scene: MenuScene, width: Int, height: Int) throws -> [(MenuGlyphs, [MenuGlyphInstance])] {
        let (scale, left, top) = MenuDesign.fit(width: width, height: height)
        var out: [(MenuGlyphs, [MenuGlyphInstance])] = []
        for text in scene.texts {
            let pixelSize = Int((text.size * scale).rounded())
            guard pixelSize > 0, !text.text.isEmpty else { continue }
            let atlas = try atlas(text.face, pixelSize: pixelSize)
            let lineWidth = atlas.width(text.text)
            var pen = left + text.x * scale
            switch text.align {
            case .left: break
            case .center: pen -= lineWidth / 2
            case .right: pen -= lineWidth
            }
            pen = pen.rounded()
            // (the line is `size` tall, its font's height centred in it, as
            // CSS lays a line out whose line height is its size)
            let lineTop = top + text.y * scale
            let baseline = (lineTop + (Float(pixelSize) - atlas.ascent - atlas.descent) / 2 + atlas.ascent).rounded()
            let (textureWidth, textureHeight) = (Float(atlas.texture.width), Float(atlas.texture.height))
            var ink: [MenuGlyphInstance] = [], glow: [MenuGlyphInstance] = []
            let glows = text.glow.count == 4 && text.glow[3] > 0
            for scalar in text.text.unicodeScalars {
                guard let glyph = atlas.glyph(scalar) else { continue }
                defer { pen += glyph.advance }
                if scalar == " " { continue }
                let x = pen.rounded() + glyph.offset.x, y = baseline + glyph.offset.y
                let rect = SIMD4(x, y, x + glyph.box.z - glyph.box.x, y + glyph.box.w - glyph.box.y)
                let uv = SIMD4(
                    glyph.box.x / textureWidth, glyph.box.y / textureHeight, glyph.box.z / textureWidth,
                    glyph.box.w / textureHeight)
                ink.append(MenuGlyphInstance(rect: rect, uv: uv, color: Self.vector(text.color), channel: .zero))
                if glows {
                    glow.append(
                        MenuGlyphInstance(rect: rect, uv: uv, color: Self.vector(text.glow), channel: SIMD4(1, 0, 0, 0))
                    )
                }
            }
            if !glow.isEmpty { out.append((atlas, glow)) }
            if !ink.isEmpty { out.append((atlas, ink)) }
        }
        return out
    }

    private static func vector(_ values: [Float]) -> SIMD4<Float> {
        values.count == 4 ? SIMD4(values[0], values[1], values[2], values[3]) : .zero
    }

    /// Draws `scene` over `target`, keeping what is there; nothing is
    /// encoded for an empty scene.
    public func encode(_ scene: MenuScene, into commands: MTLCommandBuffer, target: MTLTexture) throws {
        lastDrawn = (0, 0)
        guard !scene.shapes.isEmpty || !scene.texts.isEmpty else { return }
        let shapeQuads = shapeInstances(scene, width: target.width, height: target.height)
        let glyphRuns = try glyphInstances(scene, width: target.width, height: target.height)
        let glyphCount = glyphRuns.reduce(0) { $0 + $1.1.count }
        guard !shapeQuads.isEmpty || glyphCount > 0 else { return }

        let shapeBytes = shapeQuads.count * MemoryLayout<MenuShapeInstance>.stride
        let glyphBytes = glyphCount * MemoryLayout<MenuGlyphInstance>.stride
        frame = (frame + 1) % buffers.count
        // (a buffer whose commands have yet to finish is still being read:
        // the frame gets a new one rather than writing over a frame the
        // GPU has not drawn, as it would when the game runs ahead of it)
        let status = readers[frame]?.status
        let busy = status != nil && status != .completed && status != .error
        if busy || (buffers[frame]?.length ?? 0) < shapeBytes + glyphBytes {
            buffers[frame] = device.makeBuffer(
                length: max(shapeBytes + glyphBytes, 64 * 1024), options: .storageModeShared)
        }
        guard let buffer = buffers[frame] else { throw TextError.resources("the menus' buffer") }
        readers[frame] = commands
        shapeQuads.withUnsafeBytes { raw in
            if let base = raw.baseAddress { buffer.contents().copyMemory(from: base, byteCount: raw.count) }
        }
        var offsets: [Int] = []
        var offset = shapeBytes
        for (_, quads) in glyphRuns {
            offsets.append(offset)
            quads.withUnsafeBytes { raw in
                if let base = raw.baseAddress {
                    (buffer.contents() + offset).copyMemory(from: base, byteCount: raw.count)
                }
            }
            offset += quads.count * MemoryLayout<MenuGlyphInstance>.stride
        }

        let pass = MTLRenderPassDescriptor()
        pass.colorAttachments[0].texture = target
        pass.colorAttachments[0].loadAction = .load
        pass.colorAttachments[0].storeAction = .store
        guard let encoder = commands.timedRenderEncoder("menus", descriptor: pass) else { return }
        var size = SIMD2(Float(target.width), Float(target.height))
        if !shapeQuads.isEmpty {
            encoder.setRenderPipelineState(shapes)
            encoder.setVertexBuffer(buffer, offset: 0, index: 0)
            encoder.setFragmentBuffer(buffer, offset: 0, index: 0)
            encoder.setVertexBytes(&size, length: MemoryLayout<SIMD2<Float>>.size, index: 1)
            encoder.drawPrimitives(type: .triangle, vertexStart: 0, vertexCount: 6, instanceCount: shapeQuads.count)
        }
        if glyphCount > 0 {
            encoder.setRenderPipelineState(glyphs)
            encoder.setVertexBytes(&size, length: MemoryLayout<SIMD2<Float>>.size, index: 1)
            encoder.setFragmentSamplerState(sampler, index: 0)
            for (index, (atlas, quads)) in glyphRuns.enumerated() {
                encoder.setVertexBuffer(buffer, offset: offsets[index], index: 0)
                encoder.setFragmentTexture(atlas.texture, index: 0)
                encoder.drawPrimitives(type: .triangle, vertexStart: 0, vertexCount: 6, instanceCount: quads.count)
            }
        }
        encoder.endEncoding()
        lastDrawn = (shapeQuads.count, glyphCount)
    }
}
