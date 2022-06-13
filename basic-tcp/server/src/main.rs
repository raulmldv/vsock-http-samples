use std::{net::{Ipv6Addr, TcpListener, SocketAddrV6}, time::Duration, io::Read};

use clap::{App, Arg};

const BUFF_SIZE: usize = 8192;
const MAX_EXCHANGES : i32 = 5;

fn main() {
    let app = App::new("Server app")
        .arg(
            Arg::with_name("port")
                .long("port")
                .takes_value(true)
                .required(true)
        );
    let args = app.get_matches();

    let port = args.value_of("port").expect("Could not find port argument");

    // let loopback = Ipv4Addr::new(127, 0, 0, 1);
    // let loopback = Ipv6Addr::new(0xfc00, 0, 0, 0, 0, 0, 0, 1);
    // let sockaddr = SocketAddrV6::new(loopback, port.parse::<u16>().expect("Failed to parese port."), 0, 0);
    let listener = TcpListener::bind("0.0.0.0:".to_string() + &port.to_string()).expect("Could not bind.");

    let (mut client, _client_addr) = listener
        .accept()
        .expect("Could not accept connection");

    println!("Server running on localhost, port: {:?}", port);
    
    for _i in 0..MAX_EXCHANGES {
        let mut buffer = [0u8; BUFF_SIZE];
        let nbytes = client.read(&mut buffer);
        let nbytes = nbytes.unwrap_or(0);

        if nbytes == 0 {
            println!("No message: Empty buffer.");
        } else {
            println!(
                "{}",
                String::from_utf8(buffer.to_vec())
                    .expect("The received bytes are not UTF-8: {:?}")
            );
        }
        
        std::thread::sleep(Duration::from_millis(5000));
    }
}