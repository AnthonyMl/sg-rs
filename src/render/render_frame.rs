use std::sync::{Arc};

use cgmath::{Matrix4, Vector3, Vector4, SquareMatrix, EuclideanSpace, InnerSpace};

use physics::{PhysicsFrame};
use render::render_uniforms::{RenderUniforms};
use render::uniform_wrappers::{UMatrix4, UVector3};


pub struct RenderFrame {
	pub id: u64,
	pub scene_uniforms:  RenderUniforms, // TODO: do we need to box the uniforms?
	pub player_uniforms: RenderUniforms,
}

impl RenderFrame {
	pub fn new(physics_frame: Arc<PhysicsFrame>) -> RenderFrame {
		let light_direction = Vector3::new(-1f64, -1f64, 0f64).normalize();
		let reverse_light_direction = light_direction * -1f64;

		let view       = physics_frame.camera.view.clone();
		let projection = physics_frame.camera.projection.clone();
		let view_projection = projection * view;

		let scene_uniforms = RenderUniforms {
			model:                   UMatrix4(Matrix4::identity()),
			model_view_projection:   UMatrix4(view_projection),
			reverse_light_direction: UVector3(reverse_light_direction),
		};

		let translation = Matrix4::from_translation(physics_frame.player_position.to_vec());

		let up                  = Vector3::new(0f64, 1f64, 0f64);
		let flat_view_direction = (Vector3 { y: 0f64, .. physics_frame.get_view_direction() }).normalize();
		let right               = flat_view_direction.cross(up).normalize();
		let up                  = right.cross(flat_view_direction);
		let rotation = Matrix4::from_cols(
			right.extend(0f64),
			up.extend(0f64),
			(flat_view_direction * -1f64).extend(0f64),
			Vector4::unit_w());

		let model = translation * rotation;
		let model_view_projection = view_projection * model;

		let player_uniforms = RenderUniforms {
			model:                   UMatrix4(model),
			model_view_projection:   UMatrix4(model_view_projection),
			reverse_light_direction: UVector3(reverse_light_direction),
		};

		RenderFrame {
			id:              physics_frame.frame_counter,
			scene_uniforms:  scene_uniforms,
			player_uniforms: player_uniforms,
		}
	}
}
