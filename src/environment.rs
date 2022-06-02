use speedy2d::{Graphics2D, color::Color, font::{Font, TextOptions, TextLayout}};

const POLICY: Policy = Policy::Iterative; 
const IMPROVE_POLICY: bool = true;

#[derive(PartialEq)]
pub enum Policy {
	Iterative,
	IterativeInPlace,
}

#[derive(Copy, Clone)]
pub struct State {
	row: u32,
	col: u32,
	reward: f32,
	value: f32,
	next_value: f32,
	max_value: f32,
}

impl State {
	pub fn draw(&self, graphics: &mut Graphics2D, font: &Font, x_offset: u32, y_offset: u32, cell_size: u32) {
		let text = format!("{:.4}", self.value);
		let block = font.layout_text(&text, 0.4 * cell_size as f32, TextOptions::new());
		let x = x_offset as f32 + self.row as f32 * cell_size as f32 + 0.5 * cell_size as f32 - 0.5 * block.width();
		let y = y_offset as f32 + self.col as f32 * cell_size as f32 + 0.5 * cell_size as f32 - 0.5 * block.height();
		graphics.draw_text((x.round(), y.round()), Color::WHITE, &block);
	}
}

pub struct Environment {
	font: Font,
	x_offset: u32, 
	y_offset: u32, 
	num_rows: u32, 
	num_cols: u32, 
	cell_size: u32,
	states: Vec<Vec<State>>,
}

impl Environment {
    pub fn new(x_offset: u32, y_offset:u32, num_rows: u32, num_cols: u32, cell_size: u32) -> Self {		
		let bytes = include_bytes!("../assets/fonts/ariel.ttf");
		let font = Font::new(bytes).unwrap();

		let mut states = vec![];

		for row in 0..num_cols {
			let mut s: Vec<State> = Vec::new();
			for col in 0..num_rows  {
				s.push(State { reward: 0.0, value: 0.0, next_value: 0.0, row: row, col: col, max_value: 0.0 });
			}
			states.push(s);
		}

		states[0][0].reward = 1.0;

        Self { 
            x_offset,
            y_offset,
            num_rows,
            num_cols,
            cell_size,
			font,
			states,
		}
	}  

	pub fn act(&mut self) -> bool {
		let mut converged: bool = true;

		// Update every state
		for row in 0..self.num_rows {
			for col in 0..self.num_rows {
				let next_value = self.get_next_value(row, col);

				// Did we converge?
				if self.states[row as usize][col as usize].value != next_value {
					converged = false;
				}

				match POLICY {
					Policy::Iterative => {
						self.states[row as usize][col as usize].next_value = next_value;
					},
					Policy::IterativeInPlace => {
						self.states[row as usize][col as usize].value = next_value;
					},
				}
			}
		}

		// Copy new states into old states if Policy::Iterative
		if POLICY == Policy::Iterative {
			for row in 0..self.num_rows {
				for col in 0..self.num_rows {
					self.states[row as usize][col as usize].value = self.states[row as usize][col as usize].next_value;
				}
			}
		}

		// calculate maxes
		if IMPROVE_POLICY {
			converged = true;

			for row in 0..self.num_rows {
				for col in 0..self.num_rows {
					self.states[row as usize][col as usize].max_value = self.get_max_value(col, row);
				}
			}

			for row in 0..self.num_rows {
				for col in 0..self.num_rows {
					let max_value = self.states[row as usize][col as usize].max_value;
					if max_value != self.states[row as usize][col as usize].value {
						converged = false;
						self.states[row as usize][col as usize].value = max_value;
					}
				}
			}
		}

		converged
	}

	fn get_next_value(&mut self, col: u32, row: u32) -> f32 {
		let col = col as usize;
		let row = row as usize;

		let mut l: f32 = 0.0;
		let mut u: f32 = 0.0;
		let mut r: f32 = 0.0;
		let mut d: f32 = 0.0;

		if col != 0 { 
			l = self.states[col - 1][row].value;
		}

		if row != 0 {
			u = self.states[col][row - 1].value;
		}

		if col < (self.num_rows - 1) as usize { 
			r = self.states[col + 1][row].value;
		}

		if row < (self.num_cols - 1) as usize {
			d = self.states[col][row + 1].value;
		}

		let reward = self.states[col][row].reward as f32;
		let sum = (l + u + r + d) as f32;
		let next_value = reward + sum / 4.0;
					
		next_value
	}

	fn get_max_value(&mut self, col: u32, row: u32) -> f32 {
		let col = col as usize;
		let row = row as usize;

		let mut l: f32 = 0.0;
		let mut u: f32 = 0.0;
		let mut r: f32 = 0.0;
		let mut d: f32 = 0.0;

		if col != 0 { 
			l = self.states[col - 1][row].value;
		}

		if row != 0 {
			u = self.states[col][row - 1].value;
		}

		if col < (self.num_rows - 1) as usize { 
			r = self.states[col + 1][row].value;
		}

		if row < (self.num_cols - 1) as usize {
			d = self.states[col][row + 1].value;
		}

		let reward = self.states[col][row].reward as f32;

		let next_value = reward + (u.max(r).max(d).max(l) / 4.0);
		
		next_value
	}

	pub fn draw(&self, graphics: &mut Graphics2D) {
		self.draw_grid(graphics);
		self.draw_states(graphics);
	}

	fn draw_grid(&self, graphics: &mut Graphics2D) {
		// Draw vertical lines.
		for x in 0..(self.num_cols + 1) {
			let x = (self.x_offset + x * self.cell_size) as f32;
			let begin = (x, self.y_offset as f32);
			let end = (x, (self.y_offset + self.num_cols * self.cell_size) as f32);
			graphics.draw_line(begin, end, 1.0, Color::GRAY)
		}

		// Draw horizontal lines.
		for y in 0..(self.num_rows + 1) {
			let y = (self.y_offset + y * self.cell_size) as f32;
			let begin = (self.x_offset as f32, y);
			let end = ((self.x_offset + self.num_rows * self.cell_size) as f32, y);
			graphics.draw_line(begin, end, 1.0, Color::GRAY)
		}
	}

	fn draw_states(&self, graphics: &mut Graphics2D) {
		for row in &self.states {
			for state in row  {
				state.draw(graphics, &self.font, self.x_offset, self.y_offset, self.cell_size);
			}
		}
	}
}