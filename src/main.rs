use speedy2d::{Window, window::{WindowHelper, WindowHandler}, Graphics2D, color::Color};

mod environment;

use environment::Environment;

const NUM_X_CELLS: u32 = 15;
const NUM_Y_CELLS: u32 = 15;
const X_OFFSET: u32 = 50;
const Y_OFFSET: u32 = 50;
const CELL_SIZE: u32 = 100;

fn main() {
	let x_dimension = NUM_X_CELLS * CELL_SIZE + 2 * X_OFFSET;
	let y_dimension = NUM_Y_CELLS * CELL_SIZE + 2 * Y_OFFSET;

    let window = Window::<()>::new_centered("Simulation", (x_dimension, y_dimension)).unwrap();
    window.run_loop(Main::new());
}
pub struct Main {
	steps: u32,
    environment: Environment,
	converged: bool,
}

impl Main {
    pub fn new() -> Self {
        Self { 
			steps: 0,
        	environment: Environment::new(X_OFFSET, Y_OFFSET, NUM_X_CELLS, NUM_Y_CELLS, CELL_SIZE),
			converged: false,
    	}
	}
}

// Converged after 379 steps.
// Converged after 184 steps.
impl WindowHandler for Main {
	fn on_draw(self: &mut Main, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
		if self.converged == false {
			graphics.clear_screen(Color::BLACK);
			self.converged = self.environment.act();
			self.environment.draw(graphics);
			if self.converged {
				println!("Converged after {} steps.", self.steps);
			} else {
				println!("Steps: {}", self.steps);
			}
			self.steps += 1;
			helper.request_redraw();
		}
	}
}