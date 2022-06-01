use speedy2d::{Graphics2D, color::Color, font::{Font, TextOptions, TextLayout}};

#[derive(Copy, Clone)]
pub struct State {
	row: u32,
	col: u32,
	reward: f32,
	value: f32,
	next_value: f32,
	policy: u32,
}

impl State {
	pub fn draw(&self, graphics: &mut Graphics2D, font: &Font, x_offset: u32, y_offset: u32, cell_size: u32) {
		let text = format!("{:.2}", self.value);
		let block = font.layout_text(&text, 0.5 * cell_size as f32, TextOptions::new());
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
	update_in_place: bool,
}

impl Environment {
    pub fn new(x_offset: u32, y_offset:u32, num_rows: u32, num_cols: u32, cell_size: u32, update_in_place: bool) -> Self {		
		let bytes = include_bytes!("../assets/fonts/ariel.ttf");
		let font = Font::new(bytes).unwrap();

		let mut states = vec![];

		for row in 0..num_cols {
			let mut s: Vec<State> = Vec::new();
			for col in 0..num_rows  {
				s.push(State { reward: 0.0, value: 0.0, next_value: 0.0, row: row, col: col, policy: 0 });
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
			update_in_place,
		}
	}  

	pub fn act(&mut self) -> bool {
		let mut converged: bool = true;

		// Update new states
		for row in 0..self.num_rows {
			for col in 0..self.num_rows {
				let new_value = self.get_next_value(row, col);
				
				// Did we converge?
				if self.states[row as usize][col as usize].value != new_value {
					converged = false;
				}

				if self.update_in_place {
					self.states[row as usize][col as usize].value = new_value;
				} else {
					self.states[row as usize][col as usize].next_value = new_value;
				}
			}
		}

		// Copy new states into old states unless update_in_place
		if self.update_in_place == false {
			for row in 0..self.num_rows {
				for col in 0..self.num_rows {
					self.states[row as usize][col as usize].value = self.states[row as usize][col as usize].next_value;
				}
			}
		}

		converged
	}

	fn get_next_value(&mut self, x: u32, y: u32) -> f32 {
		let x = x as usize;
		let y = y as usize;

		let mut l: f32 = 0.0;
		let mut u: f32 = 0.0;
		let mut r: f32 = 0.0;
		let mut d: f32 = 0.0;

		if x != 0 { 
			l = self.states[x - 1][y].value;
		}

		if y != 0 {
			u = self.states[x][y - 1].value;
		}

		if x < (self.num_rows - 1) as usize { 
			r = self.states[x + 1][y].value;
		}

		if y < (self.num_cols - 1) as usize {
			d = self.states[x][y + 1].value;
		}

		let reward = self.states[x][y].reward as f32;
		let sum = (l + u + r + d) as f32;
		let new_value = reward + sum / 4.0;
		
		new_value
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