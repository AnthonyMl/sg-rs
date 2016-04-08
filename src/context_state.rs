use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};

use frame_counter::{FrameCounter};


// TODO: do something different if T is unsized
//
pub struct ContextState<T> {
	frame_counter: FrameCounter,
	ready_lock: AtomicBool,
	frame: RwLock<Arc<T>>,
}

impl<T> ContextState<T> {
	pub fn new(frame: T) -> ContextState<T> {
		ContextState {
			frame_counter: FrameCounter::new(0),
			ready_lock: AtomicBool::new(true),
			frame: RwLock::new(Arc::new(frame)),
		}
	}

	pub fn frame_counter(&self) -> u64 {
		self.frame_counter.get()
	}

	pub fn frame(&self) -> Arc<T> {
		self.frame.read().unwrap().clone()
	}

	pub fn set_frame(&self, frame: Arc<T>) {
		let mut frame_write = self.frame.write().unwrap();
		*frame_write = frame;
	}

	fn is_ready(&self) -> bool {
		self.ready_lock.compare_and_swap(true, false, Ordering::Relaxed)
	}

	fn increment(&self) -> u64 {
		self.frame_counter.increment()
	}

	fn release_ready_lock(&self) {
		self.ready_lock.store(true, Ordering::Relaxed);
	}
}

pub trait ContextStateProxy {
	fn is_ready(&self) -> bool;
	fn pre_tick(&self);
	fn post_tick(&self);
}

impl<T> ContextStateProxy for ContextState<T> {
	fn is_ready(&self) -> bool { self.is_ready() }

	fn pre_tick(&self) { self.increment(); }

	fn post_tick(&self) { self.release_ready_lock(); }
}