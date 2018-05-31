//! code playing out the speaker (currently on laptop, phone later)

extern crate alsa;
extern crate byteorder;

use byteorder::ByteOrder;

use std::net::UdpSocket;

fn main() {
	let audio = alsa::AudioManager::new();
	let mut buf = [0i16; 16];

	let socket = UdpSocket::bind("192.168.122.1:42015"/*0.0.0.0:42015*/)
		.expect("Could not bind socket");

	socket.send_to(b"electric", "10.0.0.83:42015")
		.expect("Couldn't send packet.");

	let mut netbuf = [0u8; 32];

	loop {
		match socket.recv_from(&mut netbuf) {
			Ok((size, _src)) => {
				for i in 0..size/2 {
					buf[i] = byteorder::NetworkEndian::read_i16(
						&netbuf[i*2..=i*2+1]);
				}

				let buf2 = &buf[..size/2];
				audio.push(&buf2);
			},
			Err(e) => {
				eprintln!("couldn't recieve a datagram: {}", e);
			}
		}
	}
}
