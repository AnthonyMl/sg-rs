use std::sync::{Arc};
use std::sync::atomic::{AtomicUsize, Ordering};

use mioco::{CoroutineControl, Handler, Scheduler, SchedulerThread};
use mioco::mio::{EventLoop};


pub struct BalancingScheduler {
	thread_counter: AtomicUsize,
	length_refs: Arc<Vec<AtomicUsize>>,
}

impl BalancingScheduler {
	pub fn new(num_threads: usize) -> BalancingScheduler {
		let mut length_refs = Vec::new();
		for _ in 0..num_threads { length_refs.push(AtomicUsize::new(0)); }

		BalancingScheduler {
			thread_counter: AtomicUsize::new(0),
			length_refs: Arc::new(length_refs),
		}
	}
}

unsafe impl Send for BalancingScheduler {}
unsafe impl Sync for BalancingScheduler {}

impl Scheduler for BalancingScheduler {
	fn spawn_thread(&self) -> Box<SchedulerThread> {
		let thread_id = self.thread_counter.fetch_add(1, Ordering::Relaxed);

		let thread = Box::new(BalancingSchedulerThread {
			counter:     0,
			thread_id:   thread_id,
			q:           Vec::new(),
			length_refs: self.length_refs.clone(),
		});

		thread
	}
}

struct BalancingSchedulerThread {
	counter:     usize,
	thread_id:   usize,
	q:           Vec<CoroutineControl>,
	length_refs: Arc<Vec<AtomicUsize>>,
}

impl SchedulerThread for BalancingSchedulerThread {
	fn spawned(&mut self, event_loop: &mut EventLoop<Handler>, coroutine_ctrl: CoroutineControl) {
		let (thread_id, min) = self.length_refs.iter().enumerate().fold(
			(0, usize::max_value()),
			|(min_id, min), (thread_id, length)| {
				let len = length.load(Ordering::Relaxed);

				if len < min { (thread_id, len) }
				else         { (min_id,    min) }
			}
		);

		let thread_id = if min == 0 {
			self.counter = (self.counter + 1) % self.length_refs.len();
			self.counter
		} else {
			thread_id
		};

		if thread_id == self.thread_id { coroutine_ctrl.resume(event_loop); }
		else                           { coroutine_ctrl.migrate(event_loop, thread_id); }
	}

	fn ready(&mut self, event_loop: &mut EventLoop<Handler>, coroutine_ctrl: CoroutineControl) {
		if coroutine_ctrl.is_yielding() {
			self.q.push(coroutine_ctrl);
			self.length_refs[self.thread_id].fetch_add(1, Ordering::Relaxed);
		} else {
/*			self.counter = (self.counter + 1) % self.length_refs.len();
			if self.counter == self.thread_id { coroutine_ctrl.resume(event_loop); }
			else                              { coroutine_ctrl.migrate(event_loop, self.counter); }
*/			coroutine_ctrl.resume(event_loop);
		}
	}

	fn tick(&mut self, event_loop: &mut EventLoop<Handler>) {
		while let Some(coroutine_ctrl) = self.q.pop() {
			coroutine_ctrl.resume(event_loop); // or migrate them here
			self.length_refs[self.thread_id].fetch_sub(1, Ordering::Relaxed);
		}
	}

	fn timeout(&mut self) -> Option<u64> { None }
}
