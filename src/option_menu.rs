use crate::opts::Opts;
use crate::state::*;
use macroquad::prelude::*;
use macroquad::ui::root_ui;
use macroquad::*;

pub struct OptionMenu {
	unlock_all_bool: bool,
	colorblind_bool: bool,
	colors: [[f32; 3]; 6],
}
impl OptionMenu {
	pub fn new(o: &Opts) -> Self {
		let mut colors = [[0f32; 3]; 6];
		for (i, col) in colors.iter_mut().enumerate() {
			col[0] = o.palette[i][0] as f32;
			col[1] = o.palette[i][1] as f32;
			col[2] = o.palette[i][2] as f32;
		}
		OptionMenu {
			unlock_all_bool: o.unlock_all_levels,
			colorblind_bool: o.colorblind_mode,
			colors,
		}
	}
}
impl State for OptionMenu {
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
				if ui.button(None, "SAVE") {
					o.unlock_all_levels = self.unlock_all_bool;
					o.colorblind_mode = self.colorblind_bool;
					for i in 0..6 {
						for j in 0..3 {
							o.palette[i][j] = self.colors[i][j] as u8;
						}
					}
					o.save();
					return;
				}
				ui.separator();
				if ui.button(None, "BACK") {
					ret.push(None);
					return;
				}
				ui.separator();
				ui.checkbox(hash!(), "unlock all levels", &mut self.unlock_all_bool);
				ui.checkbox(hash!(), "colorblind mode", &mut self.colorblind_bool);
				ui.separator();
				for i in 0..6 {
					ui.tree_node(hash!(i * 4), &format!("color {i}"), |ui| {
						ui.slider(
							hash!(i * 4 + 1),
							"RED",
							0f32..255f32,
							&mut self.colors[i][0],
						);
						ui.slider(
							hash!(i * 4 + 2),
							"GREEN",
							0f32..255f32,
							&mut self.colors[i][1],
						);
						ui.slider(
							hash!(i * 4 + 3),
							"BLUE",
							0f32..255f32,
							&mut self.colors[i][2],
						);
					});
				}
				ui.separator();
				if ui.button(None, "LOAD DEFAULTS") {
					let tmp = o.clear;
					*o = Opts::default();
					o.clear = tmp;
					*self = OptionMenu::new(o);
					//return;
				}
			},
		);
		ret
	}
}
