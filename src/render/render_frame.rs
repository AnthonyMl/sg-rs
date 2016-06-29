use std::f64::{MAX, MIN};
use std::sync::{Arc};

use cgmath;
use cgmath::{Matrix, Matrix3, Matrix4, Vector3, Vector4, SquareMatrix, EuclideanSpace, InnerSpace};

use context::{Context};
use physics::{PhysicsFrame};
use render::render_context::{DEPTH_DIMENSION};
use render::render_uniforms::{RenderUniforms};
use render::uniform_wrappers::{UMatrix4, UVector3};


pub struct RenderFrame {
	pub id: u64,
	pub scene_uniforms:  RenderUniforms, // TODO: do we need to box the uniforms?
	pub player_uniforms: RenderUniforms,
}

impl RenderFrame {
	pub fn new(_context: Arc<Context>, physics_frame: Arc<PhysicsFrame>) -> RenderFrame {
		let light_direction = physics_frame.light_direction;
		let reverse_light_direction = light_direction * -1.0;

		let shadow_view_projection = {
			let corners = physics_frame.camera.view_corners();

			// TODO: this should be constant/held somewhere
			//
			let shadow_width = {
				let outside_length:  f64 = (corners[4] - corners[1]).magnitude();
				let diagonal_length: f64 = (corners[1] - corners[0]).magnitude();
				outside_length.max(diagonal_length)
			};

			let up = Vector3::unit_y();
			let right = light_direction.cross(up).normalize();
			let up = right.cross(light_direction).normalize();

			let rotation_transposed = Matrix3::from_cols(right, up, reverse_light_direction);
			let rotation = rotation_transposed.transpose();

			// TODO: do something correct instead of this
			//
			let geometry_corners: Vec<Vector3<f64>> = vec![
				Vector3::new(-20.0,  0.0, -20.0),
				Vector3::new( 20.0,  0.0, -20.0),
				Vector3::new(-20.0,  0.0,  20.0),
				Vector3::new( 20.0,  0.0,  20.0),
				Vector3::new(-20.0, 10.0, -20.0),
				Vector3::new( 20.0, 10.0, -20.0),
				Vector3::new(-20.0, 10.0,  20.0),
				Vector3::new( 20.0, 10.0,  20.0)
			];
			let corners = corners.iter().chain(geometry_corners.iter());
			let transformed_corners = corners.map(|&v| rotation * v);

			let mut min_x = MAX;
			let mut min_y = MAX;
			let mut min_z = MAX;
			let mut max_z = MIN;
			for corner in transformed_corners {
				if corner.x < min_x { min_x = corner.x }
				if corner.y < min_y { min_y = corner.y }
				if corner.z < min_z { min_z = corner.z }
				if corner.z > max_z { max_z = corner.z }
			}

			let world_units_per_texel = shadow_width / (DEPTH_DIMENSION as f64);
			min_x = (min_x / world_units_per_texel).floor() * world_units_per_texel;
			min_y = (min_y / world_units_per_texel).floor() * world_units_per_texel;

			let projection = cgmath::ortho(
				min_x,
				min_x + shadow_width,
				min_y,
				min_y + shadow_width,
				-max_z,
				-min_z
			);

			let rotation = Matrix4::from_cols(
				rotation_transposed.x.extend(0.0),
				rotation_transposed.y.extend(0.0),
				rotation_transposed.z.extend(0.0),
				Vector4::new(0.0, 0.0, 0.0, 1.0)
			).transpose();
			projection * rotation
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
