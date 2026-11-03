//! The network the game starts on its way to its main menu: Winsock and
//! XNet (`port/include/xdk/xdk_winsock.h`). The app has no system link
//! (the plan has no network task), so the platform answers as an Xbox
//! with its cable out: the network starts, the title has no address
//! (`XNET_GET_XNADDR_NONE`), the link is down and no socket can be made.
//! The transport (`source/bungie_net/network/transport_endpoint_set_winsock.c`)
//! then gives up at once as not initialized, where a pending address would
//! keep it polling for ten seconds. Keys and random bytes are made as the
//! Linux port makes them (`port/linux/src/xnet.c`).
//!
//! With `debug.telnet_console` on, the title has the loopback address and
//! the game's telnet console (`source/networking/telnet_console.c`) may
//! make its stream socket, bind it to 127.0.0.1 and listen. Its only peer
//! is the host, in process, through [`Console`]: no port of the Mac's is
//! opened. Datagrams (system link) still have no network.

use std::collections::{BTreeMap, VecDeque};
use std::hash::{BuildHasher, RandomState};
use std::sync::{Arc, MutexGuard, PoisonError};
use std::time::{Duration, Instant};

use wasmtime::{Caller, Val};

use super::memory::{arg, give, Handler};
use super::space::Space;
use super::{with_space, Record, Shared};

/// XNET_GET_XNADDR_NONE: the title has no address
const NO_ADDRESS: u32 = 1;
/// XNET_GET_XNADDR_ETHERNET | XNET_GET_XNADDR_DHCP, as the Linux port
/// gives an address
const ETHERNET_DHCP: u32 = 0x02 | 0x08;
/// XNET_ETHERNET_LINK_ACTIVE | 100MBPS | FULL_DUPLEX
const LINK_UP: u32 = 0x01 | 0x02 | 0x08;
/// the size of XNADDR (`port/include/xdk/xdk_pdb.h`): its size, flags,
/// abEnet[6] and ina
const XNADDR: usize = 12;
/// and where its ina is
const XNADDR_INA: u32 = 8;
/// the size of WSADATA: two versions, szDescription[257],
/// szSystemStatus[129], two limits and lpVendorInfo, 4-aligned
const WSADATA: usize = 400;
/// where WSADATA's iMaxSockets is
const WSADATA_LIMITS: usize = 390;
/// the Linux port's limits: 64 sockets and 1264-byte datagrams
const MAX_SOCKETS: u16 = 64;
const MAX_DATAGRAM: u16 = 1264;
/// MAKEWORD(2, 2), the highest version
const HIGH_VERSION: u16 = 0x0202;
/// INVALID_SOCKET, and SOCKET_ERROR
const INVALID_SOCKET: u32 = u32::MAX;
const SOCKET_ERROR: u32 = u32::MAX;
/// the sizes of XNKID and XNKEY
const KEY_ID: usize = 8;
const KEY: usize = 16;

/// the setting that turns the console on
const TELNET_CONSOLE: &str = "debug.telnet_console";
/// AF_INET and SOCK_STREAM
const INET: u32 = 2;
const STREAM: u32 = 1;
/// the loopback address, and INADDR_ANY, in network order
const LOOPBACK: [u8; 4] = [127, 0, 0, 1];
const ANY: [u8; 4] = [0; 4];
/// the size of sockaddr_in
const SOCKADDR: usize = 16;
/// the port the host's end has, as a peer's name gives it
const HOST_PORT: u16 = 49152;
/// FIONBIO and FIONREAD
const FIONBIO: u32 = 0x8004_667E;
const FIONREAD: u32 = 0x4004_667F;
/// the sockets a set holds at most (FD_SETSIZE)
const SET_SIZE: u32 = 64;
/// how much either way waits to be taken before more is refused
const QUEUE: usize = 1 << 20;

/// how long the console's output must pause before a reply is taken as
/// ended: the game answers a line within a frame or two, a successful one
/// with at least its "\r\n"
const REPLY_PAUSE: Duration = Duration::from_millis(100);
/// how often a waiting host looks for output
const POLL: Duration = Duration::from_millis(2);

