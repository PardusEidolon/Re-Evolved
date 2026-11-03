//! The network imports the platform answers (`opence_runtime::net`): the
//! app has no system link, so the network starts and the title's address
//! is plainly absent (`XNET_GET_XNADDR_NONE`), the link is down, and the
//! transport gives up at once instead of waiting out its ten seconds.

use opence_runtime::{answers, Error, Runtime, Val};

fn module(text: &str) -> Vec<u8> {
    wat::parse_str(text).unwrap_or_default()
}

/// the module: each import re-exported as a function the test calls
const NET: &str = r#"
(module
  (import "env" "WSAStartup" (func $startup (param i32 i32) (result i32)))
  (import "env" "WSACleanup" (func $cleanup (result i32)))
  (import "env" "WSAGetLastError" (func $error (result i32)))
  (import "env" "XNetStartup" (func $xstartup (param i32) (result i32)))
  (import "env" "XNetCleanup" (func $xcleanup (result i32)))
  (import "env" "XNetGetTitleXnAddr" (func $address (param i32) (result i32)))
  (import "env" "XNetGetEthernetLinkStatus" (func $link (result i32)))
  (import "env" "XNetRandom" (func $random (param i32 i32) (result i32)))
  (import "env" "XNetCreateKey" (func $create (param i32 i32) (result i32)))
  (import "env" "XNetRegisterKey" (func $register (param i32 i32) (result i32)))
  (import "env" "XNetUnregisterKey" (func $unregister (param i32) (result i32)))
  (import "env" "XNetXnAddrToInAddr" (func $to_in (param i32 i32 i32) (result i32)))
  (import "env" "halo_ws_socket" (func $socket (param i32 i32 i32) (result i32)))
  (memory (export "memory") 1)
  (func (export "startup") (param i32 i32) (result i32) (call $startup (local.get 0) (local.get 1)))
  (func (export "cleanup") (result i32) (call $cleanup))
  (func (export "error") (result i32) (call $error))
  (func (export "xstartup") (param i32) (result i32) (call $xstartup (local.get 0)))
  (func (export "xcleanup") (result i32) (call $xcleanup))
  (func (export "address") (param i32) (result i32) (call $address (local.get 0)))
  (func (export "link") (result i32) (call $link))
  (func (export "random") (param i32 i32) (result i32) (call $random (local.get 0) (local.get 1)))
  (func (export "create") (param i32 i32) (result i32) (call $create (local.get 0) (local.get 1)))
  (func (export "register") (param i32 i32) (result i32) (call $register (local.get 0) (local.get 1)))
  (func (export "unregister") (param i32) (result i32) (call $unregister (local.get 0)))
  (func (export "to_in") (param i32 i32 i32) (result i32)
    (call $to_in (local.get 0) (local.get 1) (local.get 2)))
  (func (export "socket") (param i32 i32 i32) (result i32)
    (call $socket (local.get 0) (local.get 1) (local.get 2)))
  (data (i32.const 0x100) "\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff")
  (data (i32.const 0x200) "\0c\00\01\02\03\04\05\06\0a\00\00\07"))
"#;

/// XNET_GET_XNADDR_NONE (`port/include/xdk/xdk_winsock.h`)
const NONE: i32 = 1;
/// the size of XNADDR and of WSADATA (`port/include/xdk/xdk_pdb.h`)
const XNADDR: u64 = 12;
const WSADATA: u64 = 400;
/// INVALID_SOCKET and WSAENETDOWN (`port/include/xdk/xdk_winsock.h`)
const INVALID_SOCKET: i32 = -1;
const NETWORK_DOWN: i32 = 10050;

fn first(results: &[Val]) -> Option<i32> {
    results.first().and_then(Val::i32)
}

#[test]
fn the_title_has_no_address_and_the_link_is_down() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(NET))?;
    assert_eq!(first(&game.call_with("xstartup", &[Val::I32(0)])?), Some(0));
    assert_eq!(
        first(&game.call_with("address", &[Val::I32(0x100)])?),
        Some(NONE)
    );
    // the address is cleared but for its size
    let mut cleared = vec![0; 12];
    cleared[0] = 12;
    assert_eq!(game.read(0x100, XNADDR)?, cleared);
    assert_eq!(first(&game.call_with("link", &[])?), Some(0));
    assert_eq!(first(&game.call_with("xcleanup", &[])?), Some(0));
    Ok(())
}

#[test]
fn winsock_starts_with_its_version_and_limits() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(NET))?;
    assert_eq!(
        first(&game.call_with("startup", &[Val::I32(0x0202), Val::I32(0x400)])?),
        Some(0)
    );
    let data = game.read(0x400, WSADATA)?;
    assert_eq!(data[0..2], [2, 2]);
    assert_eq!(data[2..4], [2, 2]);
    // iMaxSockets and iMaxUdpDg follow the two strings (2 + 2 + 257 + 129)
    assert_eq!(data[390..392], 64_u16.to_le_bytes());
    assert_eq!(data[392..394], 1264_u16.to_le_bytes());
    assert_eq!(first(&game.call_with("cleanup", &[])?), Some(0));
    // a null WSADATA is not written
    assert_eq!(
        first(&game.call_with("startup", &[Val::I32(0x0202), Val::I32(0)])?),
        Some(0)
    );
    Ok(())
}

#[test]
fn a_socket_cannot_be_made_with_the_network_down() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(NET))?;
    let made = game.call_with("socket", &[Val::I32(2), Val::I32(2), Val::I32(17)])?;
    assert_eq!(first(&made), Some(INVALID_SOCKET));
    assert_eq!(first(&game.call_with("error", &[])?), Some(NETWORK_DOWN));
    Ok(())
}

#[test]
fn keys_and_random_bytes_are_made_and_an_address_converts() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(NET))?;
    assert_eq!(
        first(&game.call_with("random", &[Val::I32(0x600), Val::I32(32)])?),
        Some(0)
    );
    assert_ne!(game.read(0x600, 32)?, vec![0; 32]);
    // XNKID is 8 bytes, XNKEY 16
    assert_eq!(
        first(&game.call_with("create", &[Val::I32(0x700), Val::I32(0x708)])?),
        Some(0)
    );
    assert_ne!(game.read(0x700, 24)?, vec![0; 24]);
    assert_eq!(
        first(&game.call_with("register", &[Val::I32(0x700), Val::I32(0x708)])?),
        Some(0)
    );
    assert_eq!(
        first(&game.call_with("unregister", &[Val::I32(0x700)])?),
        Some(0)
    );
    // the address's ina, at 8 in XNADDR
    assert_eq!(
        first(&game.call_with(
            "to_in",
            &[Val::I32(0x200), Val::I32(0x700), Val::I32(0x800)]
        )?),
        Some(0)
    );
    assert_eq!(game.read(0x800, 4)?, vec![0x0a, 0, 0, 7]);
    Ok(())
}

#[test]
fn the_platform_answers_the_network_s_imports() {
    for name in [
        "env.WSAStartup",
        "env.WSACleanup",
        "env.WSAGetLastError",
        "env.XNetStartup",
        "env.XNetCleanup",
        "env.XNetGetTitleXnAddr",
        "env.XNetGetEthernetLinkStatus",
        "env.XNetRandom",
        "env.XNetCreateKey",
        "env.XNetRegisterKey",
        "env.XNetUnregisterKey",
        "env.XNetXnAddrToInAddr",
        "env.halo_ws_socket",
    ] {
        assert!(answers(name), "{name}");
    }
}
