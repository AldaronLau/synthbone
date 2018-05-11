extern crate alsa;
extern crate sample;

// mod ;

struct Player {
	pcm: alsa::pcm::PCM,
}

impl Player {
	pub fn new() -> Self {
		let pcm = alsa::pcm::PCM::new("default", alsa::Direction::Playback, true).unwrap();

		{ // TODO: when next Rust version comes out, get rid of this block - lifetime issue
			// Set hardware parameters: 44100 Hz / Mono / 16 bit
			let hwp = alsa::pcm::HwParams::any(&pcm).unwrap();
			hwp.set_channels(1).unwrap();
			hwp.set_rate(44100, alsa::ValueOr::Nearest).unwrap();
			hwp.set_format(alsa::pcm::Format::s16()).unwrap();
			hwp.set_access(alsa::pcm::Access::RWInterleaved).unwrap();
			pcm.hw_params(&hwp).unwrap();

			// Make sure we don't start the stream too early
			let hwp = pcm.hw_params_current().unwrap();
			let swp = pcm.sw_params_current().unwrap();
			swp.set_start_threshold(hwp.get_buffer_size().unwrap() - hwp.get_period_size().unwrap()).unwrap();
			pcm.sw_params(&swp).unwrap();
		}

		Player { pcm }
	}

	pub fn play(&self, buffer: &[i16]) {
		let io = self.pcm.io_i16().unwrap();
		io.writei(buffer).unwrap();
	}
}

struct Capturer {
	pcm: alsa::pcm::PCM,
}

impl Capturer {
	pub fn new() -> Self {
		let pcm = alsa::pcm::PCM::new("default", alsa::Direction::Capture, false).unwrap();

		{ // TODO: when next Rust version comes out, get rid of this block - lifetime issue
			// Set hardware parameters: 44100 Hz / Mono / 16 bit
			let hwp = alsa::pcm::HwParams::any(&pcm).unwrap();
			hwp.set_channels(1).unwrap();
			hwp.set_rate(44100, alsa::ValueOr::Nearest).unwrap();
			hwp.set_format(alsa::pcm::Format::s16()).unwrap();
			hwp.set_access(alsa::pcm::Access::RWInterleaved).unwrap();
			pcm.hw_params(&hwp).unwrap();

			// Make sure we don't start the stream too early
			let hwp = pcm.hw_params_current().unwrap();
			let swp = pcm.sw_params_current().unwrap();
			swp.set_start_threshold(hwp.get_buffer_size().unwrap() - hwp.get_period_size().unwrap()).unwrap();
			pcm.sw_params(&swp).unwrap();
		}

		Capturer { pcm }
	}

	pub fn capture(&self, buffer: &mut [i16]) {
		let io = self.pcm.io_i16().unwrap();
		io.readi(buffer).unwrap();
	}
}

fn main() {
//	use alsa::pcm::{ State};

	let player = Player::new();
	let capturer = Capturer::new();

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

	// Make a sine wave
//	let mut buf = [0i16; 1024];
//	for (i, a) in buf.iter_mut().enumerate() {
//	    *a = ((i as f32 * 2.0 * ::std::f32::consts::PI / 128.0).sin() * 8192.0) as i16
//	}

	// Play it back for 2 seconds.
//	for _ in 0..2*44100/1024 {
//	    assert_eq!(io.writei(&buf[..]).unwrap(), 1024);
//	}

	// In case the buffer was larger than 2 seconds, start the stream manually.
	// if pcm.state() != State::Running { pcm.start().unwrap() };
	// Wait for the stream to finish playback.
	// pcm.drain().unwrap();
}
