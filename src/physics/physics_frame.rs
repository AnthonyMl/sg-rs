use std::sync::{Arc};

use cgmath::{Point3, Vector3, Vector2, InnerSpace};

use camera::{Camera};
use context::{Context};


pub struct PhysicsFrame {
	pub frame_counter: u64,
	pub camera: Camera,
	pub player_position: Point3<f64>,
	pub view_direction: Vector3<f64>,
}

impl PhysicsFrame {
	pub fn frame_zero(window_size: (u32, u32)) -> PhysicsFrame {
		let player_position = Point3::new(0f64, 1f64, 0f64);
		let view_direction = Vector3::new(0f64, 0f64, 1f64);

		PhysicsFrame {
			frame_counter:    0,
			camera:           Camera::new(player_position, view_direction, window_size),
			player_position:  player_position,
			view_direction:   view_direction,
		}
	}

	pub fn new(context: Arc<Context>, frame: Arc<PhysicsFrame>) -> PhysicsFrame {
		let input_frames = context.input().get_input_frames();

		let view_delta: Vector2<f64> = input_frames.iter().fold(
			Vector2::new(0f64, 0f64),
			|sum, input_frame| { sum + input_frame.action_state.view_direction }
		);
		let right = frame.view_direction.cross(Vector3::new(0f64, 1f64, 0f64)).normalize();
		let up = right.cross(frame.view_direction).normalize();
		let view_direction
			= frame.view_direction
			+ right * view_delta.x
			-    up * view_delta.y;

		let input_direction: Vector2<f64> = input_frames.iter().fold(
			Vector2::new(0f64, 0f64),
			|sum, input_frame| { sum + input_frame.action_state.movement_direction }
		);
		let flat_view_direction = (Vector3 { y: 0f64, .. view_direction}).normalize();
		let flat_right          = (Vector3 { y: 0f64, ..          right}).normalize();

		// TODO: generalize and factor out all integration
		//
		const FUDGE: f64 = 0.1f64;
		let acceleration
			= flat_view_direction * input_direction.x * FUDGE
			+ flat_right          * input_direction.y * FUDGE;

		let player_position = frame.player_position + acceleration;

		// TODO: this can be kicked up in a task at the start (if we use last frame's position)
		//
		let camera = Camera::new(player_position, view_direction, context.render().window_size());

		PhysicsFrame {
			frame_counter: context.counter_physics(),
			camera: camera,
			player_position: player_position,
			view_direction: view_direction,
		}
	}
}
