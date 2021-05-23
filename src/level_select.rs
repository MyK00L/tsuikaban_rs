use crate::game::Game;
use crate::opts::Opts;
use crate::state::*;
use macroquad::prelude::*;
use macroquad::ui::root_ui;
use macroquad::*;

pub struct LevelSelect {}
impl LevelSelect {
	pub fn new() -> Self {
		LevelSelect {}
	}
}
impl State for LevelSelect {
	fn draw_update(&mut self, o: &mut Opts) -> Vec<Option<Box<dyn State>>> {
		if is_key_pressed(KeyCode::Escape) {
			return vec![None];
		}
		let mut ret = Vec::<Option<Box<dyn State>>>::new();
		root_ui().window(
			hash!(),
			vec2(-1.0, -1.0),
			vec2(screen_width() + 2.0, screen_height() + 2.0),
			|ui| {
				for i in 0..o.unlocked {
					if ui.button(None, &i.to_string()) {
						ret.push(Some(Box::new(Game::new(i).unwrap())));
						return;
					}
				}
				ui.separator();
				for i in o.unlocked..super::LVLS.len() {
					if ui.button(None, &i.to_string()) {
						return;
					}
				}
				ui.separator();
			},
		);
		ret
	}
}
