//! code running inside the trombone mute on a Pi Zero W

extern crate pitch; // for pitch detection
extern crate alsa; // for microphone
extern crate byteorder; // for networking endian

use std::net::UdpSocket;
use byteorder::{ NetworkEndian, WriteBytesExt };

fn main() {
	let socket = UdpSocket::bind(/*"10.0.0.83:42015"*/"raspberrypi.local:42015")
		.expect("Could not bind socket");

	// Wait for a connection.
	println!("Waiting for a connection....");
	let connection = {
		let mut buf = [0u8; 8];
		match socket.recv_from(&mut buf) {
			Ok((size, src)) => {
				if size == 8 && buf == *b"electric" {
					src
				} else {
					panic!("Error: size:{}, buf:{}",
						size, ::std::str::from_utf8(
							&buf).unwrap())
				}
			},
			Err(e) => {
				panic!("couldn't recieve a datagram: {}", e)
			},
		}
	};
	println!("Connected!");

	let audio = alsa::AudioManager::new();
	let mut buf = [0i16; 2048 /*480*/];
	let mut buf2 = [0f32; 2048];
	let mut netbuf = [0u8; 8]; // 2 f32s (hz/pitch, amplitude/volume)

	loop {
		let l = audio.pull(&mut buf);

		if l == 0 { continue }

		for i in 0..l {
			buf2[i] = buf[i] as f32;
		}

		let (hz, amplitude) = pitch::detect(&buf2);

		(&mut netbuf[0..4]).write_f32::<NetworkEndian>(hz).unwrap();
		(&mut netbuf[4..8]).write_f32::<NetworkEndian>(amplitude)
			.unwrap();

		socket.send_to(&netbuf, &connection).expect("Couldn't send");
	}
}
