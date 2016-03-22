use frame_counter::{FrameCounter};
use context::{Context};


pub struct InputContext {
	frame_counter: FrameCounter,
}

impl InputContext {
	pub fn new() -> InputContext {
		InputContext {
			frame_counter: FrameCounter::new(0),
		}
	}
}

impl Context for InputContext {
	fn rate(&self) -> u64 {
		8333333 // 120 hz
	}

	fn tick(&self) {
		self.frame_counter.increment();
	}
}
