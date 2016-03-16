use std;
use std::vec::{Vec};
use std::sync::{Arc};
use std::boxed::{FnBox};
use crossbeam::sync::{MsQueue};


pub type TaskFn = Box<FnBox()->() + Send>;

pub struct ThreadPool {
	pub join_handles: Vec<std::thread::JoinHandle<()>>,
	q: Arc<MsQueue<TaskFn>>
}

impl ThreadPool {
	pub fn new(thread_count: usize) -> ThreadPool {
		let mut pool = ThreadPool {
			join_handles: Vec::with_capacity(thread_count),
			q: Arc::new(MsQueue::new())
		};
		for _ in 0..thread_count {
			let jobs = pool.q.clone();

			pool.join_handles.push(std::thread::spawn(move || {
				match jobs.try_pop() {
					Some(f) => f(),
					None => return,
				}
			}));
		}
		pool
	}

	pub fn post(&self, work: TaskFn) {
		self.q.push(work);
	}

	pub fn steal(&self) -> Option<TaskFn> {
		self.q.try_pop()
	}

	pub fn _wait(self) {
		for handle in self.join_handles {
			let _ = handle.join();
		}
	}
}
