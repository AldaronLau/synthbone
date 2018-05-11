extern crate alsa;
extern crate sample;

mod pc;

fn main() {
	let player = pc::Player::new();
	let capturer = pc::Capturer::new();

	let mut prev = 0;

	loop {
		let mut buf = [0i16; 4];

		capturer.capture(&mut buf);

		for i in &mut buf {
			// Clipping distortion by amplification.
			*i = i.checked_mul(32).unwrap_or(::std::i16::MAX);

			// Apply reverb here.
			*i = i.checked_add(((prev as f32) * 0.8) as i16)
				.unwrap_or(::std::i16::MAX);
			prev = *i;
		}

		player.play(&buf);
	}
}
