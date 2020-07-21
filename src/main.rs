mod game;
mod level_select;
mod res;
mod state;
use macroquad::{self as mq, *};
use state::*;
use std::boxed::Box;
	
#[macroquad::main("Tsuikaban")]
async fn main() {
	let res: res::Res = res::Res::new();
	let mut stack = Vec::<Box<dyn State>>::new();
	stack.push(Box::new(
		game::Game::new(include_str!("../res/levels/level10.txt"), &res).unwrap(),
	));
	//stack.push(Box::new(level_select::LevelSelect::new(&res)));
	loop {
		let n = stack.len();
		//stack[n-1].draw();
		let tr = stack[n-1].update();
		match tr {
			Transition::POP(x) => {
				for _ in 0..x {
					stack.pop();
				}
			}
			Transition::PUSH(x) => {
				stack.push(x);
			}
		}
		next_frame().await;
	}
}
