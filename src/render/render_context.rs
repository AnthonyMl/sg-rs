use std::sync::{Arc};

use crossbeam::sync::{MsQueue};
use cgmath::{Vector3, Matrix4};

use render::render_command::{RenderCommand};
use render::render_frame::{RenderFrame};
use context::{Context, ContextType};
use frame_counter::{FrameCounter};
use constants::{NANOSECONDS_PER_SECOND};
use uniform_wrappers::{UMatrix4};


const FREQUENCY: u64 = 60;

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

	fn clear_screen(&self, contexts: Arc<ContextType>) -> Arc<RenderFrame> {
		let render_frame = Arc::new(RenderFrame::new(
			self.frame_counter.get(),
			contexts.context_physics().get_frame()
		));
		self.q.push(RenderCommand::ClearScreen{ render_frame: render_frame.clone() });
		render_frame
	}

	fn swap_buffers(&self) {
		self.q.push(RenderCommand::SwapBuffers{ frame_counter: self.frame_counter.get() });
	}

	fn draw_scene(&self, frame: &mut Arc<RenderFrame>) {
		self.q.push(RenderCommand::DrawScene{
			frame_counter: self.frame_counter.get(),
			uniforms: frame.uniforms.clone(),
		});
	}

	fn draw_player(&self, frame: &mut Arc<RenderFrame>) {
		let input_position = frame.physics_frame.player_position;
		let translation = Vector3::new(
			-input_position.x as f32,
			-input_position.y as f32,
			-input_position.z as f32);

		// TODO: change internal mvp to doubles and only convert at the end/batch transforms
		//
		let mut uniforms = frame.uniforms.clone();
		let UMatrix4(view_projection) = uniforms.mvp;
		uniforms.mvp = UMatrix4(view_projection * Matrix4::from_translation(translation));

		self.q.push(RenderCommand::DrawPlayer {
			frame_counter: self.frame_counter.get(),
			uniforms: uniforms,
		});
	}
}

impl Context for RenderContext {
	fn rate(&self) -> u64 {
		NANOSECONDS_PER_SECOND / FREQUENCY
	}

	fn tick(&self, contexts: Arc<ContextType>) {
		self.frame_counter.increment();

		let mut frame = self.clear_screen(contexts);

		self.draw_scene(&mut frame);

		self.draw_player(&mut frame);

		self.swap_buffers();
	}

	fn ready_to_tick(&self) -> bool { true }
}
