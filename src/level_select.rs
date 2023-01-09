use crate::game::Game;
use crate::opts::Opts;
use crate::state::*;
use macroquad::math::Vec2;
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
				let unlocked = if o.unlock_all_levels {
					super::LVLS.len()
				} else {
					o.clear
						.iter()
						.position(|x| x.is_none())
						.unwrap_or(super::LVLS.len())
				};
				let mut row: u32 = 0;
				let mut col: u32 = 0;
				// TODO: remove magic numbers
				const BW: u32 = 53;
				const BH: u32 = 56;
				const MARGIN: u32 = 8;
				for i in 0..super::LVLS.len() {
					if screen_width() < (col * (BW + MARGIN) + MARGIN + BW + MARGIN) as f32 {
						row += 1;
						col = 0;
					}
					if ui.button(
						Some(Vec2 {
							x: (col * (BW + MARGIN) + MARGIN) as f32,
							y: (row * (BH + MARGIN) + MARGIN) as f32,
						}),
						if unlocked >= i {
							format!("{i:0>2}")
						} else {
							"??".to_string()
						},
					) {
						if unlocked >= i {
							ret.push(Some(Box::new(Game::new(i).unwrap())));
						}
						return;
					}
					ui.label(
						Some(Vec2 {
							x: (col * (BW + MARGIN) + MARGIN) as f32,
							y: (row * (BH + MARGIN) + MARGIN) as f32,
						}),
						&o.clear[i].map(|x| x.to_string()).unwrap_or("".to_string()),
					);
					col += 1;
				}
			},
		);
		ret
	}
}
