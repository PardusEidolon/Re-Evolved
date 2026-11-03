# The compilers, tools and libraries the builds and the shell share
{ inputs, cell }:
let
  pkgs = cell.pkgs;
  # the repository as a path. std copies each cell into the store alone,
  # so a path such as ../.. would leave the repository; and a path, not
  # "${inputs.self}/...", copies only the file named (a Cargo.lock's
  # vendored crates then build again only when it changes)
  root = /. + builtins.unsafeDiscardStringContext inputs.self.outPath;
  # clang 22: the guest's compiler (its arm64_32 target), and the
  # version the profiles in pgo/ were recorded with
  llvm = pkgs.llvmPackages_22;
  # wasm32-wasip1's C library, for the reference harnesses, and the same
  # library built for wasm32-wasip1-threads (atomics, a shared memory and
  # wasi-libc's pthreads), for game.wasm
  wasi = pkgs.pkgsCross.wasi32;
  wasilibcThreads = wasi.wasilibc.overrideAttrs (old: {
    pname = "wasilibc-threads";
    cmakeFlags = old.cmakeFlags ++ [ "-DTARGET_TRIPLE=wasm32-wasip1-threads" ];
  });
  rust = pkgs.rust-bin.fromRustupToolchainFile (root + /rust-toolchain.toml);
  python = pkgs.python3.withPackages (ps: [
    ps.pytest
    ps.pyflakes
  ]);
in
{
  inherit
    root
    llvm
    rust
    python
    ;

  version = inputs.self.shortRev or inputs.self.dirtyShortRev or "dirty";

  # what tools/engine_wasm.py takes from the environment
  buildEnvironment = {
    # (GLES2, GLES3 and KHR: the OpenGL ES declarations the shader
    # translators compile with, and SDL3 3.4.16's headers, which the
    # platform layer's units include: as tools/android_build.py's SDL_TAG)
    HALO_WASM_GLES_INCLUDE = "${pkgs.libglvnd.dev}/include";
    HALO_WASM_SDL_INCLUDE = "${pkgs.sdl3.dev}/include";
    # game.wasm (tools/engine_wasm.py): the unwrapped clang, for its wasm32
    # target, wasi-libc for wasm32-wasip1-threads (the game) and for
    # wasm32-wasip1 (the reference harnesses)
    HALO_WASM_CC = "${llvm.clang-unwrapped}/bin/clang";
    HALO_WASI_LIBC_THREADS = "${wasilibcThreads}";
    HALO_WASI_LIBC_THREADS_DEV = "${wasilibcThreads.dev}";
    HALO_WASI_LIBC = "${wasi.wasilibc}";
    HALO_WASI_LIBC_DEV = "${wasi.wasilibc.dev}";
    # (compiler-rt's builtins for wasm32, which the unwrapped clang's
    # resource directory does not have)
    HALO_WASM_BUILTINS = "${wasi.llvmPackages_22.compiler-rt}/lib/wasip1/libclang_rt.builtins-wasm32.a";
  };

  tools = [
    python
    pkgs.ninja
    llvm.lld
    llvm.llvm
    rust
    pkgs.cargo-llvm-cov
    pkgs.cargo-nextest
    pkgs.cargo-mutants
    pkgs.cargo-insta
    pkgs.cargo-deny
    # (`nom build`, `nom develop`: nix's builds, as a tree)
    pkgs.nix-output-monitor
  ];
  libraries = [
    pkgs.sdl3
    pkgs.apple-sdk_26
  ];

  # crane with the repository's Rust (the engine's packages and checks)
  craneLib = (inputs.crane.mkLib pkgs).overrideToolchain (_: rust);
}
