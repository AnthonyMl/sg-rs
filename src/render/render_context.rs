use std::sync::{Arc};

use crossbeam::sync::{MsQueue};

use render::render_command::{RenderCommand};
use render::render_uniforms::{RenderUniforms};


pub struct RenderContext {
	q: Arc<MsQueue<RenderCommand>>,
	window_size: (u32, u32), // TODO: maybe this should be a per RenderFrame parameter
}

impl RenderContext {
	pub fn new(q: Arc<MsQueue<RenderCommand>>, window_size: (u32, u32)) -> RenderContext {
		RenderContext {
			q: q,
			window_size: window_size,
		}
	}

	pub fn window_size(&self) -> (u32, u32) {
		self.window_size
	}

	// --- Draw Commands --- (candidates for inlining)
	//
	pub fn clear_screen(&self, frame_counter: u64) {
		self.q.push(RenderCommand::ClearScreen { frame_counter: frame_counter });
	}

	pub fn swap_buffers(&self, frame_counter: u64) {
		self.q.push(RenderCommand::SwapBuffers { frame_counter: frame_counter });
	}

	pub fn draw_scene(&self, frame_counter: u64, uniforms: RenderUniforms) {
		self.q.push(RenderCommand::DrawScene {
			frame_counter: frame_counter,
			uniforms: uniforms,
		});
	}

	pub fn draw_player(&self, frame_counter: u64, uniforms: RenderUniforms) {
		self.q.push(RenderCommand::DrawPlayer {
			frame_counter: frame_counter,
			uniforms: uniforms,
		});
	}
}

unsafe impl Send for RenderContext {}
unsafe impl Sync for RenderContext {}
