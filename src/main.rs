mod game;
mod level_select;
mod main_menu;
mod option_menu;
mod opts;
mod pause;
mod state;
use macroquad::*;
use state::*;
use std::boxed::Box;

const LVLS: [&str; 19] = [
	include_str!("../lvls/level0.txt"),
	include_str!("../lvls/level1.txt"),
	include_str!("../lvls/level2.txt"),
	include_str!("../lvls/level3.txt"),
	include_str!("../lvls/level4.txt"),
	include_str!("../lvls/level5.txt"),
	include_str!("../lvls/level6.txt"),
	include_str!("../lvls/level7.txt"),
	include_str!("../lvls/level8.txt"),
	include_str!("../lvls/level9.txt"),
	include_str!("../lvls/level10.txt"),
	include_str!("../lvls/level11.txt"),
	include_str!("../lvls/level12.txt"),
	include_str!("../lvls/level13.txt"),
	include_str!("../lvls/level14.txt"),
	include_str!("../lvls/level15.txt"),
	include_str!("../lvls/level16.txt"),
	include_str!("../lvls/level17.txt"),
	include_str!("../lvls/level18.txt"),
];

fn main() {
	macroquad::Window::new("Tsuikaban", amain());
}

async fn amain() {
	let mut o = opts::Opts::new();
	set_ui_style(megaui::Style {
		//scroll_width: 64.0,
		button_background_focused: megaui::Color::from_rgb(31, 31, 31),
		button_background_focused_hovered: megaui::Color::from_rgb(63, 63, 63),
		button_background_focused_clicked: megaui::Color::from_rgb(127, 127, 127),
		button_background_inactive: megaui::Color::from_rgb(31, 31, 31),
		inactive_text: megaui::Color::from_rgb(255, 255, 255),
		focused_text: megaui::Color::from_rgb(255, 255, 255),
		scroll_multiplier: 16.0,
		margin: 16.0,
		margin_button: 16.0,
		window_background_focused: megaui::Color::from_rgb(0, 0, 0),
		window_background_inactive: megaui::Color::from_rgb(0, 0, 0),
		..Default::default()
	});
	let mut stack = Vec::<Box<dyn State>>::new();
	stack.push(Box::new(main_menu::MainMenu::new()));
	while !stack.is_empty() {
		let n = stack.len();
		for i in stack[n - 1].draw_update(&mut o) {
			match i {
				Some(x) => {
					stack.push(x);
				}
				None => {
					stack.pop();
				}
			}
		}
		next_frame().await;
	}
	o.save();
}
