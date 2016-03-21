use std::sync::{Arc};
use std::sync::atomic::{AtomicUsize, Ordering};
use crossbeam::sync::{MsQueue};

use render::render_command::{RenderCommand};
use context::{Context};


pub struct RenderContext {
	q: Arc<MsQueue<RenderCommand>>,
	frame_number: AtomicUsize,
}

impl RenderContext {
	pub fn new(q: Arc<MsQueue<RenderCommand>>) -> RenderContext {
		RenderContext {
			q: q,
			frame_number: AtomicUsize::new(0),
		}
	}

	pub fn swap_buffers(&self) {
		self.q.push(RenderCommand::SwapBuffers);
	}
	pub fn clear_screen(&self) {
		self.q.push(RenderCommand::ClearScreen{ frame_number: self.frame_number.load(Ordering::Relaxed) });
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
		loop {
			let v = self.frame_number.load(Ordering::Acquire);
			if v == self.frame_number.compare_and_swap(v, v + 1, Ordering::Release) { break }
		}

		self.clear_screen();

		self.draw_garbage();

		self.swap_buffers();
	}
}
