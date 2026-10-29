use std::io;
use std::net::UdpSocket;

use crate::network::protocol;

const LISTEN_ADDRESS: &str = "0.0.0.0";
const LISTEN_PORT: u16 = 8888;
const BUFFER_SIZE: usize = 1024;

pub fn packet_loop(hostname: &str) -> io::Result<()> {
    let address = format!("{LISTEN_ADDRESS}:{LISTEN_PORT}");
    let socket = UdpSocket::bind(&address)?;

    println!("Listening on {address}");

    let mut buffer = [0u8; BUFFER_SIZE];

    loop {
        let (size, sender) = socket.recv_from(&mut buffer)?;

        protocol::handle_packet(&socket, &buffer[..size], &sender, hostname)?;
    }
}
