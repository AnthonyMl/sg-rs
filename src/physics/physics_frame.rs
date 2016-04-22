use std::sync::{Arc};

use cgmath::{Point3, Vector3, Vector2, EuclideanVector};

use camera::{Camera};
use input::{InputFrame};
use context::{ContextType};


pub struct PhysicsFrame {
	pub camera: Camera,
	pub last_input_frame: InputFrame,
	pub player_position: Point3<f64>,
	pub view_direction: Vector3<f64>,
}

impl PhysicsFrame {
	pub fn frame_zero(window_size: (u32, u32)) -> PhysicsFrame {
		let player_position = Point3::new(0f64, 1f64, 0f64);
		let view_direction = Vector3::new(0f64, 0f64, 1f64);

		PhysicsFrame {
			camera:           Camera::new(player_position, view_direction, window_size),
			player_position:  player_position,
			view_direction:   view_direction,
			last_input_frame: Default::default(),
		}
	}

	pub fn new(context: Arc<ContextType>, frame: Arc<PhysicsFrame>) -> PhysicsFrame {
		let mut input_frames = context.input().get_input_frames();

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

		// TODO: add some mechanism to force input_frame_ref.read() [and things like it] to only be called once per tick
		//
		let last_input_frame = if let Some(frame) = input_frames.pop() {
			frame
		} else {
			frame.last_input_frame.clone() // TODO: is it inefficient to make a copy instead of sharing a pointer?
		};
		let input_direction = last_input_frame.action_state.movement_direction;

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
			camera: camera,
			last_input_frame: last_input_frame,
			player_position: player_position,
			view_direction: view_direction,
		}
	}
}
