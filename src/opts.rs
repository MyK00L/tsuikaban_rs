use bincode::{config, Decode, Encode};
use quad_wasmnastics::storage;

#[derive(Debug, Encode, Decode)]
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
impl Opts {
	pub fn new() -> Self {
		if let Ok(bin) = storage::load_from(&Opts::get_location()) {
			bincode::decode_from_slice(&bin, config::standard())
				.unwrap_or_default()
				.0
		} else {
			Self::default()
		}
	}
	pub fn save(&self) {
		if let Err(e) = storage::save_to(
			bincode::encode_to_vec(self, config::standard()).unwrap_or_default(),
			&Opts::get_location(),
		) {
			eprintln!("{}", e);
		}
	}
	fn get_location() -> storage::Location {
		storage::Location {
			bin_name: "tsuikaban_rs".to_string(),
			version: "1".to_string(),
			profile: "data".to_string(),
		}
	}
}
