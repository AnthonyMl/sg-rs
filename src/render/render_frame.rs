use glium::{Frame};


pub struct RenderFrame {
	pub _frame_counter: u64,
	pub draw_context: Frame,
}

impl RenderFrame {
	pub fn new(frame_counter: u64, draw_context: Frame) -> RenderFrame {
		RenderFrame {
			_frame_counter: frame_counter,
			draw_context: draw_context,
		}
	}
}
