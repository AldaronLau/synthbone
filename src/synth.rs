//! synth test

extern crate alsa; // for speaker
extern crate rand; // for noise generation

mod pink;

use pink::PinkNoiseGenerator;
use std::cell::Cell;

/// An audio wave generator.
/// `(32b wavelength, 32b amplitude, 64b sampling index)`
pub struct Generator(f32, f32, Cell<u64>);

impl Generator {
	/// Create a new generator with default pitch[hz] and volume[0-1].
	pub fn new(hz: f32, volume: f32) -> Self {
		debug_assert!(volume <= 1.0 && volume >= 0.0);
		Generator(hz / 48_000.0, volume, Cell::new(0))
	}

	/// Set pitch.  Takes in Hertz.
	pub fn pitch(&mut self, hz: f32) {
		self.0 = hz / 48_000.0;
	}

	/// Set volume.  Range 0 to 1
	pub fn volume(&mut self, volume: f32) {
		debug_assert!(volume <= 1.0 && volume >= 0.0);
		self.1 = volume;
	}

	/// Generate audio with a closure.  Closure: `|time, volume|`. `time` is
	/// [0-1] and repeats.  `volume` is also [0-1].  Returns sample.
	pub fn gen(&self, buf: &mut [i16], gen: &mut FnMut(f32, f32) -> i16) {
		let mut index = self.2.get();

		for i in buf.iter_mut() {
			*i = gen(((index as f32) * self.0) % 1.0, self.1);
			index += 1;
		}

		self.2.set(index);
	}

	/// A sine wave.
	pub fn sin(&self, buf: &mut [i16]) {
		self.gen(buf, &mut |t, v| {
			convert((t * (::std::f32::consts::PI * 2.0)).sin(), v)
		});
	}

	/// A triangle wave.
	pub fn tri(&self, buf: &mut [i16]) {
		self.gen(buf, &mut |t, v| {
			convert((t * 2.0 - 1.0).abs() * 2.0 - 1.0, v)
		});
	}

	/// A saw wave.
	pub fn saw(&self, buf: &mut [i16]) {
		self.gen(buf, &mut |t, v| {
			convert(t * 2.0 - 1.0, v)
		});
	}

	/// A square wave.
	pub fn sqr(&self, buf: &mut [i16]) {
		self.gen(buf, &mut |t, v| {
			let b = (t * 2.0 - 1.0).signum();
			let a = convert(b, v);
			println!("{}", b);
			a
		});
	}

	/// White noise
	pub fn wht(&self, buf: &mut [i16]) {
		use rand::distributions::{Uniform, Distribution};
		let dist =  Uniform::new_inclusive(-1.0, 1.0);
		let mut rng = rand::thread_rng();
		self.gen(buf, &mut |_t, v| {
			convert(dist.sample(&mut rng), v)
		});
	}

	/// Pink noise
	pub fn pnk(&self, buf: &mut [i16], pink: &mut PinkNoiseGenerator) {
		self.gen(buf, &mut |_t, _v/*todo*/| {
			pink.gen()
		});
	}
}

/// Convert an f32 sample and volume to an i16 sample.
pub fn convert(sample: f32, volume: f32) -> i16 {
	(sample * (::std::i16::MAX as f32) * volume) as i16
}

fn main() {
	let audio = alsa::AudioManager::new();
	let mut buf = [0i16; 48_000 * 5];
	let gen = Generator::new(440.0, 1.0);
	let mut pnk = PinkNoiseGenerator::new();

	gen.pnk(&mut buf, &mut pnk);
//	gen.wht(&mut buf);
	audio.push(&buf);
	::std::thread::sleep(::std::time::Duration::new(5, 0));
}
