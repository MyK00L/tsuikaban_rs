mod game;
mod level_select;
mod main_menu;
mod option_menu;
mod opts;
mod pause;
mod state;
use macroquad::prelude::*;
use macroquad::ui::root_ui;
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

#[macroquad::main("Tsuikaban")]
async fn main() {
	let mut o = opts::Opts::new();
	let skin = {
		let font_size = 24;

		let ds = root_ui()
			.style_builder()
			.color(Color::from_rgba(31, 31, 31, 255))
			.color_hovered(Color::from_rgba(63, 63, 63, 255))
			.color_clicked(Color::from_rgba(127, 127, 127, 255))
			.text_color(Color::from_rgba(255, 255, 255, 255))
			.font_size(font_size)
			.margin(macroquad::math::RectOffset {
				left: 1.0,
				right: 1.0,
				bottom: 1.0,
				top: 1.0,
			})
			.build();

		let button_style = root_ui()
			.style_builder()
			.color(Color::from_rgba(31, 31, 31, 255))
			.color_hovered(Color::from_rgba(63, 63, 63, 255))
			.color_clicked(Color::from_rgba(127, 127, 127, 255))
			.text_color(Color::from_rgba(255, 255, 255, 255))
			.font_size(font_size)
			.margin(macroquad::math::RectOffset {
				left: 16.0,
				right: 16.0,
				bottom: 16.0,
				top: 16.0,
			})
			.build();
		let window_style = root_ui()
			.style_builder()
			.color(Color::from_rgba(0, 0, 0, 255))
			.text_color(Color::from_rgba(255, 255, 255, 255))
			.font_size(font_size)
			.build();

		macroquad::ui::Skin {
			label_style: ds.clone(),
			button_style: button_style,
			tabbar_style: ds.clone(),
			window_style: window_style,
			editbox_style: ds.clone(),
			window_titlebar_style: ds.clone(),
			scrollbar_style: ds.clone(),
			scrollbar_handle_style: ds.clone(),
			checkbox_style: ds.clone(),
			group_style: ds.clone(),
			margin: 16.0f32,
			/*title_height: 1.0f32,
			scroll_width: 1.0f32,
			scroll_multiplier: 1.0f32,*/
			..root_ui().default_skin()
		}
	};
	root_ui().push_skin(&skin);

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
