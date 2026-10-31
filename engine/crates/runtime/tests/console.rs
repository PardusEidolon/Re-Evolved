//! The game's telnet console as the dev shell reaches it
//! (`opence_runtime::Console`): with `debug.telnet_console` on, the title
//! has a loopback address, the console's stream socket binds and listens
//! on 127.0.0.1, and the host connects to it in process, handing it lines
//! and taking what the game writes back. With it off the network stays
//! as the cable-out Xbox has it.

use std::time::{Duration, Instant};

use opence_runtime::{answers, Error, Runtime, Val};

fn module(text: &str) -> Vec<u8> {
    wat::parse_str(text).unwrap_or_default()
}

/// the module: each import re-exported as a function the test calls
const CONSOLE: &str = r#"
(module
  (import "env" "WSAGetLastError" (func $error (result i32)))
  (import "env" "XNetGetTitleXnAddr" (func $address (param i32) (result i32)))
  (import "env" "XNetGetEthernetLinkStatus" (func $link (result i32)))
  (import "env" "halo_ws_socket" (func $socket (param i32 i32 i32) (result i32)))
  (import "env" "halo_ws_setsockopt" (func $setsockopt (param i32 i32 i32 i32 i32) (result i32)))
  (import "env" "halo_ws_bind" (func $bind (param i32 i32 i32) (result i32)))
  (import "env" "halo_ws_listen" (func $listen (param i32 i32) (result i32)))
  (import "env" "halo_ws_ioctlsocket" (func $ioctl (param i32 i32 i32) (result i32)))
  (import "env" "halo_ws_select" (func $select (param i32 i32 i32 i32 i32) (result i32)))
  (import "env" "halo_ws_accept" (func $accept (param i32 i32 i32) (result i32)))
  (import "env" "halo_ws_recv" (func $recv (param i32 i32 i32 i32) (result i32)))
  (import "env" "halo_ws_send" (func $send (param i32 i32 i32 i32) (result i32)))
  (import "env" "halo_ws_getsockname" (func $name (param i32 i32 i32) (result i32)))
  (import "env" "halo_ws_closesocket" (func $close (param i32) (result i32)))
  (memory (export "memory") 1)
  (func (export "error") (result i32) (call $error))
  (func (export "address") (param i32) (result i32) (call $address (local.get 0)))
  (func (export "link") (result i32) (call $link))
  (func (export "socket") (param i32 i32 i32) (result i32)
    (call $socket (local.get 0) (local.get 1) (local.get 2)))
  (func (export "setsockopt") (param i32 i32 i32 i32 i32) (result i32)
    (call $setsockopt (local.get 0) (local.get 1) (local.get 2) (local.get 3) (local.get 4)))
  (func (export "bind") (param i32 i32 i32) (result i32)
    (call $bind (local.get 0) (local.get 1) (local.get 2)))
  (func (export "listen") (param i32 i32) (result i32) (call $listen (local.get 0) (local.get 1)))
  (func (export "ioctl") (param i32 i32 i32) (result i32)
    (call $ioctl (local.get 0) (local.get 1) (local.get 2)))
  (func (export "select") (param i32 i32 i32 i32 i32) (result i32)
    (call $select (local.get 0) (local.get 1) (local.get 2) (local.get 3) (local.get 4)))
  (func (export "accept") (param i32 i32 i32) (result i32)
    (call $accept (local.get 0) (local.get 1) (local.get 2)))
  (func (export "recv") (param i32 i32 i32 i32) (result i32)
    (call $recv (local.get 0) (local.get 1) (local.get 2) (local.get 3)))
  (func (export "send") (param i32 i32 i32 i32) (result i32)
    (call $send (local.get 0) (local.get 1) (local.get 2) (local.get 3)))
  (func (export "name") (param i32 i32 i32) (result i32)
    (call $name (local.get 0) (local.get 1) (local.get 2)))
  (func (export "close") (param i32) (result i32) (call $close (local.get 0)))
  ;; sockaddr_in: AF_INET, port 2323 and 127.0.0.1 in network order
  (data (i32.const 0x300) "\02\00\09\13\7f\00\00\01\00\00\00\00\00\00\00\00")
  ;; and the same port on 10.0.0.7, an address the host does not have
  (data (i32.const 0x320) "\02\00\09\13\0a\00\00\07\00\00\00\00\00\00\00\00")
  ;; FIONBIO's argument, 1
  (data (i32.const 0x400) "\01\00\00\00")
  ;; accept's and getsockname's lengths
  (data (i32.const 0x710) "\10\00\00\00")
  (data (i32.const 0x730) "\10\00\00\00")
  (data (i32.const 0x800) "hello")
  ;; the console's greeting
  (data (i32.const 0x840) "Would you like to play a game?\0d\0a"))
