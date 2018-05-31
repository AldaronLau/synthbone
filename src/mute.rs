//! code running inside the trombone mute on a Pi Zero W

extern crate alsa;
extern crate sample;
extern crate byteorder;

use byteorder::ByteOrder;

use std::net::UdpSocket;

fn main() {
	let socket = UdpSocket::bind("10.0.0.83:42015"/*0.0.0.0:42015*/)
		.expect("Could not bind socket");

	// Wait for a connection.
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

	let mut recording = vec![];
	let audio = alsa::AudioManager::new();
	let mut buf = [0i16; 16];
	let mut netbuf = [0u8; 32];

	loop {
		let l = audio.pull(&mut buf);

		let buf2 = &mut buf[..l];

		for i in buf2.iter_mut() {
			// Convert to float
			let mut j = (*i as f32) / (::std::u16::MAX as f32);

			// Clipping distortion by amplification.
			j *= 16.0;

			// Apply reverb here.
			j += *recording.last().unwrap_or(&0.0) * 0.8;

			recording.push(j);

			// Back to i16
			*i = if j < 1.0 && j > -1.0 {
				j * (::std::u16::MAX as f32)
			} else {
				(::std::u16::MAX as f32)
			} as i16;
		}

		for i in 0..buf2.len() {
			byteorder::NetworkEndian::write_i16(&mut netbuf[i*2..=i*2+1],
				buf2[i]);
		}

		socket.send_to(&netbuf[..buf2.len()*2], &connection)
			.expect("Couldn't send");
	}
}
