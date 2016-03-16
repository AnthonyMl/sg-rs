use std::sync::{Arc};
use crossbeam::sync::{MsQueue};

use render::render_command::{RenderCommand};


pub struct RenderContext {
	q: Arc<MsQueue<RenderCommand>>,
}

impl RenderContext {
	pub fn new(q: Arc<MsQueue<RenderCommand>>) -> RenderContext {
		RenderContext {
			q: q,
		}
	}

	pub fn swap_buffers(&self) {
		self.q.push(RenderCommand::SwapBuffers);
	}
	pub fn clear_screen(&self, frame_number: usize) {
		self.q.push(RenderCommand::ClearScreen{ frame_number: frame_number });
	}
	pub fn draw_garbage(&self) {
		self.q.push(RenderCommand::DrawTriangle);
	}

	pub fn tick(&self, frame_number: usize) {
		self.clear_screen(frame_number);

		self.draw_garbage();

		self.swap_buffers();
	}
}