"#;

/// XNET_GET_XNADDR_NONE, and ETHERNET | DHCP (`port/include/xdk/xdk_winsock.h`)
const NONE: i32 = 1;
const ETHERNET_DHCP: i32 = 0x02 | 0x08;
/// XNET_ETHERNET_LINK_ACTIVE | 100MBPS | FULL_DUPLEX
const LINK_UP: i32 = 0x01 | 0x02 | 0x08;
/// AF_INET, SOCK_STREAM, SOCK_DGRAM and IPPROTO_UDP
const INET: i32 = 2;
const STREAM: i32 = 1;
const DATAGRAM: i32 = 2;
const UDP: i32 = 17;
/// FIONBIO, as the game passes it
const FIONBIO: i32 = 0x8004_667E_u32 as i32;
/// INVALID_SOCKET and SOCKET_ERROR
const INVALID_SOCKET: i32 = -1;
const SOCKET_ERROR: i32 = -1;
/// WSAEWOULDBLOCK, WSAEADDRNOTAVAIL and WSAENETDOWN
const WOULD_BLOCK: i32 = 10035;
const ADDRESS_NOT_AVAILABLE: i32 = 10049;
const NETWORK_DOWN: i32 = 10050;
/// where the test keeps a set, its timeout and a receive buffer
const SET: i32 = 0x500;
const TIMEOUT: i32 = 0x600;
const BUFFER: i32 = 0x900;

fn first(results: &[Val]) -> Option<i32> {
    results.first().and_then(Val::i32)
}

/// calls `name` with `args`, all i32
fn call(game: &mut opence_runtime::Game, name: &str, args: &[i32]) -> Result<i32, Error> {
    let args: Vec<Val> = args.iter().map(|&arg| Val::I32(arg)).collect();
    Ok(first(&game.call_with(name, &args)?).unwrap_or(i32::MIN))
}

/// writes a set of `sockets`
fn put_set(game: &mut opence_runtime::Game, sockets: &[i32]) -> Result<(), Error> {
    let mut bytes = u32::try_from(sockets.len())
        .unwrap_or(0)
        .to_le_bytes()
        .to_vec();
    for socket in sockets {
        bytes.extend_from_slice(&socket.to_le_bytes());
    }
    game.write(SET as u64, &bytes)
}

/// the set at SET, as select left it
fn set(game: &opence_runtime::Game) -> Result<Vec<i32>, Error> {
    let bytes = game.read(SET as u64, 4 + 4 * 64)?;
    let count = bytes
        .get(..4)
        .and_then(|count| count.try_into().ok())
        .map_or(0, u32::from_le_bytes) as usize;
    Ok(bytes[4..]
        .as_chunks::<4>()
        .0
        .iter()
        .take(count)
        .map(|socket| i32::from_le_bytes(*socket))
        .collect())
}

/// whether select finds `socket` readable, as `endpoint_readable` asks
fn readable(game: &mut opence_runtime::Game, socket: i32) -> Result<bool, Error> {
    put_set(game, &[socket])?;
    let count = call(game, "select", &[1, SET, 0, 0, TIMEOUT])?;
    Ok(count > 0 && set(game)? == [socket])
}

/// a game with the console on, listening on 127.0.0.1:2323
fn listening() -> Result<(opence_runtime::Game, i32), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(CONSOLE))?;
    assert!(game.set_setting("debug.telnet_console", "true"));
    let listener = call(&mut game, "socket", &[INET, STREAM, 0])?;
    assert_ne!(listener, INVALID_SOCKET);
    assert_eq!(
        call(&mut game, "setsockopt", &[listener, 0xFFFF, 4, 0x400, 4])?,
        0
    );
    assert_eq!(call(&mut game, "bind", &[listener, 0x300, 16])?, 0);
    assert_eq!(call(&mut game, "listen", &[listener, 5])?, 0);
    Ok((game, listener))
}

#[test]
fn with_the_console_off_the_cable_stays_out() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(CONSOLE))?;
    assert_eq!(call(&mut game, "address", &[0x100])?, NONE);
    assert_eq!(call(&mut game, "link", &[])?, 0);
    assert_eq!(
        call(&mut game, "socket", &[INET, STREAM, 0])?,
        INVALID_SOCKET
    );
    assert_eq!(call(&mut game, "error", &[])?, NETWORK_DOWN);
    // and the host finds nothing to connect to
    assert!(!game.console().connect());
    Ok(())
}