/// Winsock's errors (`port/include/xdk/xdk_winsock.h`)
const WOULD_BLOCK: u32 = 10035;
const NOT_SOCKET: u32 = 10038;
const INVALID: u32 = 10022;
const NO_FAMILY: u32 = 10047;
const IN_USE: u32 = 10048;
const NOT_AVAILABLE: u32 = 10049;
/// WSAENETDOWN: every socket call's error while the network is down
const NETWORK_DOWN: u32 = 10050;
const RESET: u32 = 10054;
/// WSAEFAULT: a buffer outside the game's memory
const FAULT: u32 = 10014;
const NOT_CONNECTED: u32 = 10057;

/// The handler for one of the network's imports, if this module answers it.
pub(crate) fn handler(name: &str) -> Option<Handler> {
    let handler: Handler = match name.strip_prefix("env.")? {
        "WSAStartup" => startup,
        "WSAGetLastError" => last_error,
        "WSACleanup" | "XNetStartup" | "XNetCleanup" | "XNetRegisterKey" | "XNetUnregisterKey" => {
            ok
        }
        "XNetGetTitleXnAddr" => title_address,
        "XNetGetEthernetLinkStatus" => link_status,
        "XNetRandom" => random,
        "XNetCreateKey" => create_key,
        "XNetXnAddrToInAddr" => to_in_address,
        "halo_ws_socket" => socket,
        "halo_ws_setsockopt" => set_option,
        "halo_ws_bind" => bind,
        "halo_ws_listen" => listen,
        "halo_ws_accept" => accept,
        "halo_ws_ioctlsocket" => control,
        "halo_ws_select" => select,
        "halo_ws_recv" => receive,
        "halo_ws_send" => send,
        "halo_ws_getsockname" => own_name,
        "halo_ws_getpeername" => peer_name,
        "halo_ws_closesocket" => close,
        _ => return None,
    };
    Some(handler)
}

/// What a socket of the game's is.
#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    /// made, not yet bound
    Made,
    /// bound to its port
    Bound(u16),
    /// listening on its port
    Listening(u16),
    /// accepted on its port, its peer the link
    Connected(u16, u32),
}

/// The connection between the game's console and the host: what each
/// end has sent the other and not yet taken, and which ends are open.
#[derive(Debug, Default)]
struct Link {
    to_game: VecDeque<u8>,
    to_host: Vec<u8>,
    host_closed: bool,
    game_closed: bool,
}

/// The game's sockets, the links to the host and Winsock's last error.
#[derive(Debug, Default)]
pub(crate) struct Sockets {
    /// the next socket's handle and the next link's number
    next: u32,
    sockets: BTreeMap<u32, State>,
    links: BTreeMap<u32, Link>,
    /// the links waiting for each listener to accept them
    pending: BTreeMap<u32, VecDeque<u32>>,
    /// the host's link, while it has one
    host: Option<u32>,
    /// the last call's error, None for the network being down
    error: Option<u32>,
}

impl Sockets {
    /// a new number, for a socket or a link
    fn number(&mut self) -> u32 {
        self.next = self.next.wrapping_add(1);
        self.next
    }

    /// fails the call with `error`
    fn fail(&mut self, error: u32) -> u32 {
        self.error = Some(error);
        SOCKET_ERROR
    }

    /// the link the connected socket `socket` has
    fn link_of(&self, socket: u32) -> Option<u32> {
        match self.sockets.get(&socket) {
            Some(State::Connected(_, link)) => Some(*link),
            _ => None,
        }
    }

    /// drops `link` once both of its ends are closed
    fn release(&mut self, link: u32) {
        if self
            .links
            .get(&link)
            .is_some_and(|link| link.host_closed && link.game_closed)
        {
            self.links.remove(&link);
        }
    }

    /// whether `socket` has something to read: a listener a connection,
    /// a connected socket bytes or its peer's close
    fn readable(&self, socket: u32) -> bool {
        match self.sockets.get(&socket) {
            Some(State::Listening(_)) => self
                .pending
                .get(&socket)
                .is_some_and(|pending| !pending.is_empty()),
            Some(State::Connected(_, link)) => self
                .links
                .get(link)
                .is_none_or(|link| link.host_closed || !link.to_game.is_empty()),
            _ => false,
        }
    }

