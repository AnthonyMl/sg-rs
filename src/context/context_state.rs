use std::sync::atomic::{AtomicBool, Ordering};

use frame_counter::{FrameCounter};


// TODO: these Relaxed seem suspicious (do we need any seqcst?)
//
pub struct ContextState {
	frame_counter: FrameCounter,
	ready_lock: AtomicBool,
}

impl ContextState {
	pub fn new() -> ContextState {
		ContextState {
			frame_counter: FrameCounter::new(0),
			ready_lock: AtomicBool::new(true),
		}
	}

	pub fn frame_counter(&self) -> u64 {
		self.frame_counter.get()
	}

	pub fn increment(&self) -> u64 {
		self.frame_counter.increment()
	}

	pub fn is_ready(&self) -> bool {
		self.ready_lock.compare_and_swap(true, false, Ordering::Relaxed)
	}

	pub fn release_ready_lock(&self) {
		self.ready_lock.store(true, Ordering::Relaxed);
	}
}