#[test]
fn with_the_console_on_the_title_has_a_loopback_address() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(CONSOLE))?;
    assert!(game.set_setting("debug.telnet_console", "true"));
    assert_eq!(call(&mut game, "address", &[0x100])?, ETHERNET_DHCP);
    let address = game.read(0x100, 12)?;
    assert_eq!(address[0], 12);
    assert_eq!(address[8..12], [127, 0, 0, 1]);
    assert_eq!(call(&mut game, "link", &[])?, LINK_UP);
    // a setting the game does not have is not set
    assert!(!game.set_setting("debug.no_such_setting", "true"));
    Ok(())
}

#[test]
fn only_the_console_s_stream_on_loopback_is_made() -> Result<(), Error> {
    let (mut game, listener) = listening()?;
    // datagrams (system link) still have no network
    assert_eq!(
        call(&mut game, "socket", &[INET, DATAGRAM, UDP])?,
        INVALID_SOCKET
    );
    assert_eq!(call(&mut game, "error", &[])?, NETWORK_DOWN);
    // and a stream binds only to the loopback address
    let other = call(&mut game, "socket", &[INET, STREAM, 0])?;
    assert_ne!(other, INVALID_SOCKET);
    assert_ne!(other, listener);
    assert_eq!(call(&mut game, "bind", &[other, 0x320, 16])?, SOCKET_ERROR);
    assert_eq!(call(&mut game, "error", &[])?, ADDRESS_NOT_AVAILABLE);
    assert_eq!(call(&mut game, "close", &[other])?, 0);
    // the listener's own name is where it was bound
    assert_eq!(call(&mut game, "name", &[listener, 0x720, 0x730])?, 0);
    assert_eq!(game.read(0x720, 8)?, [2, 0, 0x09, 0x13, 127, 0, 0, 1]);
    Ok(())
}

#[test]
fn the_host_connects_and_the_game_accepts_it() -> Result<(), Error> {
    let (mut game, listener) = listening()?;
    let console = game.console();
    assert!(!readable(&mut game, listener)?);
    assert_eq!(
        call(&mut game, "accept", &[listener, 0x700, 0x710])?,
        INVALID_SOCKET
    );
    assert_eq!(call(&mut game, "error", &[])?, WOULD_BLOCK);

    assert!(console.connect());
    assert!(console.connected());
    assert!(readable(&mut game, listener)?);
    let client = call(&mut game, "accept", &[listener, 0x700, 0x710])?;
    assert_ne!(client, INVALID_SOCKET);
    assert_ne!(client, listener);
    // the peer is the loopback address
    assert_eq!(game.read(0x700, 8)?[..2], [2, 0]);
    assert_eq!(game.read(0x704, 4)?, [127, 0, 0, 1]);
    assert_eq!(call(&mut game, "ioctl", &[client, FIONBIO, 0x400])?, 0);
    // the connection was taken, so the listener has no more
    assert!(!readable(&mut game, listener)?);
    Ok(())
}

#[test]
fn lines_go_both_ways_until_the_host_closes() -> Result<(), Error> {
    let (mut game, listener) = listening()?;
    let console = game.console();
    assert!(console.connect());
    let client = call(&mut game, "accept", &[listener, 0, 0])?;
    assert_ne!(client, INVALID_SOCKET);

    // nothing to read yet: the game is told to come back
    assert!(!readable(&mut game, client)?);
    assert_eq!(
        call(&mut game, "recv", &[client, BUFFER, 32, 0])?,
        SOCKET_ERROR
    );
    assert_eq!(call(&mut game, "error", &[])?, WOULD_BLOCK);

    // what the game sends, the host takes
    assert_eq!(call(&mut game, "send", &[client, 0x800, 5, 0])?, 5);
    assert_eq!(console.take_output(), "hello");
    assert_eq!(console.take_output(), "");

    // a line the host hands over reaches recv, its end as telnet's
    console.send_line("game_speed 2");
    assert!(readable(&mut game, client)?);
    assert_eq!(call(&mut game, "recv", &[client, BUFFER, 8, 0])?, 8);
    assert_eq!(game.read(BUFFER as u64, 8)?, b"game_spe");
    assert_eq!(call(&mut game, "recv", &[client, BUFFER, 32, 0])?, 6);
    assert_eq!(game.read(BUFFER as u64, 6)?, b"ed 2\r\n");

    // once the host closes, the client reads as readable and then as ended
    console.close();
    assert!(!console.connected());
    assert!(readable(&mut game, client)?);
    assert_eq!(call(&mut game, "recv", &[client, BUFFER, 32, 0])?, 0);
    assert_eq!(call(&mut game, "close", &[client])?, 0);
    Ok(())
}

