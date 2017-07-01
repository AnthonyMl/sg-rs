use std::f32::{MAX, MIN};
use std::f32::consts::{PI};
use std::sync::{Arc};

use cgmath;
use cgmath::{Matrix, Matrix3, Matrix4, Vector3, Vector4, SquareMatrix, EuclideanSpace, InnerSpace, Rad};
use rand::{SeedableRng, StdRng};
use rand::distributions::{IndependentSample, Range};

use context::{Context};
use debug::{UnlitModel, UnlitUniforms};
use inverse_kinematics::{State};
use model::{Model};
use physics::{PhysicsFrame};
use render::render_context::{ModelId, DEPTH_DIMENSION};
use render::render_uniforms::{RenderUniforms};
use render::uniform_wrappers::{UMatrix4, UVector3};


pub struct RenderFrame {
	pub id: u64,
	pub models: Vec<(Arc<Model>, RenderUniforms)>,
	pub reverse_light_direction: UVector3,

	// DEBUG
	pub unlit_models: Vec<(Arc<UnlitModel>, UnlitUniforms)>,
}

impl RenderFrame {
	pub fn new(context: Arc<Context>, physics_frame: Arc<PhysicsFrame>) -> RenderFrame {
		let light_direction = physics_frame.light_direction;
		let reverse_light_direction = light_direction * -1.0;

		let shadow_view_projection = {
			let corners = physics_frame.camera.view_corners();

			// TODO: this should be constant/held somewhere
			//
			let shadow_width = {
				let outside_length:  f32 = (corners[4] - corners[1]).magnitude();
				let diagonal_length: f32 = (corners[1] - corners[0]).magnitude();
				outside_length.max(diagonal_length)
			};

			let up = Vector3::unit_y();
			let right = light_direction.cross(up).normalize();
			let up = right.cross(light_direction).normalize();

			let rotation_transposed = Matrix3::from_cols(right, up, reverse_light_direction);
			let rotation = rotation_transposed.transpose();

			// TODO: do something correct instead of this
			//
			let geometry_corners: Vec<Vector3<f32>> = vec![
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

			let world_units_per_texel = shadow_width / (DEPTH_DIMENSION as f32);
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
		};

		let translation = Matrix4::from_translation(physics_frame.player_position.to_vec());

		let up                  = Vector3::new(0f32, 1f32, 0f32);
		let flat_view_direction = (Vector3 { y: 0f32, .. physics_frame.camera.view_direction() }).normalize();
		let right               = flat_view_direction.cross(up).normalize();
		let up                  = right.cross(flat_view_direction);
		let rotation = Matrix4::from_cols(
			right.extend(0f32),
			up.extend(0f32),
			(flat_view_direction * -1f32).extend(0f32),
			Vector4::unit_w());

		let model = translation * rotation;
		let model_view_projection = view_projection * model;
		let shadow = shadow_view_projection * model;

		let player_uniforms = RenderUniforms {
			shadow:                  UMatrix4(shadow),
			model:                   UMatrix4(model),
			model_view_projection:   UMatrix4(model_view_projection),
		};

		let mut models = vec![
			(context.render.models.get(&ModelId::Scene).unwrap().clone(), scene_uniforms.clone()),
			(context.render.models.get(&ModelId::Player).unwrap().clone(), player_uniforms),
		];

		{ // TODO: all this is constant
			const D: f32 = 8f32;
			const A: f32 = 40f32;

			let seed: &[_] = &[2, 2, 2, 2];
			let mut rng: StdRng = SeedableRng::from_seed(seed);
			let range = Range::new(0f32, PI * 0.5);

			let zs = [D * -3.0, D * 3.0, D * -2.0, D * 2.0, D, -D, 0.0, A, -A];

			for &z in &zs {
				let transform =
					Matrix4::from_translation(Vector3::new(A, 0.0, z)) *
					Matrix4::from_angle_y(Rad(range.ind_sample(&mut rng)));

				let uniforms = RenderUniforms {
					shadow:                UMatrix4(shadow_view_projection * transform),
					model:                 UMatrix4(transform),
					model_view_projection: UMatrix4(view_projection * transform),
				};
				models.push((context.render.models.get(&ModelId::Tree).unwrap().clone(), uniforms));
			}
		}

		let mut unlit_models = {
			let scale = Matrix4::from_scale(3.0);
			let smvp = model_view_projection * scale;
			let svp  =       view_projection * scale;
			let scene_uniforms  = UnlitUniforms { model_view_projection: UMatrix4(smvp) };
			let player_uniforms = UnlitUniforms { model_view_projection: UMatrix4(svp) };

			vec![
				(context.render.unlit_models.get(&ModelId::Gnomon).unwrap().clone(), scene_uniforms),
				(context.render.unlit_models.get(&ModelId::Gnomon).unwrap().clone(), player_uniforms),
			]
		};

		for chain in &physics_frame.ik_chains {
			let transforms = chain.visible_joint_transforms();
			let offset = Matrix4::from_translation(chain.position);

			for joint in transforms {
				let joint = offset * joint;
				let shadow = shadow_view_projection * joint;
				let mvp = view_projection * joint;

				let uniforms = RenderUniforms {
					shadow:                UMatrix4(shadow),
					model:                 UMatrix4(joint),
					model_view_projection: UMatrix4(mvp),
				};
				models.push((context.render.models.get(&ModelId::IKModel).unwrap().clone(), uniforms));

				let mvp = mvp * Matrix4::from_scale(2.0);

				let unlit_uniforms = UnlitUniforms { model_view_projection: UMatrix4(mvp) };

				unlit_models.push((context.render.unlit_models.get(&ModelId::Gnomon).unwrap().clone(), unlit_uniforms));
			}

			match chain.state {
				State::Seeking { target, .. } | State::Waiting { target, .. } => {
					let target = view_projection * offset * Matrix4::from_translation(target);
					let unlit_uniforms = UnlitUniforms { model_view_projection: UMatrix4(target) };

					unlit_models.push((context.render.unlit_models.get(&ModelId::Indicator).unwrap().clone(), unlit_uniforms));
				},
				_ => ()
			};
		}

		RenderFrame {
			id: physics_frame.frame_counter,
			models: models,
			reverse_light_direction: UVector3(reverse_light_direction),
			unlit_models: unlit_models,
		}
	}
}
