use std::boxed::Box;

pub trait State {
	fn draw_update(&mut self, o: &mut crate::opts::Opts) -> Vec<Option<Box<dyn State>>>;
}
