use std::sync::{Arc};

use crossbeam::sync::{MsQueue};

use render::render_command::{RenderCommand};
use render::render_frame::{RenderFrame};
use context::{Context, ContextType};
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

	fn clear_screen(&self, contexts: Arc<ContextType>) {
		self.q.push(RenderCommand::ClearScreen{ render_frame: RenderFrame::new(
			self.frame_counter.get(),
			contexts.context_physics().get_frame()
		)});
	}
	fn swap_buffers(&self) {
		self.q.push(RenderCommand::SwapBuffers{ frame_counter: self.frame_counter.get() });
	}
	fn draw_scene(&self) {
		self.q.push(RenderCommand::DrawScene{ frame_counter: self.frame_counter.get() });
	}
	fn draw_player(&self) {
		self.q.push(RenderCommand::DrawPlayer{ frame_counter: self.frame_counter.get() });
	}
}

impl Context for RenderContext {
	fn rate(&self) -> u64 {
		16666666
	}

	fn tick(&self, contexts: Arc<ContextType>) {
		self.frame_counter.increment();

		self.clear_screen(contexts);

		self.draw_scene();

		self.draw_player();

		self.swap_buffers();
	}
}
