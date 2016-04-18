use std::sync::{Arc, RwLock};
use std::default::{Default};

use cgmath::{Vector3, Point3};

use context::{Context, ContextType, ContextState, ContextStateProxy};
use camera::{Camera};
use physics::{PhysicsFrame};


pub struct PhysicsContext {
	state: ContextState<PhysicsFrame>,
}

impl PhysicsContext {
	pub fn new(width: u32, height: u32) -> PhysicsContext {
		PhysicsContext {
			state: ContextState::new( PhysicsFrame {
					camera: Camera::new(width, height),
					player_position: Point3::new(0f64, 1f64, 0f64),
					last_input_frame: Arc::new(RwLock::new(Default::default())),
			}),
		}
	}

	pub fn get_frame(&self) -> Arc<PhysicsFrame> { self.state.frame() }
}

impl Context for PhysicsContext {
	fn frequency(&self) -> u64 { 120 }

	fn tick(&self, contexts: Arc<ContextType>) {
		let mut acceleration = Vector3::new(0f64, 0f64, 0f64);

		// last InputFrame wins
		// TODO: generalize and factor out all integration
		//
		let input_frame_ref = {
			if let Some(frame) = contexts.context_input().get_input_frames().pop() {
				Arc::new(RwLock::new(frame))
			} else {
				self.state.frame().last_input_frame.clone()
			}
		};

		// TODO: add some mechanism to force input_frame_ref.read() [and things like it] to only be called once per tick
		//
		let input_direction = { input_frame_ref.read().unwrap().action_state.movement_direction };

		let direction = Vector3::new(input_direction.y, 0f64, input_direction.x);
		const FUDGE: f64 = 0.1f64;
		acceleration = acceleration + (direction * FUDGE);

		let player_position = { // The locks sort of show in what way the state dependencies are separated
			let last_frame = self.state.frame();
			last_frame.player_position + acceleration
		};
		let camera = self.state.frame().camera.clone();

		let new_frame = Arc::new({ PhysicsFrame {
			camera: camera,
			player_position: player_position,
			last_input_frame: input_frame_ref,
		}});

		self.state.set_frame(new_frame);
	}

	fn state(&self) -> &ContextStateProxy { &self.state }
}
