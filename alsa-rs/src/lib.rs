extern crate libc;
extern crate nix;

mod alsa;

fn set_settings(context: &alsa::Context, pcm: &alsa::pcm::PCM) {
	// Set hardware parameters: 44100 Hz / Mono / 16 bit
	let hwp = alsa::pcm::HwParams::any(context, pcm).unwrap();
	hwp.set_channels(context, 1).unwrap();
	hwp.set_rate(context, 48000, alsa::ValueOr::Nearest).unwrap();
	let rate = hwp.get_rate(context).unwrap();
//	println!("RATE: {}", rate);
	assert_eq!(rate, 48_000);
	hwp.set_format(context, alsa::pcm::Format::s16()).unwrap();
	hwp.set_access(context, alsa::pcm::Access::RWInterleaved).unwrap();
	pcm.hw_params(context, &hwp).unwrap();
	hwp.drop(context);
}

pub struct AudioManager {
	context: alsa::Context,
	#[cfg(feature = "speaker")]
	speaker: (i64, alsa::pcm::PCM), // TODO: call drop(), it isn't being called rn.
	#[cfg(feature = "speaker")]
	speaker_buffer: Vec<i16>,
	#[cfg(feature = "microphone")]
	microphone: alsa::pcm::PCM, // TODO: call drop(), it isn't being called rn.
}

impl AudioManager {
	/// Create a new `AudioManager`.
	pub fn new() -> Self {
		let context = alsa::Context::new();

		#[cfg(feature = "speaker")]
		let (speaker, speaker_buffer) = {
			let pcm = alsa::pcm::PCM::new(&context, "default",
				alsa::Direction::Playback).unwrap();
			set_settings(&context, &pcm);
			let mut speaker_max_latency;
			(({
				let hwp = pcm.hw_params_current(&context).unwrap();
				let bs = hwp.get_buffer_size(&context).unwrap();

				println!("Buffer Size: {}", bs);
				speaker_max_latency
					= hwp.get_period_size(&context).unwrap()
						as usize * 2;

				println!("PC: {}", hwp.get_channels(&context).unwrap());
				println!("PR: {}", hwp.get_rate(&context).unwrap());

				hwp.drop(&context);
				bs
			}, pcm), vec![0i16; speaker_max_latency])
		};

		#[cfg(feature = "microphone")]
		let microphone = {
			let pcm = alsa::pcm::PCM::new(&context, "plughw:1,0",
				alsa::Direction::Capture).unwrap();
			set_settings(&context, &pcm);
			{
				let hwp = pcm.hw_params_current(&context).unwrap();
				println!("CC: {}", hwp.get_channels(&context).unwrap());
				println!("CR: {}", hwp.get_rate(&context).unwrap());
				hwp.drop(&context);
			}
			pcm
		};

		#[cfg(feature = "speaker")] {
			speaker.1.prepare(&context);
		}

		#[cfg(feature = "microphone")] {
			microphone.start(&context);
		}

		let am = AudioManager {
			context,
			#[cfg(feature = "speaker")]
			speaker,
			#[cfg(feature = "speaker")]
			speaker_buffer,
			#[cfg(feature = "microphone")]
			microphone,
		};

		am
	}

	#[cfg(feature = "speaker")]
	/// Push data to the speaker output.
	fn push(&self, buffer: &[i16]) {
		if self.speaker.1.writei(&self.context, buffer).unwrap_or_else(|x| {
			0
		}) != buffer.len()
		{
			println!("buffer underrun!");

			self.speaker.1.recover(&self.context, 32, true).unwrap_or_else(|x| {
				panic!("ERROR: {}", x)
			});

			if self.speaker.1.writei(&self.context, buffer).unwrap_or_else(|x| {
				0
			}) != buffer.len() {
				panic!("double buffer underrun!");
			}
		}
	}

	/// Generate & push data to speaker output.  When a new sample is
	/// needed, closure `generator` will be called.  This should be called
	/// in a loop.
	pub fn play(&mut self, generator: &mut FnMut() -> i16) {
		let left = self.left() as usize;
		let write = if left < self.speaker_buffer.len() {
			self.speaker_buffer.len() - left
		} else { 0 };

		for i in 0..write {
			self.speaker_buffer[i] = generator();
		}

		self.push(&self.speaker_buffer[..write]);
	}

	#[cfg(feature = "microphone")]
	/// Pull data from the microphone input.
	pub fn pull(&self, buffer: &mut [i16]) -> usize {
		self.microphone.readi(&self.context, buffer).unwrap_or(0)
	}

	/// Get the number of samples left in the buffer.
	#[cfg(feature = "speaker")]
	fn left(&self) -> i64 {
		self.speaker.0 - self.speaker.1.status(&self.context).unwrap().get_avail(&self.context)
	}
}
