use std::sync::{Arc};

use cgmath::{Matrix4, Vector3, Vector4, SquareMatrix, EuclideanSpace, InnerSpace};

use render::render_uniforms::{RenderUniforms};
use render::uniform_wrappers::{UMatrix4};
use context::{ContextType};


pub struct RenderFrame;

impl RenderFrame {
	pub fn frame_zero() -> RenderFrame { RenderFrame }

	pub fn new(context: Arc<ContextType>, _last_frame: Arc<RenderFrame>) -> RenderFrame {
		let physics_frame = context.frame_physics();

		// TODO: do something about passing the frame counter on every
		// a guard/builder pattern that sends a command queue to base on drop or something
		//

		let rc = context.render();

		let frame_counter = context.counter_render();

		rc.clear_screen(frame_counter);

		let view       = physics_frame.camera.view.clone();
		let projection = physics_frame.camera.projection.clone();
		let view_projection = projection * view;

		rc.draw_scene(frame_counter, RenderUniforms {
			model:                 UMatrix4(Matrix4::identity()),
			model_view_projection: UMatrix4(view_projection),
		});

		let translation = Matrix4::from_translation(physics_frame.player_position.to_vec());

		let up                  = Vector3::new(0f64, 1f64, 0f64);
		let flat_view_direction = (Vector3 { y: 0f64, .. physics_frame.view_direction}).normalize();
		let right               = flat_view_direction.cross(up).normalize();
		let up                  = right.cross(flat_view_direction);
		let rotation = Matrix4::from_cols(
			right.extend(0f64),
			up.extend(0f64),
			(flat_view_direction * -1f64).extend(0f64),
			Vector4::unit_w());

		let model = translation * rotation;
		let model_view_projection = view_projection * model;

		rc.draw_player(frame_counter, RenderUniforms {
			model:                 UMatrix4(model),
			model_view_projection: UMatrix4(model_view_projection),
		});

		rc.swap_buffers(frame_counter);

		RenderFrame
	}
}
