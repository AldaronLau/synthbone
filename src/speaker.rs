//! code playing out the speaker (currently on laptop, phone later)

extern crate twang; // for sound generation / effects
extern crate adi; // for speaker
extern crate byteorder; // for networking endian

use std::sync::{Arc, Mutex};
use std::thread;
use std::net::UdpSocket;
use byteorder::{ NetworkEndian, ReadBytesExt };

struct SynthData {
	pitch: f64,
	volume: f64,
}

fn main() {
	let data = Arc::new(Mutex::new(SynthData { pitch: 440.0, volume: 0.5 }));
	let ndata = data.clone();

	let mut speaker = adi::speaker::Speaker::new(0, false).unwrap();
	let mut netbuf = [0u8; 16]; // 2 f64s
	let mut gen = twang::Generator::new(440.0, 1.0);

	let socket = UdpSocket::bind(/*"192.168.122.1:42015"*/"192.168.4.14:42015")
		.expect("Could not bind socket");

	socket.send_to(b"electric", "raspberrypi.local:42015"/*"10.0.0.83:42015"*/)
		.expect("Couldn't send packet.");

	println!("Connected!");

	thread::spawn(move || { loop {
		// Check for packets updating pitch and volume.
		match socket.recv_from(&mut netbuf) {
			Ok((_size, _src)) => {
				let hz = (&netbuf[0..8])
					.read_f64::<NetworkEndian>().unwrap();
				let amplitude = (&netbuf[8..16])
					.read_f64::<NetworkEndian>().unwrap();
				{
					let mut data = ndata.lock().unwrap();

					if hz < 15_000.0 { data.pitch = hz; }
					data.volume = amplitude;
					println!("{} {}", data.pitch, data.volume);
					// data unlocked
				}
			},
			Err(e) => {
				eprintln!("couldn't recieve a datagram: {}", e);
			}
		}
	} });

	loop {
		// Write out the the speaker.
		speaker.update(&mut || {
			// Do
			{
				let data = data.lock().unwrap();
				gen.pitch(data.pitch);
				gen.volume(data.volume * 4.0);
				// data unlocked.
			}

			let x = gen.next();

			twang::out(twang::tri(x))
		});
	}
}
