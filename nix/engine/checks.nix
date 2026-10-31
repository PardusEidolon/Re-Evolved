# `nix flake check`: the workflow's Rust checks (conductor/workflow.md),
# on the dependencies engine-rust builds with. The tests that need the
# player's maps or default.xbe skip themselves here.
{ inputs, cell }:
let
  inherit (inputs.cells.repo.toolchain) craneLib root python;
  pkgs = inputs.cells.repo.pkgs;
  fs = pkgs.lib.fileset;
  inherit (cell.rust)
    commonArgs
    testArgs
    cargoArtifacts
    testArtifacts
    ;
in
{
  fmt = craneLib.cargoFmt commonArgs;
  clippy = craneLib.cargoClippy (
    commonArgs
    // {
      inherit cargoArtifacts;
      cargoClippyExtraArgs = "--workspace -- --deny warnings";
    }
  );
  # (in test-opt, as tools/engine_build.py tests)
  tests = craneLib.cargoNextest (
    testArgs
    // {
      cargoArtifacts = testArtifacts;
      CARGO_PROFILE = "test-opt";
      cargoNextestExtraArgs = "--workspace";
    }
  );
  # bans, licences and sources (engine/deny.toml); the advisories need
  # the network
  deny = craneLib.cargoDeny commonArgs;
  # the workspace's coverage (lcov), the tests run as `tests` runs them
  # (from the tests' source, which has the census's manifest and the
  # files its tests read)
  coverage = craneLib.cargoLlvmCov (
    testArgs
    // {
      cargoArtifacts = testArtifacts;
      CARGO_PROFILE = "test-opt";
      # (`cargo llvm-cov test`: nextest would read `--profile` as its own)
      cargoLlvmCovExtraArgs = "--workspace --lcov --output-path $out";
    }
  );
  # the menus' functions that do nothing (port/assets/menus/UNWIRED.md),
  # current with the screens and menu_functions.c
  menus =
    pkgs.runCommand "opence-menus-check"
      {
        src = fs.toSource {
          inherit root;
          fileset = fs.unions [
            (root + /tools/menu_functions_check.py)
            (root + /tools/test_menu_functions_check.py)
            (root + /port/assets/menus/ce)
            (root + /port/assets/menus/UNWIRED.md)
            (root + /port/linux/game/menu_functions.c)
            (root + /port/linux/game/menu_tags.c)
            (root + /source/interface/ui_widget_event_handler_functions.c)
          ];
        };
        nativeBuildInputs = [ python ];
      }
      ''
        cd $src
        python3 tools/menu_functions_check.py --check
        python3 -m pytest -q -p no:cacheprovider tools/test_menu_functions_check.py
        touch $out
      '';
  # the tree formatted as `nix fmt` formats it (nix/repo/formatter.nix)
  treefmt = inputs.cells.repo.formatter.check;
}
