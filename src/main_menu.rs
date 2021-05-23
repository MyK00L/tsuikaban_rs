use crate::level_select::LevelSelect;
use crate::option_menu::OptionMenu;
use crate::opts::Opts;
use crate::state::*;
use macroquad::prelude::*;
use macroquad::ui::root_ui;
use macroquad::*;

pub struct MainMenu {}
impl MainMenu {
	pub fn new() -> Self {
		MainMenu {}
	}
}
impl State for MainMenu {
	fn draw_update(&mut self, o: &mut Opts) -> Vec<Option<Box<dyn State>>> {
		let mut ret = Vec::<Option<Box<dyn State>>>::new();
		root_ui().window(
			hash!(),
			vec2(-1.0, -1.0),
			vec2(screen_width() + 2.0, screen_height() + 2.0),
			|ui| {
				if ui.button(None, "LEVEL SELECT") {
					ret.push(Some(Box::new(LevelSelect::new())));
					return;
				}
				if ui.button(None, "OPTIONS") {
					ret.push(Some(Box::new(OptionMenu::new(o))));
					return;
				}
				if ui.button(None, "EXIT GAME") {
					ret.push(None);
					return;
				}

				ui.separator();
			},
		);
		ret
	}
}
