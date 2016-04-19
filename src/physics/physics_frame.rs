use std::sync::{Arc, RwLock};

use cgmath::{Point3, Vector3};

use camera::{Camera};
use input::{InputFrame};
use frame::{Frame};
use context::{ContextType};


pub struct PhysicsFrame {
	pub camera: Camera,
	pub last_input_frame: Arc<RwLock<InputFrame>>,
	pub player_position: Point3<f64>,
}

impl PhysicsFrame {
	pub fn new(contexts: Arc<ContextType>, frame: Frame) -> PhysicsFrame {
		let frame = (match frame {
			Frame::Physics(physics_frame) => Some(physics_frame),
			_ => None,
		}).unwrap();

		let mut acceleration = Vector3::new(0f64, 0f64, 0f64);

		// last InputFrame wins
		// TODO: generalize and factor out all integration
		//
		let input_frame_ref = {
			if let Some(frame) = contexts.context_input().get_input_frames().pop() {
				Arc::new(RwLock::new(frame))
			} else {
				frame.last_input_frame.clone()
			}
		};

		// TODO: add some mechanism to force input_frame_ref.read() [and things like it] to only be called once per tick
		//
		let input_direction = { input_frame_ref.read().unwrap().action_state.movement_direction };

		let direction = Vector3::new(input_direction.y, 0f64, input_direction.x);
		const FUDGE: f64 = 0.1f64;
		acceleration = acceleration + (direction * FUDGE);

		let player_position = frame.player_position + acceleration;
		let camera = frame.camera.clone();

		PhysicsFrame {
			camera: camera,
			last_input_frame: input_frame_ref,
			player_position: player_position,
		}
	}
}
