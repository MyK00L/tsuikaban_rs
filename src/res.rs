use macroquad::*;

pub struct Res {
	pub floor: Texture2D,
	pub wall: Texture2D,
	pub door: Texture2D,
	pub cube: Texture2D,
	pub player: Texture2D,
}
impl Res {
	pub fn new() -> Self {
		let floor_image = image::load_from_memory_with_format(
			std::include_bytes!("../res/ground.png"),
			image::ImageFormat::PNG,
		)
		.unwrap()
		.into_rgba();
		let floor_f = load_texture_from_image(&Image {
			bytes: floor_image.into_raw(),
			width: 64 as u16,
			height: 64 as u16,
		});

		let wall_image = image::load_from_memory_with_format(
			std::include_bytes!("../res/wall.png"),
			image::ImageFormat::PNG,
		)
		.unwrap()
		.into_rgba();
		let wall_f = load_texture_from_image(&Image {
			bytes: wall_image.into_raw(),
			width: 64 as u16,
			height: 64 as u16,
		});

		let door_image = image::load_from_memory_with_format(
			std::include_bytes!("../res/door.png"),
			image::ImageFormat::PNG,
		)
		.unwrap()
		.into_rgba();
		let door_f = load_texture_from_image(&Image {
			bytes: door_image.into_raw(),
			width: 64 as u16,
			height: 64 as u16,
		});

		let cube_image = image::load_from_memory_with_format(
			std::include_bytes!("../res/cube.png"),
			image::ImageFormat::PNG,
		)
		.unwrap()
		.into_rgba();
		let cube_f = load_texture_from_image(&Image {
			bytes: cube_image.into_raw(),
			width: 64 as u16,
			height: 64 as u16,
		});

		let player_image = image::load_from_memory_with_format(
			std::include_bytes!("../res/player.png"),
			image::ImageFormat::PNG,
		)
		.unwrap()
		.into_rgba();
		let player_f = load_texture_from_image(&Image {
			bytes: player_image.into_raw(),
			width: 64 as u16,
			height: 64 as u16,
		});

		//let wall_f = load_texture("res/wall.png");
		//let door_f = load_texture("res/door.png");
		//let cube_f = load_texture("res/cube.png");
		//let player_f = load_texture("res/player.png");
		Res {
			floor: floor_f,
			wall: wall_f,
			door: door_f,
			cube: cube_f,
			player: player_f,
		}
	}
}
