use crate::option_menu::OptionMenu;
use crate::opts::Opts;
use crate::state::*;
use macroquad::*;

pub struct Pause {}
impl Pause {
	pub fn new() -> Self {
		Pause {}
	}
}
impl State for Pause {
	fn draw_update(&mut self, o: &mut Opts) -> Vec<Option<Box<dyn State>>> {
		if is_key_pressed(KeyCode::Escape) {
			return vec![None];
		}
		let mut ret = Vec::<Option<Box<dyn State>>>::new();
		draw_window(
			hash!(),
			vec2(-1.0, -1.0),
			vec2(screen_width() + 2.0, screen_height() + 2.0),
			WindowParams {
				label: "".to_string(),
				movable: false,
				close_button: false,
				titlebar: false,
			},
			|ui| {
				if ui.button(None, "RETURN TO GAME") {
					ret.push(None);
					return;
				}
				if ui.button(None, "OPTIONS") {
					ret.push(Some(Box::new(OptionMenu::new(o))));
					return;
				}
				ui.separator();
				if ui.button(None, "RETURN TO LEVEL SELECT") {
					ret.push(None);
					ret.push(None);
					return;
				}
				if ui.button(None, "RETURN TO MAIN MENU") {
					ret.push(None);
					ret.push(None);
					ret.push(None);
					return;
				}
			},
		);
		ret
	}
}
