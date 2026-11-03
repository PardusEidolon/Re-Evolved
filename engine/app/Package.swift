// swift-tools-version: 6.2
// OpenCE Engine.app: the new engine's app (conductor/archive/native_engine_20261021).
// tools/engine_build.py builds the Rust library (engine/crates/ffi) first,
// writes its UniFFI bindings and the snapshot's header into the two
// generated folders, and links the library by passing its path.
import PackageDescription

let package = Package(
    name: "OpenCEEngine",
    platforms: [.macOS(.v26)],
    targets: [
        // the crates' C declarations: UniFFI's (generated)
        .systemLibrary(name: "opence_ffiFFI", path: "Sources/opence_ffiFFI"),
        // the ffi crate's Swift bindings (generated)
        .target(name: "OpenCEEngineCore", dependencies: ["opence_ffiFFI"],
                // (as uniffi-bindgen writes it, for Swift 5's checks)
                swiftSettings: [.swiftLanguageMode(.v5)]),
        // the app's side of the contract: the game's input and sound, and the user's maps
        .target(name: "OpenCEEngineKit", dependencies: ["OpenCEEngineCore", "OpenCEEngineRender", "opence_ffiFFI"]),
        // the window: its Metal layer, display link, mouse capture and keys
        .target(name: "OpenCEEngineWindow"),
        // the renderer: game.wasm's render batches on Metal, and the benchmark
        .target(name: "OpenCEEngineRender", dependencies: ["OpenCEEngineCore", "opence_ffiFFI"]),
        .executableTarget(name: "OpenCEEngine",
                          dependencies: ["OpenCEEngineKit", "OpenCEEngineWindow", "OpenCEEngineRender",
                                         "opence_ffiFFI"]),
        .testTarget(name: "OpenCEEngineTests",
                    dependencies: ["OpenCEEngineKit", "OpenCEEngineCore", "opence_ffiFFI", "OpenCEEngineWindow",
                                   "OpenCEEngineRender"]),
    ]
)
