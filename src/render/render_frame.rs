use glium::{Frame};


pub struct RenderFrame {
	pub _frame_number: usize,
	pub draw_context: Frame,
}

impl RenderFrame {
	pub fn new(frame_number: usize, draw_context: Frame) -> RenderFrame {
		RenderFrame {
			_frame_number: frame_number,
			draw_context: draw_context,
		}
	}
}
