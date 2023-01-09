#[cfg(feature = "save-options")]
use {
	serde::{Deserialize, Serialize},
	std::fs::File,
	std::io::{Read, Write},
};

#[cfg(feature = "save-options")]
const FILENAME: &str = ".config.bin";

#[derive(Debug)]
#[cfg_attr(feature = "save-options", derive(Serialize, Deserialize))]
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
		if let Ok(mut f) = File::open(FILENAME) {
			let mut buf = Vec::new();
			f.read_to_end(&mut buf).unwrap();
			bincode::deserialize(&buf).unwrap_or_default()
		} else {
			Opts::default()
		}
	}
	pub fn save(&self) {
		let mut file = File::create(FILENAME).unwrap();
		file.write_all(&bincode::serialize(&self).unwrap()).unwrap();
	}
}
#[cfg(not(feature = "save-options"))]
impl Opts {
	pub fn new() -> Self {
		Self::default()
	}
	pub fn save(&self) {}
}
