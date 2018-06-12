//! code playing out the speaker (currently on laptop, phone later)

extern crate pitch; // for pitch detection
extern crate alsa; // for speaker
extern crate byteorder; // for networking endian

use std::net::UdpSocket;
use byteorder::{ NetworkEndian, ReadBytesExt };

fn main() {
//	let audio = alsa::AudioManager::new();
//	let mut buf = [0i16; 480];

	let socket = UdpSocket::bind(/*"192.168.122.1:42015"*/"192.168.4.14:42015")
		.expect("Could not bind socket");

	socket.send_to(b"electric", "raspberrypi.local:42015"/*"10.0.0.83:42015"*/)
		.expect("Couldn't send packet.");
	println!("Connected!");

	let mut netbuf = [0u8; 8];

	loop {
		match socket.recv_from(&mut netbuf) {
			Ok((_size, _src)) => {
				let hz = (&netbuf[0..4])
					.read_f32::<NetworkEndian>().unwrap();
				let amplitude = (&netbuf[4..8])
					.read_f32::<NetworkEndian>().unwrap();

				println!("{} {}", hz, amplitude);

//				audio.push(&buf2);
			},
			Err(e) => {
				eprintln!("couldn't recieve a datagram: {}", e);
			}
		}
	}
}
