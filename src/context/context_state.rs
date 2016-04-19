use std::sync::{RwLock};
use std::sync::atomic::{AtomicBool, Ordering};

use frame_counter::{FrameCounter};
use frame::{Frame};


// TODO: do something different if T is unsized
//
pub struct ContextState {
	frame_counter: FrameCounter,
	ready_lock: AtomicBool,
	frame: RwLock<Frame>,
}

impl ContextState {
	pub fn new(frame: Frame) -> ContextState {
		ContextState {
			frame_counter: FrameCounter::new(0),
			ready_lock: AtomicBool::new(true),
			frame: RwLock::new(frame),
		}
	}

	pub fn frame_counter(&self) -> u64 {
		self.frame_counter.get()
	}

	pub fn frame(&self) -> Frame {
		(*self.frame.read().unwrap()).clone()
	}

	pub fn tick_enter(&self) -> u64 {
		self.frame_counter.increment()
	}

	pub fn tick_exit(&self, frame: Frame) {
		self.set_frame(frame);

		self.release_ready_lock();
	}

	pub fn is_ready(&self) -> bool {
		self.ready_lock.compare_and_swap(true, false, Ordering::Relaxed)
	}

	fn set_frame(&self, frame: Frame) {
		let mut frame_write = self.frame.write().unwrap();
		*frame_write = frame;
	}

	fn release_ready_lock(&self) {
		self.ready_lock.store(true, Ordering::Relaxed);
	}
}
