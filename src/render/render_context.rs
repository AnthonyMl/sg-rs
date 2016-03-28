use std::sync::{Arc};

use crossbeam::sync::{MsQueue};

use render::render_command::{RenderCommand};
use context::{Context, ContextType};
use frame_counter::{FrameCounter};
use physics::{PhysicsFrame};


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
	pub fn clear_screen(&self, physics_frame: Arc<PhysicsFrame>) {
		self.q.push(RenderCommand::ClearScreen{
			frame_counter: self.frame_counter.get(),
			physics_frame: physics_frame,
		});
	}
	pub fn draw_garbage(&self) {
		self.q.push(RenderCommand::DrawTriangle);
	}
}

impl Context for RenderContext {
	fn rate(&self) -> u64 {
		16666666
	}

	fn tick(&self, contexts: Arc<ContextType>) {
		self.frame_counter.increment();

		let physics_frame = contexts.context_physics().get_frame();

		self.clear_screen(physics_frame);

		self.draw_garbage();

		self.swap_buffers();
	}
}
