use alsa;

fn set_settings(pcm: &alsa::pcm::PCM) {
	// Set hardware parameters: 44100 Hz / Mono / 16 bit
	let hwp = alsa::pcm::HwParams::any(pcm).unwrap();
	hwp.set_channels(1).unwrap();
	hwp.set_rate(44100, alsa::ValueOr::Nearest).unwrap();
	hwp.set_format(alsa::pcm::Format::s16()).unwrap();
	hwp.set_access(alsa::pcm::Access::RWInterleaved).unwrap();
	pcm.hw_params(&hwp).unwrap();

	// Make sure we don't start the stream too early
	let hwp = pcm.hw_params_current().unwrap();
	let swp = pcm.sw_params_current().unwrap();
	swp.set_start_threshold(
			hwp.get_buffer_size().unwrap()
				- hwp.get_period_size().unwrap()
		)
		.unwrap();
	pcm.sw_params(&swp).unwrap();
}

pub struct Player {
	pcm: alsa::pcm::PCM,
}

impl Player {
	pub fn new() -> Self {
		let pcm = alsa::pcm::PCM::new("default",
			alsa::Direction::Playback, true).unwrap();
		set_settings(&pcm);
		Player { pcm }
	}

	pub fn play(&self, buffer: &[i16]) {
		let io = self.pcm.io_i16().unwrap();
		io.writei(buffer).unwrap();
	}
}

pub struct Capturer {
	pcm: alsa::pcm::PCM,
}

impl Capturer {
	pub fn new() -> Self {
		let pcm = alsa::pcm::PCM::new("default",
			alsa::Direction::Capture, false).unwrap();
		set_settings(&pcm);
		Capturer { pcm }
	}

	pub fn capture(&self, buffer: &mut [i16]) {
		let io = self.pcm.io_i16().unwrap();
		io.readi(buffer).unwrap();
	}
}
