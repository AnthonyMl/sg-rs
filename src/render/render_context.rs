use std::sync::{Arc};

use crossbeam::sync::{MsQueue};
use cgmath::{Vector3, Matrix4};

use render::render_command::{RenderCommand};
use render::render_frame::{RenderFrame};
use render::uniform_wrappers::{UMatrix4};
use render::render_uniforms::{RenderUniforms};
use context::{Context, ContextType, ContextState};
use frame::{Frame};


pub struct RenderContext {
	q: Arc<MsQueue<RenderCommand>>,
	state: ContextState,
}

impl RenderContext {
	pub fn new(q: Arc<MsQueue<RenderCommand>>, physics_frame: Frame) -> RenderContext {
		let physics_frame = (match physics_frame {
			Frame::Physics(frame) => Some(frame),
			_ => None,
		}).unwrap();

		RenderContext {
			q: q,
			state: ContextState::new(
				Frame::Render(Arc::new(RenderFrame {
					frame_counter: 0,
					physics_frame: physics_frame.clone(),
					uniforms: RenderUniforms {
						mvp: UMatrix4(physics_frame.camera.mtx_full),
					},
				}))
			),
		}
	}

	fn clear_screen(&self, render_frame: Arc<RenderFrame>) {
		self.q.push(RenderCommand::ClearScreen{ render_frame: render_frame });
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

unsafe impl Send for RenderContext {}
unsafe impl Sync for RenderContext {}

impl Context for RenderContext {
	fn frequency(&self) -> u64 { 60 }

	fn tick(&self, contexts: Arc<ContextType>, _: Frame) -> Frame {
		let physics_frame = contexts.context_physics().get_frame();

		let mut frame = Arc::new(RenderFrame {
			frame_counter: self.state().frame_counter(),
			physics_frame: physics_frame.clone(),
			uniforms: RenderUniforms {
				mvp: UMatrix4(physics_frame.camera.mtx_full),
			},
		});

		self.clear_screen(frame.clone());

		self.draw_scene(&mut frame);

		self.draw_player(&mut frame);

		self.swap_buffers(&frame);

		Frame::Render(frame)
	}

	fn state(&self) -> &ContextState { &self.state }
}
