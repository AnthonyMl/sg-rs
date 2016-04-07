use std::sync::atomic::{AtomicUsize, Ordering};


pub struct FrameCounter {
	counter: AtomicUsize,
}

impl FrameCounter {
	pub fn new(value: usize) -> FrameCounter {
		FrameCounter {
			counter: AtomicUsize::new(value),
		}
	}

	pub fn get(&self) -> u64 {
		self.counter.load(Ordering::Relaxed) as u64
	}

	pub fn increment(&self) -> u64 {
		loop {
			let v = self.counter.load(Ordering::Acquire);
			if v == self.counter.compare_and_swap(v, v + 1, Ordering::Release) {
				return v as u64;
			}
		}
	}
}
