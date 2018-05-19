extern crate libc;
extern crate nix;

mod alsa;

fn set_settings(context: &alsa::Context, pcm: &alsa::pcm::PCM) {
	// Set hardware parameters: 44100 Hz / Mono / 16 bit
	let hwp = alsa::pcm::HwParams::any(context, pcm).unwrap();
	hwp.set_channels(context, 1).unwrap();
	hwp.set_rate(context, 44100, alsa::ValueOr::Nearest).unwrap();
	hwp.set_format(context, alsa::pcm::Format::s16()).unwrap();
	hwp.set_access(context, alsa::pcm::Access::RWInterleaved).unwrap();
	pcm.hw_params(context, &hwp).unwrap();
	hwp.drop(context);
}

pub struct AudioManager {
	context: alsa::Context,
	speaker: alsa::pcm::PCM, // TODO: call drop(), it isn't being called rn.
	microphone: alsa::pcm::PCM, // TODO: call drop(), it isn't being called rn.
}

impl AudioManager {
	/// Create a new `AudioManager`.
	pub fn new() -> Self {
		let context = alsa::Context::new();

		let speaker = {
			let pcm = alsa::pcm::PCM::new(&context,
				"default", //"bluealsa:HCI=hci0,DEV=08:EB:ED:EE:A7:47,PROFILE=a2dp",
				alsa::Direction::Playback).unwrap();
			set_settings(&context, &pcm);
			// Make sure we don't start the stream too early
			{
				let hwp = pcm.hw_params_current(&context).unwrap();

				println!("{} {}",
					hwp.get_buffer_size(&context).unwrap(),
					hwp.get_period_size(&context).unwrap());

				println!("PC: {}", hwp.get_channels(&context).unwrap());
				println!("PR: {}", hwp.get_rate(&context).unwrap());

				hwp.drop(&context);
			}
			pcm
		};

		let microphone = {
			let pcm = alsa::pcm::PCM::new(&context, "hw:1,0",
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

		speaker.prepare(&context);
		microphone.start(&context);

		let am = AudioManager {
			context,
			speaker,
			microphone,
		};

		am
	}

	/// Push data to the speaker output.
	pub fn push(&self, buffer: &[i16]) {
		if self.speaker.writei(&self.context, buffer).unwrap_or_else(|x| {
			0
		}) != buffer.len()
		{
			self.speaker.recover(&self.context, 32, true).unwrap_or_else(|x| {
				panic!("ERROR: {}", x)
			});

			if self.speaker.writei(&self.context, buffer).unwrap_or_else(|x| {
				0
			}) != buffer.len() {
				panic!("double buffer underrun!");
			}

			println!("buffer underrun!");
		}
	}

	/// Pull data from the microphone input.
	pub fn pull(&self, buffer: &mut [i16]) -> usize {
		let mut avail = self.microphone.avail(&self.context) as usize;

		if avail > buffer.len()  {
			avail = buffer.len();
		}

		let buffer = &mut buffer[..avail];

		self.microphone.readi(&self.context, buffer).unwrap_or(0)
	}
}

//	pub fn delay(&self, ad: &AudioDevice) -> isize {
//		self.pcm.status(&self.context).unwrap()
//			.get_avail(&self.context) as isize
//	}

//	pub fn delay(&self, ad: &AudioDevice) -> isize {
//		self.pcm.status(&self.context).unwrap()
//			.get_avail(&self.context) as isize
//	}
