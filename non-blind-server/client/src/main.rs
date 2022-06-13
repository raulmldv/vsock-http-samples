use std::{time::Duration};

use clap::{App, Arg};
use std::os::unix::io::{AsRawFd, RawFd};
use nix::sys::socket::{connect, shutdown, socket, send};
use nix::sys::socket::{AddressFamily, Shutdown, SockAddr, SockFlag, SockType};
use nix::unistd::close;
use std::mem::size_of;
use byteorder::{ByteOrder, LittleEndian};
use nix::errno::Errno::EINTR;
use nix::sys::socket::MsgFlags;

const MAX_CONNECTION_ATTEMPTS: usize = 5;

struct VsockSocket {
    socket_fd: RawFd,
}

impl VsockSocket {
    fn new(socket_fd: RawFd) -> Self {
        VsockSocket { socket_fd }
    }
}

impl Drop for VsockSocket {
    fn drop(&mut self) {
        shutdown(self.socket_fd, Shutdown::Both)
            .unwrap_or_else(|e| eprintln!("Failed to shut socket down: {:?}", e));
        close(self.socket_fd).unwrap_or_else(|e| eprintln!("Failed to close socket: {:?}", e));
    }
}

impl AsRawFd for VsockSocket {
    fn as_raw_fd(&self) -> RawFd {
        self.socket_fd
    }
}

pub fn send_u64(fd: RawFd, val: u64) -> Result<(), String> {
    let mut buf = [0u8; size_of::<u64>()];
    LittleEndian::write_u64(&mut buf, val);
    send_loop(fd, &mut buf, size_of::<u64>().try_into().unwrap())?;
    Ok(())
}

pub fn send_loop(fd: RawFd, buf: &[u8], len: u64) -> Result<(), String> {
    let len: usize = len.try_into().map_err(|err| format!("{:?}", err))?;
    let mut send_bytes = 0;

    while send_bytes < len {
        let size = match send(fd, &buf[send_bytes..len], MsgFlags::empty()) {
            Ok(size) => size,
            Err(nix::Error::Sys(EINTR)) => 0,
            Err(err) => return Err(format!("{:?}", err)),
        };
        send_bytes += size;
    }

    Ok(())
}

/// Initiate a connection on an AF_VSOCK socket
fn vsock_connect(cid: u32, port: u32) -> Result<VsockSocket, String> {
    let sockaddr = SockAddr::new_vsock(cid, port);
    let mut err_msg = String::new();

    for i in 0..MAX_CONNECTION_ATTEMPTS {
        let vsocket = VsockSocket::new(
            socket(
                AddressFamily::Vsock,
                SockType::Stream,
                SockFlag::empty(),
                None,
            )
            .map_err(|err| format!("Failed to create the socket: {:?}", err))?,
        );
        match connect(vsocket.as_raw_fd(), &sockaddr) {
            Ok(_) => return Ok(vsocket),
            Err(e) => err_msg = format!("Failed to connect: {}", e),
        }

        // Exponentially backoff before retrying to connect to the socket
        std::thread::sleep(std::time::Duration::from_secs(1 << i));
    }

    Err(err_msg)
}

fn main() {
    let app = App::new("Client app")
        .arg(
            Arg::with_name("addr")
                .long("addr")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name("port")
                .long("port")
                .takes_value(true)
                .required(true)
        );
    let args = app.get_matches();

    let _addr = args.value_of("addr").expect("Could not find addr argument");
    let _port = args.value_of("port").expect("Could not find port argument");

    // let mut socket = TcpStream::connect(addr.to_string() + ":" + &port.to_string()).expect("Failed connection to server.");
    let vsocket = vsock_connect(16, 8000).expect("Failed connection to server.");
    let fd = vsocket.as_raw_fd();
    // let sockaddr = SockAddr::new_vsock(16, 8000);
    // let mut socket = VsockStream::connect(&sockaddr).expect("Failed connection to server.");
    loop {
        // fd.write_all(b"Hello from Client!\r\n").expect("Failed to send bytes.");
        let data = "Hello, world!".to_string();
        let buf = data.as_bytes();
        let len: u64 = buf.len().try_into().map_err(|err| 
            format!("{:?}", err)
        ).expect("Failed to get len.");
        send_u64(fd, len).expect("Failed u64 send.");
        send_loop(fd, &buf, len).expect("Failed send loop.");
        std::thread::sleep(Duration::from_millis(5000));
    }
}