#[test]
fn the_host_sees_the_game_close_its_end() -> Result<(), Error> {
    let (mut game, listener) = listening()?;
    let console = game.console();
    assert!(console.connect());
    let client = call(&mut game, "accept", &[listener, 0, 0])?;
    assert_eq!(call(&mut game, "send", &[client, 0x800, 5, 0])?, 5);
    assert_eq!(call(&mut game, "close", &[client])?, 0);
    // what was sent before the close is still there to take
    assert!(!console.connected());
    assert_eq!(console.take_output(), "hello");
    // a closed socket is no socket
    assert_eq!(
        call(&mut game, "recv", &[client, BUFFER, 32, 0])?,
        SOCKET_ERROR
    );
    // and with the listener closed there is nothing to connect to
    assert_eq!(call(&mut game, "close", &[listener])?, 0);
    assert!(!console.connect());
    Ok(())
}

/// the greeting's length, at 0x840
const GREETING: i32 = 32;

#[test]
fn asking_hands_the_line_over_and_waits_for_the_reply() -> Result<(), Error> {
    let (mut game, listener) = listening()?;
    let console = game.console();
    let asking = std::thread::spawn(move || console.ask("game_speed 2", Duration::from_secs(10)));
    // the game, a frame at a time: it accepts, greets, reads the line and
    // answers it
    let started = Instant::now();
    let mut client = INVALID_SOCKET;
    while client == INVALID_SOCKET && started.elapsed() < Duration::from_secs(10) {
        client = call(&mut game, "accept", &[listener, 0, 0])?;
        std::thread::sleep(Duration::from_millis(1));
    }
    assert_ne!(client, INVALID_SOCKET);
    assert_eq!(
        call(&mut game, "send", &[client, 0x840, GREETING, 0])?,
        GREETING
    );
    let mut line = Vec::new();
    while !line.ends_with(b"\r\n") && started.elapsed() < Duration::from_secs(10) {
        let count = call(&mut game, "recv", &[client, BUFFER, 32, 0])?;
        if count > 0 {
            line.extend(game.read(BUFFER as u64, count as u64)?);
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    assert_eq!(line, b"game_speed 2\r\n");
    assert_eq!(call(&mut game, "send", &[client, 0x800, 5, 0])?, 5);
    let reply = asking.join().ok().flatten();
    assert_eq!(reply.as_deref(), Some("hello"));
    Ok(())
}

#[test]
fn asking_with_the_console_off_has_no_answer() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let game = runtime.load(&module(CONSOLE))?;
    let started = Instant::now();
    assert_eq!(
        game.console().ask("game_speed 2", Duration::from_secs(10)),
        None
    );
    assert!(started.elapsed() < Duration::from_secs(1));
    Ok(())
}

/// a socket path of the test's own, short enough for sun_path
fn socket_path(name: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!("opence-{}-{name}.sock", std::process::id()))
}

