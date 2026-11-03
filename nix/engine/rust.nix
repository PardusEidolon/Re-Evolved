# The engine's Rust workspace (engine/) for crane: its source, and its
# dependencies built once for the packages and the checks
{ inputs, cell }:
let
  pkgs = inputs.cells.repo.pkgs;
  inherit (inputs.cells.repo.toolchain) root craneLib version;
  inherit (pkgs) lib;
  fs = lib.fileset;

  # crane vendors each crate as a link into the store, and askama (in
  # uniffi_bindgen) reaches its askama.toml by a path made relative to the
  # link's real place, which then climbs out of the store; uniffi_bindgen
  # is copied in as a directory, the others stay links
  vendored = craneLib.vendorCargoDeps { cargoLock = root + /engine/Cargo.lock; };
  cargoVendorDir = pkgs.runCommand "opence-engine-vendor" { } ''
    mkdir $out
    for registry in ${vendored}/*/; do
      registry=''${registry%/}
      mkdir $out/$(basename $registry)
      for crate in $registry/*; do
        into=$out/$(basename $registry)/$(basename $crate)
        case $(basename $crate) in
          uniffi_bindgen-*) cp -R $(readlink -f $crate) $into ;;
          *) ln -s $(readlink -f $crate) $into ;;
        esac
      done
    done
    substitute ${vendored}/config.toml $out/config.toml --replace-fail ${vendored} $out
  '';
in
rec {
  # the workspace, without the Swift app, the census's manifest and the
  # fuzzing workspace (but with its manifest and targets, which the
  # importer's fuzz smoke test reads), and what its builds read outside
  # it: the reference traces (the tests' include_str!), the render batch's
  # header (the render crate's tests' include_str!) and the build script
  commonFiles = fs.unions [
    (fs.difference (root + /engine) (
      fs.unions [
        (root + /engine/app)
        (root + /engine/census)
        (root + /engine/fuzz)
      ]
    ))
    (root + /engine/fuzz/Cargo.toml)
    (root + /engine/fuzz/fuzz_targets)
    (root + /tools/traces)
    (root + /port/wasmtime/include/opence_render.h)
    (root + /tools/engine_build.py)
  ];
  commonArgs = {
    pname = "opence-engine-rust";
    inherit version;
    src = fs.toSource {
      inherit root;
      fileset = commonFiles;
    };
    cargoLock = root + /engine/Cargo.lock;
    cargoToml = root + /engine/Cargo.toml;
    inherit cargoVendorDir;
    # (the workspace is engine/, not the source's root)
    postUnpack = ''
      cd $sourceRoot/engine
      sourceRoot="."
    '';
    strictDeps = true;
    buildInputs = [ pkgs.apple-sdk_26 ];
  };

  # the tests' source: the workspace's, and what the census's tests read
  # (engine/crates/census): the original's C and headers, the app's Swift,
  # the census's manifest and the docs it writes, and what game.wasm
  # compiles (port/linux/port.json) with the stand-in the manifest names;
  # only the tests check is built from it, so the
  # packages do not build again when these change
  testArgs = commonArgs // {
    src = fs.toSource {
      inherit root;
      fileset = fs.unions [
        commonFiles
        (fs.fileFilter (file: file.hasExt "c" || file.hasExt "h") (root + /source))
        (root + /engine/app/Sources)
        (root + /engine/census)
        (root + /docs)
        (root + /port/linux/port.json)
        (root + /port/linux/game/tiff_file.c)
      ];
    };
  };

  # every dependency, for the library, the bindings' generator (the
  # bindgen feature) and the tests; its version is fixed, since the
  # commit's would rename it, and so build it again, at every commit
  cargoArtifacts = craneLib.buildDepsOnly (
    commonArgs
    // {
      version = "locked";
      buildPhaseCargoCommand = ''
        cargoWithProfile check --locked --workspace --all-targets
        cargoWithProfile build --locked --workspace
        cargoWithProfile build --locked -p opence_ffi --features bindgen --bin uniffi-bindgen
        cargoWithProfile test --locked --workspace --no-run
      '';
    }
  );

  # the tests' dependencies, in the profile the suite tests in (test-opt:
  # optimised, with debug assertions and overflow checks, which the sim's
  # checks tests look for)
  testArtifacts = craneLib.buildDepsOnly (
    commonArgs
    // {
      version = "locked";
      CARGO_PROFILE = "test-opt";
      buildPhaseCargoCommand = ''
        cargoWithProfile test --locked --workspace --no-run
      '';
    }
  );
}
