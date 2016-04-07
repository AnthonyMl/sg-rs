use std::sync::{Arc};

use crossbeam::sync::{MsQueue};
use cgmath::{Vector3, Matrix4};

use render::render_command::{RenderCommand};
use render::render_frame::{RenderFrame};
use context::{Context, ContextType};
use context_state::{ContextState, ContextStateTrait};
use constants::{NANOSECONDS_PER_SECOND};
use uniform_wrappers::{UMatrix4};


const FREQUENCY: u64 = 60;

pub struct RenderContext {
	q: Arc<MsQueue<RenderCommand>>,
	state: ContextState<()>,
}

impl RenderContext {
	pub fn new(q: Arc<MsQueue<RenderCommand>>) -> RenderContext {
		RenderContext {
			q: q,
			state: ContextState::new(()),
		}
	}

	fn clear_screen(&self, contexts: Arc<ContextType>) -> Arc<RenderFrame> {
		let render_frame = Arc::new(RenderFrame::new(
			self.state.frame_counter.get(),
			contexts.context_physics().get_frame()
		));
		self.q.push(RenderCommand::ClearScreen{ render_frame: render_frame.clone() });
		render_frame
	}

	fn swap_buffers(&self) {
		self.q.push(RenderCommand::SwapBuffers{ frame_counter: self.state.frame_counter.get() });
	}

	fn draw_scene(&self, frame: &mut Arc<RenderFrame>) {
		self.q.push(RenderCommand::DrawScene{
			frame_counter: self.state.frame_counter.get(),
			uniforms: frame.uniforms.clone(),
		});
	}

	fn draw_player(&self, frame: &mut Arc<RenderFrame>) {
		let input_position = frame.physics_frame.player_position;
		let translation = Vector3::new(
			input_position.x as f32,
			input_position.y as f32,
			input_position.z as f32);

		// TODO: change internal mvp to doubles and only convert at the end/batch transforms
		//
		let mut uniforms = frame.uniforms.clone();
		let UMatrix4(view_projection) = uniforms.mvp;
		uniforms.mvp = UMatrix4(view_projection * Matrix4::from_translation(translation));

		self.q.push(RenderCommand::DrawPlayer {
			frame_counter: self.state.frame_counter.get(),
			uniforms: uniforms,
		});
	}
}

impl Context for RenderContext {
	fn rate(&self) -> u64 {
		NANOSECONDS_PER_SECOND / FREQUENCY
	}

	fn tick(&self, contexts: Arc<ContextType>) {
		let mut frame = self.clear_screen(contexts);

		self.draw_scene(&mut frame);

		self.draw_player(&mut frame);

		self.swap_buffers();
	}

	fn is_ready(&self) -> bool { self.state.is_ready() }
	fn pre_tick(&self)         { self.state.frame_counter.increment(); }
	fn post_tick(&self)        { self.state.end_tick(); }
}
