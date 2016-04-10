use std::sync::{Arc};

use crossbeam::sync::{MsQueue};
use cgmath::{Vector3, Matrix4};

use render::render_command::{RenderCommand};
use render::render_frame::{RenderFrame};
use context::{Context, ContextType};
use context_state::{ContextState, ContextStateProxy};
use uniform_wrappers::{UMatrix4};


type RenderState = ContextState<()>;

pub struct RenderContext {
	q: Arc<MsQueue<RenderCommand>>,
	state: RenderState,
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
			self.state.frame_counter(),
			contexts.context_physics().get_frame()
		));
		self.q.push(RenderCommand::ClearScreen{ render_frame: render_frame.clone() });
		render_frame
	}

	fn swap_buffers(&self, frame: &Arc<RenderFrame>) {
		self.q.push(RenderCommand::SwapBuffers{ frame_counter: frame.frame_counter });
	}

	fn draw_scene(&self, frame: &mut Arc<RenderFrame>) {
		self.q.push(RenderCommand::DrawScene{
			frame_counter: frame.frame_counter,
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
			frame_counter: frame.frame_counter,
			uniforms: uniforms,
		});
	}
}

impl Context for RenderContext {
	fn frequency(&self) -> u64 { 60 }

	fn tick(&self, contexts: Arc<ContextType>) {
		let mut frame = self.clear_screen(contexts);

		self.draw_scene(&mut frame);

		self.draw_player(&mut frame);

		self.swap_buffers(&frame);
	}

	fn state(&self) -> &ContextStateProxy { &self.state }
}
