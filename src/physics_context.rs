use std::sync::atomic::{AtomicUsize, Ordering};

use context::{Context};


pub struct PhysicsContext {
	frame_number: AtomicUsize,
}

impl PhysicsContext {
	pub fn new() -> PhysicsContext {
		PhysicsContext {
			frame_number: AtomicUsize::new(0),
		}
	}
}

impl Context for PhysicsContext {
	fn rate(&self) -> u64 {
		8333333 // 120 hz
	}

	fn tick(&self) {
		loop {
			let v = self.frame_number.load(Ordering::Acquire);
			if v == self.frame_number.compare_and_swap(v, v + 1, Ordering::Release) { break }
		}
	}
}
