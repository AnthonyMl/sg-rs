use std::sync::{Arc};

use glium::{Frame};

use physics::{PhysicsFrame};


pub struct RenderFrame {
	pub _frame_counter: u64,
	pub draw_context: Frame,
	pub physics_frame: Arc<PhysicsFrame>,
}

impl RenderFrame {
	pub fn new(frame_counter: u64, draw_context: Frame, physics_frame: Arc<PhysicsFrame>) -> RenderFrame {
		RenderFrame {
			_frame_counter: frame_counter,
			draw_context: draw_context,
			physics_frame: physics_frame,
		}
	}
}
