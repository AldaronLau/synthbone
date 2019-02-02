//! code running inside the trombone mute on a Pi Zero W

extern crate pitch; // for pitch detection
extern crate adi; // for microphone
extern crate byteorder; // for networking endian

use std::fs::File;
use std::net::UdpSocket;
use byteorder::{ NetworkEndian, WriteBytesExt };

const SCALE: f32 = ::std::i16::MAX as f32;

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

	let mic = adi::speaker::Microphone::new(0, false).unwrap();
	let mut buf = [0i16; 2048 /*480*/];
	let mut buf2 = [0f32; 2048];
	let mut netbuf = [0u8; 8]; // 2 f32s (hz/pitch, amplitude/volume)

	loop {
		let l = mic.update(&mut buf);

		if l == 0 { continue }

		for i in 0..l {
			let mut a = (buf[i] as f32) / SCALE;
			if a > -0.005 && a < 0.005 { // reduce noise
				a = 0.0;
			}
			buf2[i] = a;
		}

		let (hz, mut amplitude) = pitch::detect(&buf2);

		if amplitude < 0.05 { // noise floor
			amplitude = 0.0;
		}

		if hz < 60.0 {
			let mut file = File::create("bad.raw").unwrap();

			for i in buf2.iter() {
				file.write_f32::<byteorder::LittleEndian>(*i)
					.unwrap();
			}
		}

		(&mut netbuf[0..4]).write_f32::<NetworkEndian>(hz).unwrap();
		(&mut netbuf[4..8]).write_f32::<NetworkEndian>(amplitude)
			.unwrap();

		socket.send_to(&netbuf, &connection).expect("Couldn't send");
	}
}
