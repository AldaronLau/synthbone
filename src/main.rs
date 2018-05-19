extern crate alsa;
extern crate sample;

// mod pc;

fn main() {
	let ad = alsa::AudioDevice::new();
	let player = alsa::Player::new(&ad);
	let capturer = alsa::Capturer::new(&ad);

	let mut recording = vec![];
//	let mut captured = 0;
//	let mut played = 0;

	loop {
		// 4 sample buffer works really well, no buffer underruns.  4 is
		// the least number of samples we can get at a time without
		// introducing latency.
		let mut buf = [0i16; 16];

		let l = capturer.capture(&ad, &mut buf);

		if l == 0 { continue } else {
//			captured += l;
//			println!("{}", player.delay(), l);
		}

		let buf2 = &mut buf[0..l];

		for i in buf2.iter_mut() {
			// Convert to float
			let mut j = (*i as f32) / (::std::u16::MAX as f32);

			// Clipping distortion by amplification.
			j *= 32.0;

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

//		played += buf2.len();
		player.play(&ad, &buf2);

//		println!("{} -> {}", capturer.delay(&ad), player.delay(&ad));
	}

	
}
