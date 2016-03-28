use std::sync::{Arc, RwLock};

use frame_counter::{FrameCounter};
use context::{Context, ContextType};
use camera::{Camera};
use physics::{PhysicsFrame};


const FREQUENCY: u64 = 120;

pub struct PhysicsContext {
	frame_counter: FrameCounter,
	frame: RwLock<Arc<PhysicsFrame>>,
}

impl PhysicsContext {
	pub fn new(width: u32, height: u32) -> PhysicsContext {
		PhysicsContext {
			frame_counter: FrameCounter::new(0),
			frame: RwLock::new(Arc::new(PhysicsFrame { camera: Camera::new(width, height) })),
		}
	}

	pub fn get_frame(&self) -> Arc<PhysicsFrame> {
		let t_n1 = self.frame.read().unwrap();

		(*t_n1).clone()
	}
}

impl Context for PhysicsContext {
	fn rate(&self) -> u64 {
		1000000000 / FREQUENCY
	}

	fn tick(&self, _contexts: Arc<ContextType>) {
		self.frame_counter.increment();

		let new_frame = Arc::new({
			let t_n1 = self.frame.read().unwrap();

			PhysicsFrame {
				camera: t_n1.camera.clone(),
			}
		});

		{
			let mut self_frame_ref = self.frame.write().unwrap();

			*self_frame_ref = new_frame;
		}
	}
}