    /// whether `socket` may be written to: connected, its peer open
    fn writable(&self, socket: u32) -> bool {
        self.link_of(socket)
            .and_then(|link| self.links.get(&link))
            .is_some_and(|link| !link.host_closed)
    }

    /// the host connects to the listener, if the game has one
    fn connect(&mut self) -> bool {
        self.close_host();
        let listener = self
            .sockets
            .iter()
            .find_map(|(socket, state)| matches!(state, State::Listening(_)).then_some(*socket));
        let Some(listener) = listener else {
            return false;
        };
        let link = self.number();
        self.links.insert(link, Link::default());
        self.pending.entry(listener).or_default().push_back(link);
        self.host = Some(link);
        true
    }

    /// the host closes its end
    fn close_host(&mut self) {
        if let Some(link) = self.host.take() {
            if let Some(open) = self.links.get_mut(&link) {
                open.host_closed = true;
            }
            self.release(link);
        }
    }

    /// the host's link, while both ends are open
    fn host_link(&mut self) -> Option<&mut Link> {
        let link = self.links.get_mut(&self.host?)?;
        (!link.game_closed).then_some(link)
    }
}

/// The game's console as the host reaches it: one connection at a time,
/// as a telnet client would make, to hand it lines and take its replies.
#[derive(Clone)]
pub struct Console {
    shared: Arc<Shared>,
}

impl Console {
    pub(crate) fn new(shared: Arc<Shared>) -> Self {
        Self { shared }
    }

    fn sockets(&self) -> MutexGuard<'_, Sockets> {
        self.shared
            .sockets
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
    }

    /// Connects to the console, closing the connection there was; false
    /// if the game is not listening (the console is off, or not yet up).
    #[must_use]
    pub fn connect(&self) -> bool {
        self.sockets().connect()
    }

    /// Whether the connection is open at both ends.
    #[must_use]
    pub fn connected(&self) -> bool {
        self.sockets()
            .host_link()
            .is_some_and(|link| !link.host_closed)
    }

    /// Hands the console `line`, ended as telnet ends it; nothing if the
    /// connection is closed, or too much is already waiting.
    pub fn send_line(&self, line: &str) {
        if let Some(link) = self.sockets().host_link() {
            if link.to_game.len() + line.len() + 2 <= QUEUE {
                link.to_game.extend(line.bytes().chain(*b"\r\n"));
            }
        }
    }

    /// What the game has written to the connection since the last call
    /// (and before it closed its end), as text.
    #[must_use]
    pub fn take_output(&self) -> String {
        let mut sockets = self.sockets();
        let Some(link) = sockets.host.and_then(|link| sockets.links.get_mut(&link)) else {
            return String::new();
        };
        let bytes = std::mem::take(&mut link.to_host);
        String::from_utf8_lossy(&bytes).into_owned()
    }

    /// Closes the connection.
    pub fn close(&self) {
        self.sockets().close_host();
    }

    /// Hands the console `line` and gives what the game writes back, once
    /// it pauses, within `limit`. A connection is made first if there is
    /// none, its greeting taken and dropped. None if the game is not
    /// listening (the console is off, or not yet up).
    #[must_use]
    pub fn ask(&self, line: &str, limit: Duration) -> Option<String> {
        let started = Instant::now();
        if !self.connected() {
            if !self.connect() {
                return None;
            }
            self.reply(started, limit);
        }
        self.send_line(line);
        Some(self.reply(started, limit))
    }

    /// What the game writes back, once it pauses, within `limit`: the
    /// answer to a line handed over by [`Console::send_line`].
    #[must_use]
    pub fn answer(&self, limit: Duration) -> String {
        self.reply(Instant::now(), limit)
    }

    /// what the game writes until it pauses after its first output, or
    /// `limit` from `started` passes, or the game closes its end
    fn reply(&self, started: Instant, limit: Duration) -> String {
        let mut text = String::new();
        let mut last = None;
        loop {
            let more = self.take_output();
            if !more.is_empty() {
                text.push_str(&more);
                last = Some(Instant::now());
            }
            let paused = last.is_some_and(|last: Instant| last.elapsed() >= REPLY_PAUSE);
            if paused || started.elapsed() >= limit || (more.is_empty() && !self.connected()) {
                return text;
            }
            std::thread::sleep(POLL);
        }
    }
}

