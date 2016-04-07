use std::sync::{Arc};

use cgmath::{Vector3, Point3};

use context::{Context, ContextType};
use camera::{Camera};
use physics::{PhysicsFrame};
use constants::{NANOSECONDS_PER_SECOND};
use context_state::{ContextState, ContextStateTrait};


const FREQUENCY: u64 = 120; // ticks/second

// TODO: remove pub
pub struct PhysicsContext {
	state: ContextState<PhysicsFrame>
}

impl PhysicsContext {
	pub fn new(width: u32, height: u32) -> PhysicsContext {
		PhysicsContext {
			state: ContextState::new( PhysicsFrame {
				camera: Camera::new(width, height),
				player_position: Point3::new(0f64, 1f64, 0f64),
			}),
		}
	}

	pub fn get_frame(&self) -> Arc<PhysicsFrame> {
		let t_n1 = self.state.frame.read().unwrap();

		(*t_n1).clone()
	}
}

impl Context for PhysicsContext {
	fn rate(&self) -> u64 {
		NANOSECONDS_PER_SECOND / FREQUENCY
	}

	fn tick(&self, contexts: Arc<ContextType>) {
		let mut acceleration = Vector3::new(0f64, 0f64, 0f64);

		// last InputFrame wins
		// TODO: generalize and factor out all integration
		//
		let input_frames = contexts.context_input().get_input_frames();
		if let Some(frame) = input_frames.last() {
			let input_direction = frame.action_state.movement_direction;
			let direction = Vector3::new(input_direction.y, 0f64, input_direction.x);
			const FUDGE: f64 = 1f64;
			acceleration = acceleration + (direction * FUDGE);
		}

		let new_frame = Arc::new({
			let player_position = { // The locks sort of show in what way the state dependencies are separated
				let last_frame = self.state.frame.read().unwrap().clone();
				last_frame.player_position + acceleration
			};
			let camera = self.state.frame.read().unwrap().camera.clone();

			PhysicsFrame {
				camera: camera,
				player_position: player_position,
			}
		});

		{
			let mut self_frame_ref = self.state.frame.write().unwrap();
			*self_frame_ref = new_frame;
		}
	}

	fn is_ready(&self) -> bool { self.state.is_ready() }
	fn pre_tick(&self)         { self.state.frame_counter.increment(); }
	fn post_tick(&self)        { self.state.end_tick(); }
}
