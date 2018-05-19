extern crate alsa;
extern crate sample;

// mod pc;

fn main() {
	let mut recording = vec![];
	let audio = alsa::AudioManager::new();
	let mut buf = [0i16; 16];

	loop {
		let l = audio.pull(&mut buf);

		let buf2 = &mut buf[0..l];

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

		audio.push(&buf2);
	}
}
