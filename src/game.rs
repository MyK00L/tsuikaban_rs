use crate::res::Res;
use crate::state::*;
use macroquad::*;

#[derive(Clone, Copy)]
enum Direction {
	LEFT,
	RIGHT,
	UP,
	DOWN,
}
fn step(p: (usize, usize), dir: Direction) -> (usize, usize) {
	match dir {
		Direction::LEFT => (p.0, p.1.wrapping_sub(1)),
		Direction::RIGHT => (p.0, p.1 + 1),
		Direction::UP => (p.0.wrapping_sub(1), p.1),
		Direction::DOWN => (p.0 + 1, p.1),
	}
}
fn unstep(p: (usize, usize), dir: Direction) -> (usize, usize) {
	match dir {
		Direction::LEFT => (p.0, p.1 + 1),
		Direction::RIGHT => (p.0, p.1.wrapping_sub(1)),
		Direction::UP => (p.0 + 1, p.1),
		Direction::DOWN => (p.0.wrapping_sub(1), p.1),
	}
}
#[derive(Clone, Copy, Default)]
struct Cube {
	col: Color,
	n: i16,
}
#[derive(Clone, Copy)]
enum Tile {
	FLOOR,
	WALL,
	DOOR,
	CUBE(Cube),
}
pub struct Game<'a> {
	pos: (usize, usize),
	m: Vec<Vec<Tile>>,
	res: &'a Res,
}
impl<'a> Game<'a> {
	pub fn new(buf: &str, res: &'a Res) -> Result<Self, Box<dyn std::error::Error>> {
		let mut v = buf.split_whitespace().map(String::from);
		let h: usize = v.next().ok_or("malformatted level file")?.parse()?;
		let w: usize = v.next().ok_or("malformatted level file")?.parse()?;
		let mut m = vec![vec![Tile::FLOOR; w]; h];
		for i in 0..h {
			let s = v.next().ok_or("malformatted level file")?;
			for (j, c) in s.chars().enumerate() {
				m[i][j] = match c {
					'.' => Tile::FLOOR,
					'#' => Tile::WALL,
					'D' => Tile::DOOR,
					_ => Tile::FLOOR,
				}
			}
		}
		let mut pos = (0usize, 0usize);
		pos.0 = v.next().ok_or("malformatted level file")?.parse()?;
		pos.1 = v.next().ok_or("malformatted level file")?.parse()?;
		let ncubes: usize = v.next().ok_or("malformatted level file")?.parse()?;
		for _ in 0..ncubes {
			let x: usize = v.next().ok_or("malformatted level file")?.parse()?;
			let y: usize = v.next().ok_or("malformatted level file")?.parse()?;
			let r: u8 = v.next().ok_or("malformatted level file")?.parse()?;
			let g: u8 = v.next().ok_or("malformatted level file")?.parse()?;
			let b: u8 = v.next().ok_or("malformatted level file")?.parse()?;
			let num: i16 = v.next().ok_or("malformatted level file")?.parse()?;
			m[y][x] = Tile::CUBE(Cube {
				n: num,
				col: Color([r, g, b, 255]),
			});
		}

		Ok(Game {
			pos: pos,
			m: m,
			res: res,
		})
	}
	fn is_inside(&self, p: (usize, usize)) -> bool {
		p.0 < self.m.len() && p.1 < self.m[0].len()
	}
	fn mov(&mut self, dir: Direction) -> bool {
		let np = step(self.pos, dir);
		if self.is_inside(np) && self.mov_block(np, dir) {
			self.pos = np;
			true
		} else {
			false
		}
	}
	fn mov_block(&mut self, p: (usize, usize), dir: Direction) -> bool {
		match &self.m[p.0][p.1] {
			Tile::FLOOR => true,
			Tile::CUBE(x) => {
				let np = step(p, dir);
				if !self.is_inside(np) {
					false
				} else {
					match &self.m[np.0][np.1] {
						Tile::CUBE(y) => {
							if y.col == x.col {
								self.m[np.0][np.1] = if x.n + y.n == 0 {
									Tile::FLOOR
								} else {
									Tile::CUBE(Cube {
										col: x.col,
										n: x.n + y.n,
									})
								};
								self.m[p.0][p.1] = Tile::FLOOR;
								true
							} else {
								if self.mov_block(np, dir) {
									self.m[np.0][np.1] = self.m[p.0][p.1];
									self.m[p.0][p.1] = Tile::FLOOR;
									true
								} else {
									false
								}
							}
						}
						_ => {
							if self.mov_block(np, dir) {
								self.m[np.0][np.1] = self.m[p.0][p.1];
								self.m[p.0][p.1] = Tile::FLOOR;
								true
							} else {
								false
							}
						}
					}
				}
			}
			_ => false,
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
impl<'a> State<'a> for Game<'a> {
	fn update(&'a mut self) -> Transition {
		if is_key_pressed(KeyCode::Right) {
			self.mov(Direction::RIGHT);
		} else if is_key_pressed(KeyCode::Left) {
			self.mov(Direction::LEFT);
		} else if is_key_pressed(KeyCode::Up) {
			self.mov(Direction::UP);
		} else if is_key_pressed(KeyCode::Down) {
			self.mov(Direction::DOWN);
		} else if is_mouse_button_down(MouseButton::Left) {
			if self.get_mouse_pos() == step(self.pos, Direction::RIGHT) {
				self.mov(Direction::RIGHT);
			} else if self.get_mouse_pos() == step(self.pos, Direction::LEFT) {
				self.mov(Direction::LEFT);
			} else if self.get_mouse_pos() == step(self.pos, Direction::UP) {
				self.mov(Direction::UP);
			} else if self.get_mouse_pos() == step(self.pos, Direction::DOWN) {
				self.mov(Direction::DOWN);
			}
		}
		return Transition::POP(0);
	}
	fn draw(&mut self) {
		let scale =
			(screen_height() / self.m.len() as f32).min(screen_width() / self.m[0].len() as f32);
		for i in 0..self.m.len() {
			for j in 0..self.m[0].len() {
				match self.m[i][j] {
					Tile::FLOOR => {
						draw_texture_ex(
							self.res.floor,
							j as f32 * scale,
							i as f32 * scale,
							WHITE,
							DrawTextureParams {
								dest_size: Some(vec2(scale, scale)),
								..Default::default()
							},
						);
					}
					Tile::WALL => {
						draw_texture_ex(
							self.res.wall,
							j as f32 * scale,
							i as f32 * scale,
							WHITE,
							DrawTextureParams {
								dest_size: Some(vec2(scale, scale)),
								..Default::default()
							},
						);
					}
					Tile::DOOR => {
						draw_texture_ex(
							self.res.door,
							j as f32 * scale,
							i as f32 * scale,
							WHITE,
							DrawTextureParams {
								dest_size: Some(vec2(scale, scale)),
								..Default::default()
							},
						);
					}
					Tile::CUBE(x) => {
						draw_texture_ex(
							self.res.floor,
							j as f32 * scale,
							i as f32 * scale,
							WHITE,
							DrawTextureParams {
								dest_size: Some(vec2(scale, scale)),
								..Default::default()
							},
						);
						draw_texture_ex(
							self.res.cube,
							j as f32 * scale,
							i as f32 * scale,
							x.col,
							DrawTextureParams {
								dest_size: Some(vec2(scale, scale)),
								..Default::default()
							},
						);
						let txt = x.n.to_string();
						let font_size = scale * 2.0 / 3.0;
						let text_size = measure_text(&txt, font_size);
						draw_text(
							&txt,
							(j as f32 + 0.5) * scale - text_size.0 / 2.0,
							(i as f32 + 0.25) * scale - text_size.1 / 2.0,
							font_size,
							BLACK,
						);
					}
				}
			}
		}
		draw_texture_ex(
			self.res.player,
			self.pos.1 as f32 * scale,
			self.pos.0 as f32 * scale,
			WHITE,
			DrawTextureParams {
				dest_size: Some(vec2(scale, scale)),
				..Default::default()
			},
		);
	}
}
