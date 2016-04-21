use std::sync::{Arc};
use std::default::{Default};

use cgmath::{Point3, Vector3};

use context::{ContextState};
use camera::{Camera};
use physics::{PhysicsFrame};
use frame::{Frame};


pub struct PhysicsContext {
	pub state: ContextState,
}

impl PhysicsContext {
	pub fn new(window_size: (u32, u32)) -> PhysicsContext {
		let player_position = Point3::new(0f64, 1f64, 0f64);
		let view_direction = Vector3::new(0f64, 0f64, 1f64);

		PhysicsContext {
			state: ContextState::new( Frame::Physics(Arc::new( PhysicsFrame {
						camera: Camera::new(player_position, view_direction, window_size),
						player_position: player_position,
						view_direction: view_direction,
						last_input_frame: Default::default(),
			}))),
		}
	}

	pub fn get_frame(&self) -> Arc<PhysicsFrame> {
		(match self.state.frame() {
			Frame::Physics(physics_frame) => Some(physics_frame),
			_ => None,
		}).unwrap()
	}
}

unsafe impl Send for PhysicsContext {}
unsafe impl Sync for PhysicsContext {}
