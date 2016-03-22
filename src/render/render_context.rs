use std::sync::{Arc};

use crossbeam::sync::{MsQueue};

use render::render_command::{RenderCommand};
use context::{Context};
use frame_counter::{FrameCounter};


pub struct RenderContext {
	q: Arc<MsQueue<RenderCommand>>,
	frame_counter: FrameCounter,
}

impl RenderContext {
	pub fn new(q: Arc<MsQueue<RenderCommand>>) -> RenderContext {
		RenderContext {
			q: q,
			frame_counter: FrameCounter::new(0),
		}
	}

	pub fn swap_buffers(&self) {
		self.q.push(RenderCommand::SwapBuffers);
	}
	pub fn clear_screen(&self) {
		self.q.push(RenderCommand::ClearScreen{ frame_counter: self.frame_counter.get() });
	}
	pub fn draw_garbage(&self) {
		self.q.push(RenderCommand::DrawTriangle);
	}
}

impl Context for RenderContext {
	fn rate(&self) -> u64 {
		16666666
	}

	fn tick(&self) {
		self.frame_counter.increment();

		self.clear_screen();

		self.draw_garbage();

		self.swap_buffers();
	}
}
