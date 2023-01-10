#[cfg(feature = "save-options")]
use {
	bincode::{config, Decode, Encode},
	std::fs::File,
};

#[cfg(feature = "save-options")]
const FILENAME: &str = ".config.bin";

#[derive(Debug)]
#[cfg_attr(feature = "save-options", derive(Encode, Decode))]
pub struct Opts {
	pub clear: [Option<usize>; super::LVLS.len()],
	pub unlock_all_levels: bool,
	pub colorblind_mode: bool,
	pub palette: [[u8; 4]; 6],
}
impl Default for Opts {
	fn default() -> Self {
		Opts {
			clear: [None; super::LVLS.len()],
			unlock_all_levels: false,
			colorblind_mode: false,
			palette: [
				[255, 0, 0, 255],
				[255, 228, 32, 255],
				[32, 228, 255, 255],
				[255, 32, 228, 255],
				[15, 78, 228, 255],
				[64, 228, 0, 255],
			],
		}
	}
}
#[cfg(feature = "save-options")]
impl Opts {
	pub fn new() -> Self {
		if let Ok(mut file) = File::open(FILENAME) {
			bincode::decode_from_std_read(&mut file, config::standard()).unwrap_or_default()
		} else {
			Opts::default()
		}
	}
	pub fn save(&self) {
		let mut file = File::create(FILENAME).unwrap();
		bincode::encode_into_std_write(self, &mut file, config::standard()).unwrap();
	}
}
#[cfg(not(feature = "save-options"))]
impl Opts {
	pub fn new() -> Self {
		Self::default()
	}
	pub fn save(&self) {}
}
