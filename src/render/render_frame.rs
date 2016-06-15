use std::sync::{Arc};

use cgmath;
use cgmath::{Matrix4, Point3, Vector3, Vector4, SquareMatrix, EuclideanSpace, InnerSpace};

use camera;
use context::{Context};
use physics::{PhysicsFrame};
use render::render_uniforms::{RenderUniforms};
use render::uniform_wrappers::{UMatrix4, UVector3};


pub struct RenderFrame {
	pub id: u64,
	pub scene_uniforms:  RenderUniforms, // TODO: do we need to box the uniforms?
	pub player_uniforms: RenderUniforms,
}

impl RenderFrame {
	pub fn new(context: Arc<Context>, physics_frame: Arc<PhysicsFrame>) -> RenderFrame {
		let light_direction = Vector3::new(1f64, -1f64, -1.5f64).normalize();
		let reverse_light_direction = light_direction * -1f64;

		let shadow_view_projection = {
			const SHADOW_NEAR_PLANE: f64 = 0.001;
			const SHADOW_FAR_PLANE: f64 = 40.0;
			let shadow_width = {
				let aspect_ratio = context.render.aspect_ratio();
				let y = camera::FAR_PLANE * (0.5 * camera::FIELD_OF_VIEW).tan();
				let x = y * aspect_ratio;
				let length = (x * x + y * y + camera::FAR_PLANE * camera::FAR_PLANE).sqrt();
				let y_near = camera::NEAR_PLANE * (0.5 * camera::FIELD_OF_VIEW).tan();
				let x_near = y_near * aspect_ratio;
				let length_near = (x_near * x_near + y_near * y_near + camera::NEAR_PLANE * camera::NEAR_PLANE).sqrt();

				let outside_length = length - length_near;
				let diagonal_length = (4.0 * (x * x + y * y)).sqrt();
				outside_length.max(diagonal_length)
			};

			let projection = cgmath::ortho(
				-0.5 * shadow_width,
				 0.5 * shadow_width,
				-0.5 * shadow_width,
				 0.5 * shadow_width,
				SHADOW_NEAR_PLANE,
				SHADOW_FAR_PLANE
			);

			let eye = Point3::from_vec(reverse_light_direction * 0.5 * SHADOW_FAR_PLANE);

			let center = Point3::origin();

			let up = Vector3::unit_z();
			let right = reverse_light_direction.cross(up).normalize();
			let up = right.cross(reverse_light_direction).normalize();

			projection * Matrix4::look_at(eye, center, up)
		};

		let view       = physics_frame.camera.view.clone();
		let projection = physics_frame.camera.projection.clone();
		let view_projection = projection * view;

		let scene_uniforms = RenderUniforms {
			shadow:                  UMatrix4(shadow_view_projection),
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
		let shadow = shadow_view_projection * model;

		let player_uniforms = RenderUniforms {
			shadow:                  UMatrix4(shadow),
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