/// whether the console is on, as the game's settings have it
fn console_on(record: &Record) -> bool {
    record.settings().boolean(TELNET_CONSOLE) == Some(true)
}

/// runs `call` on the sockets, with the caller's memory, and gives its
/// result
fn with_sockets(
    caller: &mut Caller<'_, Record>,
    results: &mut [Val],
    call: impl FnOnce(&mut Space<'_>, &mut Sockets) -> u32,
) -> wasmtime::Result<()> {
    let result = with_space(caller, |space, record| call(space, &mut record.sockets()))?;
    give(results, result);
    Ok(())
}

/// a sockaddr_in of `port` on the loopback address
fn loopback(port: u16) -> [u8; SOCKADDR] {
    let mut address = [0; SOCKADDR];
    address[..2].copy_from_slice(&u16::try_from(INET).unwrap_or(2).to_le_bytes());
    address[2..4].copy_from_slice(&port.to_be_bytes());
    address[4..8].copy_from_slice(&LOOPBACK);
    address
}

/// writes `address` at `at` as far as the length at `length` allows, and
/// its own length there; nothing if `at` is null
fn put_address(space: &mut Space<'_>, at: u32, length: u32, address: &[u8; SOCKADDR]) {
    if at == 0 {
        return;
    }
    let room = if length == 0 {
        SOCKADDR
    } else {
        space
            .read(length, 4)
            .and_then(|bytes| bytes.try_into().ok())
            .map_or(0, |bytes| u32::from_le_bytes(bytes) as usize)
            .min(SOCKADDR)
    };
    space.write(at, address.get(..room).unwrap_or_default());
    if length != 0 {
        space.write(length, &u32::try_from(SOCKADDR).unwrap_or(16).to_le_bytes());
    }
}

/// the calls that succeed with nothing to do
fn ok(_: &mut Caller<'_, Record>, _: &[Val], results: &mut [Val]) -> wasmtime::Result<()> {
    give(results, 0);
    Ok(())
}

/// `WSAStartup(version, data)`: the version asked for, 2.2 at most, and
/// the limits; a null `data` is not written
fn startup(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let (version, data) = (arg(params, 0), arg(params, 1));
    if data != 0 {
        let mut written = [0; WSADATA];
        let version = u16::try_from(version & 0xFFFF).unwrap_or(HIGH_VERSION);
        written[0..2].copy_from_slice(&version.to_le_bytes());
        written[2..4].copy_from_slice(&HIGH_VERSION.to_le_bytes());
        written[WSADATA_LIMITS..WSADATA_LIMITS + 2].copy_from_slice(&MAX_SOCKETS.to_le_bytes());
        written[WSADATA_LIMITS + 2..WSADATA_LIMITS + 4]
            .copy_from_slice(&MAX_DATAGRAM.to_le_bytes());
        with_space(caller, |space, _| space.write(data, &written))?;
    }
    give(results, 0);
    Ok(())
}

/// `WSAGetLastError()`: the last call's error, the network being down
/// until one fails otherwise
fn last_error(
    caller: &mut Caller<'_, Record>,
    _: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let error = caller.data().sockets().error;
    give(results, error.unwrap_or(NETWORK_DOWN));
    Ok(())
}

/// `XNetGetTitleXnAddr(address)`: cleared but for its size, and none; with
/// the console on, the loopback address
fn title_address(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let address = arg(params, 0);
    let on = console_on(caller.data());
    if address != 0 {
        let mut written = [0; XNADDR];
        written[0] = u8::try_from(XNADDR).unwrap_or(u8::MAX);
        if on {
            written[8..12].copy_from_slice(&LOOPBACK);
        }
        with_space(caller, |space, _| space.write(address, &written))?;
    }
    give(results, if on { ETHERNET_DHCP } else { NO_ADDRESS });
    Ok(())
}

/// `XNetGetEthernetLinkStatus()`: no link, or with the console on one up
fn link_status(
    caller: &mut Caller<'_, Record>,
    _: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    give(
        results,
        if console_on(caller.data()) {
            LINK_UP
        } else {
            0
        },
    );
    Ok(())
}

/// `size` random bytes, from the standard library's randomly keyed hasher
fn random_bytes(size: usize) -> Vec<u8> {
    let state = RandomState::new();
    (0..size.div_ceil(8))
        .flat_map(|block| state.hash_one(block).to_le_bytes())
        .take(size)
        .collect()
}

/// `XNetRandom(buffer, size)`
fn random(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let (buffer, size) = (arg(params, 0), arg(params, 1));
    let bytes = random_bytes(usize::try_from(size).unwrap_or(0));
    if buffer != 0 {
        with_space(caller, |space, _| space.write(buffer, &bytes))?;
    }
    give(results, 0);
    Ok(())
}

/// `XNetCreateKey(identifier, key)`: both random
fn create_key(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let (identifier, key) = (arg(params, 0), arg(params, 1));
    let bytes = random_bytes(KEY_ID + KEY);
    with_space(caller, |space, _| {
        if identifier != 0 {
            space.write(identifier, bytes.get(..KEY_ID).unwrap_or_default());
        }
        if key != 0 {
            space.write(key, bytes.get(KEY_ID..).unwrap_or_default());
        }
    })?;
    give(results, 0);
    Ok(())
}

/// `XNetXnAddrToInAddr(address, identifier, result)`: the address's own
/// ina, as the Linux port gives it for a system link peer
fn to_in_address(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let (address, result) = (arg(params, 0), arg(params, 2));
    if address != 0 && result != 0 {
        with_space(caller, |space, _| {
            let ina = space
                .read(address.wrapping_add(XNADDR_INA), 4)
                .unwrap_or_default();
            space.write(result, &ina);
        })?;
    }
    give(results, 0);
    Ok(())
}

/// `socket(family, type, protocol)`: with the console on, a stream on
/// AF_INET; otherwise none, the network being down
fn socket(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let stream = arg(params, 0) == INET && arg(params, 1) == STREAM;
    let on = console_on(caller.data());
    let mut sockets = caller.data().sockets();
    let made = if stream && on && sockets.sockets.len() < usize::from(MAX_SOCKETS) {
        let socket = sockets.number();
        sockets.sockets.insert(socket, State::Made);
        socket
    } else {
        sockets.error = None;
        INVALID_SOCKET
    };
    drop(sockets);
    give(results, made);
    Ok(())
}

/// `setsockopt(socket, level, name, value, length)`: taken, and nothing
/// to do on the loopback
fn set_option(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let socket = arg(params, 0);
    with_sockets(caller, results, |_, sockets| {
        if sockets.sockets.contains_key(&socket) {
            0
        } else {
            sockets.fail(NOT_SOCKET)
        }
    })
}

/// `bind(socket, address, length)`: to a free port on the loopback
/// address (or any address, which is the loopback's)
fn bind(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let (socket, address, length) = (arg(params, 0), arg(params, 1), arg(params, 2));
    with_sockets(caller, results, |space, sockets| {
        if sockets.sockets.get(&socket) != Some(&State::Made) {
            return sockets.fail(if sockets.sockets.contains_key(&socket) {
                INVALID
            } else {
                NOT_SOCKET
            });
        }
        let Some(bytes) = (length as usize >= 8)
            .then(|| space.read(address, 8))
            .flatten()
        else {
            return sockets.fail(INVALID);
        };
        if u32::from(u16::from_le_bytes([bytes[0], bytes[1]])) != INET {
            return sockets.fail(NO_FAMILY);
        }
        if bytes[4..8] != LOOPBACK && bytes[4..8] != ANY {
            return sockets.fail(NOT_AVAILABLE);
        }
        let port = u16::from_be_bytes([bytes[2], bytes[3]]);
        let taken = sockets.sockets.values().any(|state| {
            matches!(state, State::Bound(bound) | State::Listening(bound) if *bound == port)
        });
        if taken {
            return sockets.fail(IN_USE);
        }
        sockets.sockets.insert(socket, State::Bound(port));
        0
    })
}

/// `listen(socket, backlog)`: a bound socket starts taking connections
fn listen(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let socket = arg(params, 0);
    with_sockets(caller, results, |_, sockets| {
        match sockets.sockets.get(&socket).copied() {
            Some(State::Bound(port)) => {
                sockets.sockets.insert(socket, State::Listening(port));
                0
            }
            Some(State::Listening(_)) => 0,
            Some(_) => sockets.fail(INVALID),
            None => sockets.fail(NOT_SOCKET),
        }
    })
}

/// `accept(socket, address, length)`: the connection waiting longest, as
/// a new socket, its peer's name written; none waiting would block
fn accept(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let (socket, address, length) = (arg(params, 0), arg(params, 1), arg(params, 2));
    with_sockets(caller, results, |space, sockets| {
        let Some(State::Listening(port)) = sockets.sockets.get(&socket).copied() else {
            sockets.fail(if sockets.sockets.contains_key(&socket) {
                INVALID
            } else {
                NOT_SOCKET
            });
            return INVALID_SOCKET;
        };
        let Some(link) = sockets
            .pending
            .get_mut(&socket)
            .and_then(VecDeque::pop_front)
        else {
            sockets.fail(WOULD_BLOCK);
            return INVALID_SOCKET;
        };
        let accepted = sockets.number();
        sockets
            .sockets
            .insert(accepted, State::Connected(port, link));
        put_address(space, address, length, &loopback(HOST_PORT));
        accepted
    })
}

/// `ioctlsocket(socket, command, argument)`: FIONBIO is taken (every
/// call returns at once), FIONREAD gives the bytes waiting
fn control(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let (socket, command, argument) = (arg(params, 0), arg(params, 1), arg(params, 2));
    with_sockets(caller, results, |space, sockets| {
        if !sockets.sockets.contains_key(&socket) {
            return sockets.fail(NOT_SOCKET);
        }
        match command {
            FIONBIO => 0,
            FIONREAD => {
                let waiting = sockets
                    .link_of(socket)
                    .and_then(|link| sockets.links.get(&link))
                    .map_or(0, |link| link.to_game.len());
                if argument != 0 {
                    let waiting = u32::try_from(waiting).unwrap_or(u32::MAX);
                    space.write(argument, &waiting.to_le_bytes());
                }
                0
            }
            _ => sockets.fail(INVALID),
        }
    })
}

/// leaves in the set at `at` (a count and up to 64 sockets) only the
/// sockets `keep` keeps; how many it kept
fn filter_set(space: &mut Space<'_>, at: u32, keep: impl Fn(u32) -> bool) -> u32 {
    if at == 0 {
        return 0;
    }
    let count = space
        .read(at, 4)
        .and_then(|bytes| bytes.try_into().ok())
        .map_or(0, u32::from_le_bytes)
        .min(SET_SIZE);
    let kept: Vec<u32> = space
        .read(at.wrapping_add(4), count * 4)
        .unwrap_or_default()
        .as_chunks::<4>()
        .0
        .iter()
        .map(|socket| u32::from_le_bytes(*socket))
        .filter(|&socket| keep(socket))
        .collect();
    let mut written = u32::try_from(kept.len())
        .unwrap_or(0)
        .to_le_bytes()
        .to_vec();
    written.extend(kept.iter().flat_map(|socket| socket.to_le_bytes()));
    space.write(at, &written);
    u32::try_from(kept.len()).unwrap_or(0)
}

/// `select(count, readable, writable, failed, timeout)`: at once, never
/// waiting, the sets cut to the sockets ready; how many are
fn select(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let (readable, writable, failed) = (arg(params, 1), arg(params, 2), arg(params, 3));
    with_sockets(caller, results, |space, sockets| {
        let read = filter_set(space, readable, |socket| sockets.readable(socket));
        let written = filter_set(space, writable, |socket| sockets.writable(socket));
        filter_set(space, failed, |_| false);
        read + written
    })
}

/// `recv(socket, buffer, length, flags)`: the bytes waiting, up to
/// `length`; 0 once the host has closed and nothing waits; none waiting
/// would block
fn receive(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let (socket, buffer, length) = (arg(params, 0), arg(params, 1), arg(params, 2));
    with_sockets(caller, results, |space, sockets| {
        let Some(link) = sockets.link_of(socket) else {
            return sockets.fail(if sockets.sockets.contains_key(&socket) {
                NOT_CONNECTED
            } else {
                NOT_SOCKET
            });
        };
        let Some(open) = sockets.links.get_mut(&link) else {
            return 0;
        };
        if open.to_game.is_empty() {
            return if open.host_closed {
                0
            } else {
                sockets.fail(WOULD_BLOCK)
            };
        }
        let taken = (length as usize).min(open.to_game.len());
        let bytes: Vec<u8> = open.to_game.drain(..taken).collect();
        space.write(buffer, &bytes);
        u32::try_from(taken).unwrap_or(0)
    })
}

/// `send(socket, buffer, length, flags)`: all of it, for the host to take;
/// refused once the host has closed, or too much waits
fn send(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let (socket, buffer, length) = (arg(params, 0), arg(params, 1), arg(params, 2));
    with_sockets(caller, results, |space, sockets| {
        let Some(link) = sockets.link_of(socket) else {
            return sockets.fail(if sockets.sockets.contains_key(&socket) {
                NOT_CONNECTED
            } else {
                NOT_SOCKET
            });
        };
        let Some(open) = sockets
            .links
            .get_mut(&link)
            .filter(|open| !open.host_closed)
        else {
            return sockets.fail(RESET);
        };
        if open.to_host.len() + length as usize > QUEUE {
            return sockets.fail(WOULD_BLOCK);
        }
        let Some(bytes) = space.read(buffer, length) else {
            return sockets.fail(FAULT);
        };
        open.to_host.extend_from_slice(&bytes);
        length
    })
}

/// `getsockname(socket, address, length)`: the loopback address and the
/// port the socket has
fn own_name(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let (socket, address, length) = (arg(params, 0), arg(params, 1), arg(params, 2));
    with_sockets(caller, results, |space, sockets| {
        match sockets.sockets.get(&socket).copied() {
            Some(State::Bound(port) | State::Listening(port) | State::Connected(port, _)) => {
                put_address(space, address, length, &loopback(port));
                0
            }
            Some(State::Made) => sockets.fail(INVALID),
            None => sockets.fail(NOT_SOCKET),
        }
    })
}

/// `getpeername(socket, address, length)`: the host's end, for a
/// connected socket
fn peer_name(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let (socket, address, length) = (arg(params, 0), arg(params, 1), arg(params, 2));
    with_sockets(caller, results, |space, sockets| {
        match sockets.sockets.get(&socket) {
            Some(State::Connected(..)) => {
                put_address(space, address, length, &loopback(HOST_PORT));
                0
            }
            Some(_) => sockets.fail(NOT_CONNECTED),
            None => sockets.fail(NOT_SOCKET),
        }
    })
}

/// `closesocket(socket)`: gone; a connected socket's link is closed at
/// the game's end, a listener's waiting links at both
fn close(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let socket = arg(params, 0);
    with_sockets(caller, results, |_, sockets| {
        let Some(state) = sockets.sockets.remove(&socket) else {
            return sockets.fail(NOT_SOCKET);
        };
        let closed: Vec<u32> = match state {
            State::Connected(_, link) => vec![link],
            State::Listening(_) => sockets
                .pending
                .remove(&socket)
                .map(Vec::from)
                .unwrap_or_default(),
            _ => Vec::new(),
        };
        for link in closed {
            if let Some(open) = sockets.links.get_mut(&link) {
                open.game_closed = true;
            }
            sockets.release(link);
        }
        0
    })
}
