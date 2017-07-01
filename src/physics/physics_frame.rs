use std::sync::{Arc};
use std::f32::consts::{PI};

use cgmath::{Point3, Vector3, InnerSpace};
use rand;
use rand::distributions::{IndependentSample, Range};

use camera::{Camera, to_view_direction};
use context::{Context};
use input::{InputFrame};
use inverse_kinematics::{Axis, Chain, Joint, State, Transition, updater};
use inverse_kinematics::cyclic_coordinate_descent::{cyclic_coordinate_descent};
use inverse_kinematics::jacobian_transpose::{jacobian_transpose};


// TODO: put in a soft cap on elevation with a slow drift
//
pub struct PhysicsFrame {
	pub frame_counter:   u64,
	pub camera:          Camera,
	pub player_position: Point3<f32>,
	pub ik_chains:       Vec<Chain>,

	pub light_direction: Vector3<f32>,
	pub aspect_ratio:    f32,
}

impl PhysicsFrame {
	pub fn frame_zero(aspect_ratio: f32) -> PhysicsFrame {
		let light_direction = Vector3::new(0.4, -1.0, -0.6).normalize();
		let player_position = Point3::new(0f32, 1f32, 0f32);
		let camera = Camera::new(player_position, 0.0, 0.0, aspect_ratio);
		let ik_chains = vec![
			Chain {
				joints: vec![
					Joint { length: 0.0, axis: Axis::Y },
					Joint { length: 3.0, axis: Axis::X },
					Joint { length: 3.0, axis: Axis::X },
					Joint { length: 3.0, axis: Axis::X }
				],
				angles: vec![0.0, 0.0, 0.0, 0.1],
				state: State::Done,
				position: Vector3::new(59.0, -9.0, -9.0),
				ik_fun: cyclic_coordinate_descent,
			},
			Chain {
				joints: vec![
					Joint { length: 0.0, axis: Axis::Y },
					Joint { length: 3.0, axis: Axis::X },
					Joint { length: 3.0, axis: Axis::X },
					Joint { length: 3.0, axis: Axis::X }
				],
				angles: vec![0.0, 0.0, 0.0, 0.1],
				state: State::Done,
				position: Vector3::new(59.0, -9.0, 9.0),
				ik_fun: jacobian_transpose,
			}
		];

		PhysicsFrame {
			frame_counter:   0,
			camera:          camera,
			player_position: player_position,
			ik_chains:       ik_chains,
			light_direction: light_direction,
			aspect_ratio:    aspect_ratio,
		}
	}

	pub fn new(context: Arc<Context>, frame: Arc<PhysicsFrame>, input_frame: Arc<InputFrame>) -> PhysicsFrame {
		let angles_delta = -input_frame.view_angles_delta; // TODO: scale
		let camera = frame.camera.update(frame.player_position, angles_delta.x, angles_delta.y, context.render.aspect_ratio());

		let player_position = {
			let view_direction = camera.view_direction();
			let right = view_direction.cross(Vector3::new(0f32, 1f32, 0f32)).normalize();

			let input_direction = input_frame.movement_delta; // TODO: scale

			let flat_view_direction = (Vector3 { y: 0f32, .. view_direction }).normalize();
			let flat_right          = (Vector3 { y: 0f32, ..          right }).normalize();

			// TODO: generalize and factor out all integration
			//
			const FUDGE: f32 = 0.1f32;
			let acceleration
				= flat_view_direction * input_direction.x * FUDGE
				+ flat_right          * input_direction.y * FUDGE;

			frame.player_position + acceleration
		};
		let target = sphere_point(9.0);
		let ik_chains = frame.ik_chains.iter().map(|chain| {
			if chain.state == State::Done {
				updater::update(chain, Transition::NewTarget{
					target: target,
					num_transition_frames: 180
				})
			} else {
				updater::update(chain, Transition::Update)
			}
		}).collect();

		PhysicsFrame {
			frame_counter: frame.frame_counter + 1,
			camera: camera,
			player_position: player_position,
			ik_chains: ik_chains,

			light_direction: frame.light_direction,
			aspect_ratio: frame.aspect_ratio,
		}
	}
}

fn sphere_point(radius: f32) -> Vector3<f32> {
	let mut rng = rand::thread_rng();

	let azimuth_range = Range::new(0.0, PI);
	let elevation_range = Range::new(PI * 0.5, PI * 0.75);
	let radius_range = Range::new(radius * 0.2, radius);

	let unit = to_view_direction(
		azimuth_range.ind_sample(&mut rng),
		elevation_range.ind_sample(&mut rng)
	);

	unit * radius_range.ind_sample(&mut rng)
}
