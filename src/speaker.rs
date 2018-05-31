//! code playing out the speaker (currently on laptop, phone later)

extern crate alsa;
extern crate opus;

use opus::Channels;

use std::net::UdpSocket;

fn main() {
	let audio = alsa::AudioManager::new();
	let mut buf = [0i16; 480];

	let socket = UdpSocket::bind(/*"192.168.122.1:42015"*/"192.168.4.14:42015")
		.expect("Could not bind socket");

	socket.send_to(b"electric", "raspberrypi.local:42015"/*"10.0.0.83:42015"*/)
		.expect("Couldn't send packet.");
	println!("Connected!");

	let mut netbuf = [0u8; 480*2];

	let mut opus = opus::Decoder::new(48000, Channels::Mono).unwrap();

	loop {
		match socket.recv_from(&mut netbuf) {
			Ok((size, _src)) => {
				let l = opus.decode(&netbuf[..size], &mut buf, false)
					.unwrap();

				let buf2 = &buf[..l];
				audio.push(&buf2);
			},
			Err(e) => {
				eprintln!("couldn't recieve a datagram: {}", e);
			}
		}
	}
}
