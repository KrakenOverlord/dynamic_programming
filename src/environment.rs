use speedy2d::{Graphics2D, color::Color, font::{Font, TextOptions, TextLayout}};

#[derive(PartialEq)]
pub enum Policy {
	Iterative,
	IterativeInPlace,
}

const POLICY: Policy = Policy::Iterative; 
const IMPROVE_POLICY: bool = true;

#[derive(Copy, Clone, Debug)]
struct ActionValues {
	reward: f32,
	probability: f32,
}

#[derive(Copy, Clone, Debug)]
enum Action {
	Up(ActionValues),
	Right(ActionValues),
	Down(ActionValues), 
	Left(ActionValues),
}

pub struct State {
	row: u32,
	col: u32,
	actions: Vec<Action>,
	policy: Action,
	value: f32,
	next_value: f32,
}

impl State {
	fn terminal(&self) -> bool {
		self.actions.len() == 0
	}

	pub fn draw(&self, graphics: &mut Graphics2D, font: &Font, x_offset: u32, y_offset: u32, cell_size: u32) {
		// draw policy
		if self.terminal() == false {
			let policy = match self.policy {
				Action::Up(_) => "U",
				Action::Right(_) => "R",
				Action::Down(_) => "D",
				Action::Left(_) => "L",
			};
			let policy_text = format!("{}{}:{}", self.row, self.col, policy);
			let value_block = font.layout_text(&policy_text, 0.4 * cell_size as f32, TextOptions::new());
			let x = x_offset as f32 + self.col as f32 * cell_size as f32 + 0.5 * cell_size as f32 - 0.5 * value_block.width();
			let y = y_offset as f32 + self.row as f32 * cell_size as f32 + 0.5 * cell_size as f32 - value_block.height();
			graphics.draw_text((x.round(), y.round()), Color::WHITE, &value_block);
		}

		// draw value
		let value_text = format!("{:.4}", self.value);
		let value_block = font.layout_text(&value_text, 0.4 * cell_size as f32, TextOptions::new());
		let x = x_offset as f32 + self.col as f32 * cell_size as f32 + 0.5 * cell_size as f32 - 0.5 * value_block.width();
		let y = y_offset as f32 + self.row as f32 * cell_size as f32 + 0.5 * cell_size as f32 - 0.25 * value_block.height();
		graphics.draw_text((x.round(), y.round()), Color::WHITE, &value_block);
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

		let states = Environment::initialize_states(num_rows, num_cols);

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

	fn initialize_states(num_rows: u32, num_cols: u32) -> Vec<Vec<State>> {
		let mut states = vec![];

		for row in 0..num_rows {
			let mut s: Vec<State> = Vec::new();
			for col in 0..num_cols  {
				s.push(State { 
					row: row, 
					col: col, 
					actions: vec![
						Action::Up(ActionValues { probability: 0.25, reward: -1.0 }), 
						Action::Right(ActionValues { probability: 0.25, reward: -1.0 }), 
						Action::Down(ActionValues { probability: 0.25, reward: -1.0 }), 
						Action::Left(ActionValues { probability: 0.25, reward: -1.0 }),
					],
					value: 0.0, 
					next_value: 0.0, 
					policy: Action::Up(ActionValues { probability: 0.0, reward: 0.0 }),
				});
			}
			states.push(s);
		}

		// initialize terminal states
		states[0][0].actions = vec![];
		states[3][3].actions = vec![];

		states
	}

	pub fn act(&mut self) -> bool {
		let mut converged: bool = true;

		// Update every state
		for row in 0..self.num_rows {
			for col in 0..self.num_cols {
				let state = &self.states[row as usize][col as usize];
				if state.terminal() {
					continue;
				}
				let next_value = self.get_next_value(state);

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

		if IMPROVE_POLICY {
			for row in 0..self.num_rows {
				for col in 0..self.num_cols {
					let row = row as usize;
					let col = col as usize;
					let state = &self.states[row][col];
					if state.terminal() {
						continue;
					}

					match self.get_greedy_action(state) {
						Some(a) => {
							self.states[row][col].policy = a;
						},
						None => (),
					}
				} 
			}
		}

		converged
	}

	fn get_next_value(& self, state: &State) -> f32 {
		let mut value = 0.0;
		for action in &state.actions {
			value += self.get_action_value(state, action);
		}

		value
	}

	fn get_greedy_action(&self, state: &State) -> Option<Action> {
		if state.actions.len() == 0 {
			return None;
		}

		let first_action = state.actions.first().unwrap();
		let mut value = self.get_action_value(state, first_action);
		let mut greedy_action = first_action;
		for action in &state.actions {
			let current_value = self.get_action_value(state, action);
			println!("Action = {:?} => {}", action, current_value); 

			if current_value > value {
				println!("Upgrading Action = {:?} => {} > {}", action, current_value, value); 
				greedy_action = action;
				value = current_value;
			}
		}
		println!("Greedy Action = {:?}", greedy_action); 

		Some(greedy_action.clone())
	}

	fn get_action_value(&self, state: &State, action: &Action) -> f32 {
		match action {
			Action::Up(av) => {
				if state.row == 0 {
					av.probability * (av.reward + state.value)
				}
				else {
					av.probability * (av.reward + self.states[state.col as usize][(state.row - 1) as usize].value)
				}	
			},
			Action::Right(av) => {
				if state.col == (self.num_cols - 1) {
					av.probability * (av.reward + state.value)
				}
				else {
					av.probability * (av.reward + self.states[(state.col + 1) as usize][state.row as usize].value)
				}	
			},
			Action::Down(av) => {
				if state.row == (self.num_rows - 1) {
					av.probability * (av.reward + state.value)
				}
				else {
					av.probability * (av.reward + self.states[state.col as usize][(state.row + 1) as usize].value)
				}	
			},
			Action::Left(av) => {
				if state.col == 0 {
					av.probability * (av.reward + state.value)
				}
				else {
					av.probability * (av.reward + self.states[(state.col - 1) as usize][state.row as usize].value)
				}	
			},
		}
	}

	pub fn draw(&self, graphics: &mut Graphics2D) {
		self.draw_grid(graphics);
		self.draw_states(graphics);
	}

	fn draw_grid(&self, graphics: &mut Graphics2D) {
		// Draw horizontal lines.
		for row in 0..(self.num_rows + 1) {
			let y = (self.y_offset + row * self.cell_size) as f32;
			let begin = (self.x_offset as f32, y);
			let end = ((self.x_offset + self.num_rows * self.cell_size) as f32, y);
			graphics.draw_line(begin, end, 1.0, Color::GRAY)
		}

		// Draw vertical lines.
		for col in 0..(self.num_cols + 1) {
			let x = (self.x_offset + col * self.cell_size) as f32;
			let begin = (x, self.y_offset as f32);
			let end = (x, (self.y_offset + self.num_cols * self.cell_size) as f32);
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