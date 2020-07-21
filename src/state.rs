use std::boxed::Box;

pub enum Transition<'a> {
	POP(usize),
	PUSH(Box<dyn State<'a> + 'a>),
}

pub trait State<'a> {
	fn update(&'a mut self) -> Transition;
	fn draw(&mut self);
}
