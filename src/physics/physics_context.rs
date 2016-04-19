use std::sync::{Arc, RwLock};
use std::default::{Default};

use cgmath::{Point3};

use context::{Context, ContextType, ContextState};
use camera::{Camera};
use physics::{PhysicsFrame};
use frame::{Frame};


pub struct PhysicsContext {
	pub state: ContextState,
}

impl PhysicsContext {
	pub fn new(width: u32, height: u32) -> PhysicsContext {
		PhysicsContext {
			state: ContextState::new( Frame::Physics(Arc::new( PhysicsFrame {
						camera: Camera::new(width, height),
						player_position: Point3::new(0f64, 1f64, 0f64),
						last_input_frame: Arc::new(RwLock::new(Default::default())),
			}))),
		}
	}

	pub fn get_frame(&self) -> Arc<PhysicsFrame> {
		(match self.state().frame() {
			Frame::Physics(physics_frame) => Some(physics_frame),
			_ => None,
		}).unwrap()
	}
}

unsafe impl Send for PhysicsContext {}
unsafe impl Sync for PhysicsContext {}

impl Context for PhysicsContext {
	fn frequency(&self) -> u64 { 120 }

	fn tick(&self, contexts: Arc<ContextType>, frame: Frame) -> Frame {
		Frame::Physics(Arc::new(PhysicsFrame::new(contexts, frame)))
	}

	fn state(&self) -> &ContextState { &self.state }
}
