use std::io::Write;
use std::net::TcpStream;
use std::{time::Duration};

use clap::{App, Arg};

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

    let addr = args.value_of("addr").expect("Could not find addr argument");
    let port = args.value_of("port").expect("Could not find port argument");

    let mut socket = TcpStream::connect(addr.to_string() + ":" + &port.to_string()).expect("Failed connection to server.");
    loop {
        socket.write_all(b"Hello from Client!\r\n").expect("Failed to send bytes.");
        std::thread::sleep(Duration::from_millis(5000));
    }
}