/// the game's half of a console session: accepts, greets, reads a line
/// and answers it with `answer` (at `address`, `length` bytes)
fn answer_a_line(
    game: &mut opence_runtime::Game,
    listener: i32,
    address: i32,
    length: i32,
) -> Result<Vec<u8>, Error> {
    let started = Instant::now();
    let mut client = INVALID_SOCKET;
    while client == INVALID_SOCKET && started.elapsed() < Duration::from_secs(10) {
        client = call(game, "accept", &[listener, 0, 0])?;
        std::thread::sleep(Duration::from_millis(1));
    }
    assert_ne!(client, INVALID_SOCKET);
    assert_eq!(call(game, "send", &[client, 0x840, GREETING, 0])?, GREETING);
    let mut line = Vec::new();
    while !line.ends_with(b"\r\n") && started.elapsed() < Duration::from_secs(10) {
        let count = call(game, "recv", &[client, BUFFER, 32, 0])?;
        if count > 0 {
            line.extend(game.read(BUFFER as u64, count as u64)?);
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    assert_eq!(call(game, "send", &[client, address, length, 0])?, length);
    Ok(line)
}

#[test]
fn the_control_socket_runs_a_line_on_the_game_s_console() -> Result<(), Error> {
    use std::io::{BufRead, BufReader, Write};
    let (mut game, listener) = listening()?;
    let path = socket_path("line");
    let control =
        opence_runtime::control::bind(&path).map_err(|error| Error::Setup(error.to_string()))?;
    opence_runtime::control::serve(control, opence_runtime::control::Driven::of(&mut game));
    let asking = std::thread::spawn(move || -> Option<String> {
        let mut stream = std::os::unix::net::UnixStream::connect(&path).ok()?;
        stream.write_all(b"\"game_speed 2\"\n").ok()?;
        let mut reply = String::new();
        BufReader::new(stream).read_line(&mut reply).ok()?;
        let _ = std::fs::remove_file(&path);
        Some(reply)
    });
    let line = answer_a_line(&mut game, listener, 0x800, 5)?;
    assert_eq!(line, b"game_speed 2\r\n");
    let reply = asking.join().ok().flatten();
    assert_eq!(reply.as_deref(), Some("{\"ok\":true,\"text\":\"hello\"}\n"));
    Ok(())
}

#[test]
fn the_control_socket_refuses_what_the_console_cannot_run() -> Result<(), Error> {
    // with the console off there is no game to ask; a structured request
    // the socket does not know is refused
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(CONSOLE))?;
    let driven = opence_runtime::control::Driven::of(&mut game);
    for (request, error) in [
        (
            "\"game_speed 2\"",
            "the game's console is not listening yet",
        ),
        (
            "{\"request\": \"state\"}",
            r#"there is no request \"state\""#,
        ),
    ] {
        let reply = opence_runtime::control::answer(&driven, request);
        assert_eq!(reply, format!("{{\"error\":\"{error}\",\"ok\":false}}"));
    }
    Ok(())
}

#[test]
fn the_platform_answers_the_console_s_imports() {
    for name in [
        "env.halo_ws_bind",
        "env.halo_ws_listen",
        "env.halo_ws_accept",
        "env.halo_ws_select",
        "env.halo_ws_recv",
        "env.halo_ws_send",
        "env.halo_ws_ioctlsocket",
        "env.halo_ws_setsockopt",
        "env.halo_ws_getsockname",
        "env.halo_ws_getpeername",
        "env.halo_ws_closesocket",
    ] {
        assert!(answers(name), "{name}");
    }
}

#[test]
fn the_control_socket_opens_and_closes_the_app_s_shell() -> Result<(), Error> {
    // (the app's dev shell is the window's, so the socket leaves the app a
    // request it takes at its next poll, the last one asked winning)
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(CONSOLE))?;
    let driven = opence_runtime::control::Driven::of(&mut game);
    let ask = |request: &str| opence_runtime::control::answer(&driven, request);
    assert_eq!(driven.take_shell(), None);
    assert_eq!(
        ask(r#"{"request": "shell", "open": true}"#),
        r#"{"ok":true}"#
    );
    assert_eq!(driven.take_shell(), Some(true));
    assert_eq!(driven.take_shell(), None);
    ask(r#"{"request": "shell", "open": true}"#);
    ask(r#"{"request": "shell", "open": false}"#);
    assert_eq!(driven.take_shell(), Some(false));
    assert_eq!(
        ask(r#"{"request": "shell"}"#),
        r#"{"error":"a shell request needs \"open\", true or false","ok":false}"#
    );
    assert_eq!(driven.take_shell(), None);
    Ok(())
}

#[test]
fn the_control_socket_reads_and_writes_a_setting_while_the_game_runs() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(CONSOLE))?;
    let driven = opence_runtime::control::Driven::of(&mut game);
    let ask = |request: &str| opence_runtime::control::answer(&driven, request);
    let read = r#"{"request": "setting", "name": "lighting.bounce"}"#;
    assert_eq!(
        ask(read),
        r#"{"name":"lighting.bounce","ok":true,"value":"stock"}"#
    );
    assert_eq!(
        ask(r#"{"request": "setting", "name": "lighting.bounce", "value": "baked"}"#),
        r#"{"name":"lighting.bounce","ok":true,"value":"baked"}"#
    );
    assert_eq!(
        ask(read),
        r#"{"name":"lighting.bounce","ok":true,"value":"baked"}"#
    );
    // a boolean reads as the settings file writes it
    assert_eq!(
        ask(r#"{"request": "setting", "name": "display.per_pixel_lighting"}"#),
        r#"{"name":"display.per_pixel_lighting","ok":true,"value":"false"}"#
    );
    for (request, error) in [
        (
            r#"{"request": "setting", "name": "no.such_setting"}"#,
            r#"there is no setting \"no.such_setting\""#,
        ),
        (
            r#"{"request": "setting", "name": "no.such_setting", "value": "1"}"#,
            r#"there is no setting \"no.such_setting\""#,
        ),
        (r#"{"request": "setting"}"#, r#"a setting needs \"name\""#),
    ] {
        assert_eq!(
            ask(request),
            format!("{{\"error\":\"{error}\",\"ok\":false}}")
        );
    }
    Ok(())
}
