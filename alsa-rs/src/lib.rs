extern crate libc;
extern crate nix;

mod alsa;

fn set_settings(ad: &AudioDevice, pcm: &alsa::pcm::PCM) {
	// Set hardware parameters: 44100 Hz / Mono / 16 bit
	let hwp = alsa::pcm::HwParams::any(&ad.context, pcm).unwrap();
	hwp.set_channels(&ad.context, 1).unwrap();
	hwp.set_rate(&ad.context, 44100, alsa::ValueOr::Nearest).unwrap();
	hwp.set_format(&ad.context, alsa::pcm::Format::s16()).unwrap();
	hwp.set_access(&ad.context, alsa::pcm::Access::RWInterleaved).unwrap();
//	hwp.set_period_size(&ad.context, 2048).unwrap();
	pcm.hw_params(&ad.context, &hwp).unwrap();
	hwp.drop(&ad.context);
}

pub struct AudioDevice {
	context: alsa::Context,
}

impl AudioDevice {
	pub fn new() -> Self {
		AudioDevice {
			context: alsa::Context::new(),
		}
	}
}

pub struct Player {
//	pcm: alsa::pcm::PCM, // TODO: call drop(), it isn't being called rn.
	io: alsa::pcm::IO<i16>,
}

impl Player {
	pub fn new(ad: &AudioDevice) -> Self {
		let pcm = alsa::pcm::PCM::new(&ad.context,
			"default", //"bluealsa:HCI=hci0,DEV=08:EB:ED:EE:A7:47,PROFILE=a2dp",
			alsa::Direction::Playback).unwrap();
		set_settings(ad, &pcm);
		pcm.prepare(&ad.context);
		// Make sure we don't start the stream too early
		{
			let hwp = pcm.hw_params_current(&ad.context).unwrap();
//			let swp = pcm.sw_params_current(&ad.context).unwrap();
//			swp.set_start_threshold(&ad.context,
//					hwp.get_buffer_size(&ad.context).unwrap()
//						- hwp.get_period_size(&ad.context).unwrap()
//				)
//				.unwrap();

			println!("{} {}",
				hwp.get_buffer_size(&ad.context).unwrap(),
				hwp.get_period_size(&ad.context).unwrap());

			println!("PC: {}", hwp.get_channels(&ad.context).unwrap());
			println!("PR: {}", hwp.get_rate(&ad.context).unwrap());

//			pcm.sw_params(&ad.context, &swp).unwrap();
//			swp.drop(&ad.context);
			hwp.drop(&ad.context);
		}
		let io = pcm.io_i16(&ad.context).unwrap();
		let this = Player { io };

		this.play(ad, vec![0; 2048].as_slice());

		this
	}

	pub fn play(&self, ad: &AudioDevice, buffer: &[i16]) {
		if self.io.writei(&ad.context, buffer).unwrap_or_else(|x| {
//			println!("{}", x);
			0
		}) != buffer.len()
		{
			self.io.recover(&ad.context, 32, true).unwrap_or_else(|x| {
				panic!("ERROR: {}", x)
			});

//			self.io.prepare(&ad.context);

			if self.io.writei(&ad.context, buffer).unwrap_or_else(|x| {
				0
			}) != buffer.len() {
				panic!("double buffer underrun!");
			}




//			panic!("buffer underrun!");
			println!("buffer underrun!");
		}
	}

	pub fn delay(&self, ad: &AudioDevice) -> isize {
		self.io.status(&ad.context).unwrap()
			.get_avail(&ad.context) as isize
	}
}

pub struct Capturer {
//	pcm: alsa::pcm::PCM, // TODO: call drop(), it isn't being called rn.
	io: alsa::pcm::IO<i16>,
}

impl Capturer {
	pub fn new(ad: &AudioDevice) -> Self {
		// Do not this make non-blocking - it creates a million threads
		// when it falls behind.
		let pcm = alsa::pcm::PCM::new(&ad.context, "hw:1,0",
			alsa::Direction::Capture).unwrap();
		set_settings(ad, &pcm);
		{
			let hwp = pcm.hw_params_current(&ad.context).unwrap();
			println!("CC: {}", hwp.get_channels(&ad.context).unwrap());
			println!("CR: {}", hwp.get_rate(&ad.context).unwrap());
			hwp.drop(&ad.context);
		}
//		pcm.prepare(&ad.context);
		let io = pcm.io_i16(&ad.context).unwrap();
		Capturer { io }
	}

	pub fn capture(&self, ad: &AudioDevice, buffer: &mut [i16]) -> usize {
//		if self.delay(ad) != 0 {
			self.io.readi(&ad.context, buffer).unwrap_or(0)
//		} else {
//			0
//		}
	}

	pub fn delay(&self, ad: &AudioDevice) -> isize {
		self.io.status(&ad.context).unwrap()
			.get_avail(&ad.context) as isize
	}
}
