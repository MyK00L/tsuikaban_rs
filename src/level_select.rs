use crate::game::Game;
use crate::res::Res;
use crate::state::*;
use macroquad::*;

const NUM_LEVELS: usize = 15;

pub struct LevelSelect<'a> {
	res: &'a Res,
}
impl<'a> LevelSelect<'a> {
	pub fn new(res: &'a Res) -> Self {
		LevelSelect { res: res }
	}
}
impl<'a> State<'a> for LevelSelect<'a> {
	fn update(&'a mut self) -> Transition {
		let mut ret = Transition::POP(0);
		draw_window(
			hash!(),
			vec2(0., 0.),
			vec2(screen_width(), screen_height()),
			WindowParams {
				titlebar: false,
				..Default::default()
			},
			|ui| {
				for i in 0..15 {
					if ui.button(None, &format!("{}", i)) {
						//ret = Transition::PUSH(Box::<dyn State+'a>::new(
						//	Game::new(include_str!("../res/levels/level10.txt"), self.res).unwrap()));
						return;
					}
				}
			},
		);
		Transition::PUSH(Box/*::<dyn State+'a>*/::new(Game::new(include_str!("../res/levels/level10.txt"), self.res).unwrap()))
		//Transition::POP(0)
	}
	fn draw(&mut self) {}
}
