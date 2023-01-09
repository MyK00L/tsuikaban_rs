use crate::opts::Opts;
use crate::pause::Pause;
use crate::state::*;
use macroquad::prelude::*;

const ANIMATION_FRAMES: usize = 6;
const CUBE_SIZE: f32 = 0.9;

#[derive(Clone, Copy, Debug)]
enum Direction {
	Left,
	Right,
	Up,
	Down,
}
fn step(p: (usize, usize), dir: Direction) -> (usize, usize) {
	match dir {
		Direction::Left => (p.0, p.1.wrapping_sub(1)),
		Direction::Right => (p.0, p.1 + 1),
		Direction::Up => (p.0.wrapping_sub(1), p.1),
		Direction::Down => (p.0 + 1, p.1),
	}
}
fn unstep(p: (usize, usize), dir: Direction) -> (usize, usize) {
	match dir {
		Direction::Left => (p.0, p.1 + 1),
		Direction::Right => (p.0, p.1.wrapping_sub(1)),
		Direction::Up => (p.0 + 1, p.1),
		Direction::Down => (p.0.wrapping_sub(1), p.1),
	}
}
#[derive(Clone, Copy, Debug, Default)]
struct Cube {
	col: usize,
	n: i16,
}
#[derive(Clone, Copy, Debug)]
enum Tile {
	Floor,
	Wall,
	Door,
	Cube(Cube),
}
pub struct Game {
	lvl_num: usize,
	pos: (usize, usize),
	m: Vec<Vec<Tile>>,
	undo_stack: Vec<(Direction, usize, Option<Cube>)>,
	anime: usize,
}
impl Game {
	pub fn new(lvl_num: usize) -> Result<Self, Box<dyn std::error::Error>> {
		let mut tokens = super::LVLS[lvl_num].split_whitespace().map(String::from);
		let height: usize = tokens.next().ok_or("malformatted level file")?.parse()?;
		let width: usize = tokens.next().ok_or("malformatted level file")?.parse()?;
		let mut m = vec![vec![Tile::Floor; width]; height];
		for row in m.iter_mut() {
			let s = tokens.next().ok_or("malformatted level file")?;
			for (j, c) in s.chars().enumerate() {
				row[j] = match c {
					'.' => Tile::Floor,
					'#' => Tile::Wall,
					'D' => Tile::Door,
					_ => Tile::Floor,
				}
			}
		}
		let mut pos = (0usize, 0usize);
		pos.0 = tokens.next().ok_or("malformatted level file")?.parse()?;
		pos.1 = tokens.next().ok_or("malformatted level file")?.parse()?;
		let ncubes: usize = tokens.next().ok_or("malformatted level file")?.parse()?;
		for _ in 0..ncubes {
			let y: usize = tokens.next().ok_or("malformatted level file")?.parse()?;
			let x: usize = tokens.next().ok_or("malformatted level file")?.parse()?;
			let c: usize = tokens.next().ok_or("malformatted level file")?.parse()?;
			let num: i16 = tokens.next().ok_or("malformatted level file")?.parse()?;
			m[y][x] = Tile::Cube(Cube { n: num, col: c });
		}
		Ok(Game {
			lvl_num,
			pos,
			m,
			undo_stack: vec![],
			anime: 0,
		})
	}
	fn is_inside(&self, p: (usize, usize)) -> bool {
		p.0 < self.m.len() && p.1 < self.m[0].len()
	}
	fn mov(&mut self, dir: Direction) {
		let mut p = step(self.pos, dir);
		if self.is_inside(p) {
			match self.m[p.0][p.1] {
				Tile::Floor | Tile::Door => {
					self.anime = ANIMATION_FRAMES;
					self.pos = p;
					self.undo_stack.push((dir, 0, None));
				}
				Tile::Cube(x) => {
					let mut last_col = x.col;
					p = step(p, dir);
					let mut moved_cubes = 1usize;
					while self.is_inside(p)
						&& match self.m[p.0][p.1] {
							Tile::Cube(x) => {
								let ret = x.col != last_col;
								last_col = x.col;
								ret
							}
							_ => false,
						} {
						moved_cubes += 1;
						p = step(p, dir);
					}
					if self.is_inside(p) {
						match self.m[p.0][p.1] {
							Tile::Floor => {
								self.undo_stack.push((dir, moved_cubes, None));
								while p != self.pos {
									let np = unstep(p, dir);
									self.m[p.0][p.1] = self.m[np.0][np.1];
									p = np;
								}
								self.anime = ANIMATION_FRAMES;
								self.pos = step(self.pos, dir);
							}
							Tile::Cube(x) => {
								self.undo_stack.push((dir, moved_cubes, Some(x)));
								let np = unstep(p, dir);
								if let Tile::Cube(y) = self.m[np.0][np.1] {
									self.m[p.0][p.1] = match x.n + y.n {
										0 => Tile::Floor,
										num => Tile::Cube(Cube { col: x.col, n: num }),
									}
								}
								p = np;
								while p != self.pos {
									let np = unstep(p, dir);
									self.m[p.0][p.1] = self.m[np.0][np.1];
									p = np;
								}
								self.anime = ANIMATION_FRAMES;
								self.pos = step(self.pos, dir);
							}
							_ => {}
						}
					}
				}
				_ => {}
			}
		}
	}
	fn undo(&mut self) {
		if let Some(st) = self.undo_stack.pop() {
			let mut p = self.pos;
			self.pos = unstep(self.pos, st.0);
			for _ in 0..st.1 {
				let np = step(p, st.0);
				self.m[p.0][p.1] = self.m[np.0][np.1];
				p = np;
			}
			let np = p;
			p = unstep(p, st.0);
			if let Some(x) = st.2 {
				match self.m[p.0][p.1] {
					Tile::Cube(y) => {
						self.m[np.0][np.1] = Tile::Cube(x);
						self.m[p.0][p.1] = Tile::Cube(Cube {
							col: x.col,
							n: y.n - x.n,
						});
					}
					_ => {
						self.m[np.0][np.1] = Tile::Cube(x);
						self.m[p.0][p.1] = Tile::Cube(Cube {
							col: x.col,
							n: -x.n,
						});
					}
				}
			} else if st.1 > 0 {
				self.m[np.0][np.1] = Tile::Floor;
			}
		}
	}
	fn get_mouse_pos(&self) -> (usize, usize) {
		let scale =
			(screen_height() / self.m.len() as f32).min(screen_width() / self.m[0].len() as f32);
		let p = mouse_position();
		if p.0 < 0.0 || p.1 < 0.0 {
			(self.m.len(), self.m[0].len())
		} else {
			((p.1 / scale) as usize, (p.0 / scale) as usize)
		}
	}
}
fn step_fun(x: f32) -> f32 {
	x * x
}
impl State for Game {
	fn draw_update(&mut self, o: &mut Opts) -> Vec<Option<Box<dyn State>>> {
		clear_background(BLACK);
		let scale =
			(screen_height() / self.m.len() as f32).min(screen_width() / self.m[0].len() as f32);
		let mut delta = (0.0, 0.0);
		if let Some(x) = self.undo_stack.last() {
			match x.0 {
				Direction::Left => {
					delta.1 = -step_fun(self.anime as f32 / ANIMATION_FRAMES as f32);
				}
				Direction::Right => {
					delta.1 = step_fun(self.anime as f32 / ANIMATION_FRAMES as f32);
				}
				Direction::Up => {
					delta.0 = -step_fun(self.anime as f32 / ANIMATION_FRAMES as f32);
				}
				Direction::Down => {
					delta.0 = step_fun(self.anime as f32 / ANIMATION_FRAMES as f32);
				}
			}
		}
		for i in 0..self.m.len() {
			for j in 0..self.m[0].len() {
				match self.m[i][j] {
					Tile::Floor | Tile::Cube(_) => {
						draw_rectangle(j as f32 * scale, i as f32 * scale, scale, scale, WHITE);
					}
					Tile::Door => {
						draw_circle(
							(j as f32 + 0.5) * scale,
							(i as f32 + 0.5) * scale,
							scale * 0.40,
							WHITE,
						);
					}
					_ => {}
				}
			}
		}
		if self.anime > 0 {
			if let Some(x) = self.undo_stack.last() {
				if let Some(c) = x.2 {
					let mut p = self.pos;
					match x.0 {
						Direction::Left => {
							p.1 -= x.1;
						}
						Direction::Right => {
							p.1 += x.1;
						}
						Direction::Up => {
							p.0 -= x.1;
						}
						Direction::Down => {
							p.0 += x.1;
						}
					}
					draw_rectangle(
						(p.1 as f32 + (1.0 - CUBE_SIZE) / 2.0) * scale,
						(p.0 as f32 + (1.0 - CUBE_SIZE) / 2.0) * scale,
						scale * CUBE_SIZE,
						scale * CUBE_SIZE,
						o.palette[c.col].into(),
					);
					if let Tile::Floor = self.m[p.0][p.1] {
						draw_rectangle(
							(p.1 as f32 + (1.0 - CUBE_SIZE) / 2.0 - delta.1) * scale,
							(p.0 as f32 + (1.0 - CUBE_SIZE) / 2.0 - delta.0) * scale,
							scale * CUBE_SIZE,
							scale * CUBE_SIZE,
							o.palette[c.col].into(),
						);
					}
				}
			}
		}
		for i in 0..self.m.len() {
			for j in 0..self.m[0].len() {
				if let Tile::Cube(x) = self.m[i][j] {
					let to_animate = if let Some(x) = self.undo_stack.last() {
						match x.0 {
							Direction::Left => i == self.pos.0 && j < self.pos.1 && self.pos.1 - j <= x.1,
							Direction::Right => i == self.pos.0 && j > self.pos.1 && j - self.pos.1 <= x.1,
							Direction::Up => j == self.pos.1 && i < self.pos.0 && self.pos.0 - i <= x.1,
							Direction::Down => j == self.pos.1 && i > self.pos.0 && i - self.pos.0 <= x.1,
						}
					} else {
						false
					};
					let txt = x.n.to_string();
					let font_size = scale * 2.0 / 3.0;
					let text_size = measure_text(&txt, None, 1, font_size);
					if to_animate {
						draw_rectangle(
							(j as f32 + (1.0 - CUBE_SIZE) / 2.0 - delta.1) * scale,
							(i as f32 + (1.0 - CUBE_SIZE) / 2.0 - delta.0) * scale,
							scale * CUBE_SIZE,
							scale * CUBE_SIZE,
							o.palette[x.col].into(),
						);
						draw_text(
							&txt,
							(j as f32 + 0.5 - delta.1) * scale - text_size.width / 2.0,
							(i as f32 + 0.5 - delta.0) * scale + text_size.offset_y / 4.0,
							font_size,
							BLACK,
						);
					} else {
						draw_rectangle(
							(j as f32 + (1.0 - CUBE_SIZE) / 2.0) * scale,
							(i as f32 + (1.0 - CUBE_SIZE) / 2.0) * scale,
							scale * CUBE_SIZE,
							scale * CUBE_SIZE,
							o.palette[x.col].into(),
						);
						draw_text(
							&txt,
							(j as f32 + 0.5) * scale - text_size.width / 2.0,
							(i as f32 + 0.5) * scale + text_size.offset_y / 4.0,
							font_size,
							BLACK,
						);
					}
				}
			}
		}
		draw_circle(
			(self.pos.1 as f32 + 0.5 - delta.1) * scale,
			(self.pos.0 as f32 + 0.5 - delta.0) * scale,
			scale * 0.40,
			BLACK,
		);

		if is_key_pressed(KeyCode::Escape) {
			return vec![Some(Box::new(Pause::new()))];
		}
		if self.anime > 0 {
			self.anime -= 1;
			return vec![];
		}

		let win = matches!(self.m[self.pos.0][self.pos.1], Tile::Door);
		if win {
			if self.lvl_num + 1 == o.unlocked {
				o.unlocked += 1;
				o.save();
			}
			let mut ret = Vec::<Option<Box<dyn State>>>::new();
			ret.push(None);
			if self.lvl_num + 1 < super::LVLS.len() {
				ret.push(Some(Box::new(Game::new(self.lvl_num + 1).unwrap())));
			}
			return ret;
		}

		if is_key_pressed(KeyCode::U) {
			self.undo();
		} else if is_key_down(KeyCode::Right) {
			self.mov(Direction::Right);
		} else if is_key_down(KeyCode::Left) {
			self.mov(Direction::Left);
		} else if is_key_down(KeyCode::Up) {
			self.mov(Direction::Up);
		} else if is_key_down(KeyCode::Down) {
			self.mov(Direction::Down);
		} else if is_mouse_button_down(MouseButton::Left) {
			if self.get_mouse_pos() == step(self.pos, Direction::Right) {
				self.mov(Direction::Right);
			} else if self.get_mouse_pos() == step(self.pos, Direction::Left) {
				self.mov(Direction::Left);
			} else if self.get_mouse_pos() == step(self.pos, Direction::Up) {
				self.mov(Direction::Up);
			} else if self.get_mouse_pos() == step(self.pos, Direction::Down) {
				self.mov(Direction::Down);
			}
		}
		vec![]
	}
}
