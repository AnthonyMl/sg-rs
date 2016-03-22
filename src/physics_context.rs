use frame_counter::{FrameCounter};
use context::{Context};


pub struct PhysicsContext {
	frame_counter: FrameCounter,
}

impl PhysicsContext {
	pub fn new() -> PhysicsContext {
		PhysicsContext {
			frame_counter: FrameCounter::new(0),
		}
	}
}

impl Context for PhysicsContext {
	fn rate(&self) -> u64 {
		8333333 // 120 hz
	}

	fn tick(&self) {
		self.frame_counter.increment();
	}
}
