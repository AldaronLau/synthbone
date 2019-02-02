extern crate sound; // for sound generation / effects
extern crate alsa; // for speaker

use sound::{ Generator, PnkGenerator, WhtGenerator };

fn main() {
	let mut audio = alsa::AudioManager::new();
	let mut gen = Generator::new(220.0, 1.0);
	let mut _pnk = PnkGenerator::new();
	let mut _wht = WhtGenerator::new();

	loop {
		audio.play(&mut || gen.gen(&mut |x| {
//			sound::wht(&mut _wht)
			sound::pnk(&mut _pnk)
		}));
	}
}
