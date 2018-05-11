extern crate alsa;
extern crate sample;

mod pc;

fn main() {
	let player = pc::Player::new();
	let capturer = pc::Capturer::new();

	let mut recording = vec![];

	loop {
		// 4 sample buffer works really well, no buffer underruns.  4 is
		// the least number of samples we can get at a time without
		// introducing latency.
		let mut buf = [0i16; 4];

		capturer.capture(&mut buf);

		for i in buf.iter_mut() {
			// Convert to float
			let mut j = (*i as f32) / (::std::u16::MAX as f32);

			// Clipping distortion by amplification.
			j *= 32.0;

			// Apply reverb here.
			j += *recording.last().unwrap_or(&0.0) * 0.8;

			recording.push(j);

			// Back to u16
			*i = if j < 1.0 && j > -1.0 {
				j * (::std::u16::MAX as f32)
			} else {
				(::std::u16::MAX as f32)
			} as i16;
		}

		player.play(&buf);
	}

	
}
