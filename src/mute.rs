//! code running inside the trombone mute on a Pi Zero W

extern crate alsa;
extern crate sample;
extern crate opus;

use opus::{ Channels, Application };

use std::net::UdpSocket;

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

	let mut recording = vec![];
	let audio = alsa::AudioManager::new();
	let mut buf = [0i16; 480];
	let mut netbuf = [0u8; 480*2];

	let mut opus = opus::Encoder::new(48000, Channels::Mono,
			Application::LowDelay)
		.unwrap();

	loop {
		let l = audio.pull(&mut buf);

		if l == 0 { continue }

		let buf2 = &mut buf[..l];

		for i in buf2.iter_mut() {
			// Convert to float
			let mut j = (*i as f32) / (::std::i16::MAX as f32);

			// Noise reduction (floor of 0.1)
			if j < 0.05 && j > -0.05 {
				j = 0.0;
			}

			// Clipping distortion by amplification.
			j *= 16.0;

			// Apply reverb here.
			j += *recording.last().unwrap_or(&0.0) * 0.8;

			recording.push(j);

			// Back to i16
			*i = if j < 1.0 && j > -1.0 {
				(j * (::std::i16::MAX as f32)) as i16
			} else if j > 0.0 { // Positive Clipping
				::std::i16::MAX
			} else { // Negative Clipping
				::std::i16::MIN
			};
		}

		println!("{}", buf2.len());

		let nl = opus.encode(buf2, &mut netbuf).unwrap();

		socket.send_to(&netbuf[..nl], &connection)
			.expect("Couldn't send");
	}
}
