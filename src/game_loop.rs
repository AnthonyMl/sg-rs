use std::thread;
use std::sync::{Arc};

use time;
use context;
use thread_pool::{ThreadPool};
use context::{Context, ContextType};


// TODO:
// job should have priority
// job should have estimated length
// 		s.t. the gl thread only does short, high priority tasks
//

// --- THREADING MODEL ---
// 1 gl / worker thread (minimize gl tasks to actual gl calls)
// (n-1) worker threads where n is the number of cores on the system
// 1 lightweight game loop thread (constantly yielding/sleeping)
//
pub fn init() {
	const WIDTH: usize = 640;
	const HEIGHT: usize = 480;

	let (context, mut rp) = context::create(WIDTH, HEIGHT); // main thread is ui thread

	const NUM_WORKER_THREADS: usize = 3;
	let pool = Arc::new(Box::new(ThreadPool::new(NUM_WORKER_THREADS)));
	let pool_ref = pool.clone();

	thread::spawn(move || {
		game_loop(context, pool_ref); // lightweight game_loop thread
	});

	loop {
		if rp.handle_system_events() { break }

		rp.handle_render_commands();

		match pool.steal() {
			Some(f) => f(),
			None => ()
		}
	}
//	pool.wait(); // TODO: figure out how to sync this so that we can grab ownership of the pool safely
}

fn game_loop(contexts: ContextType, pool: Arc<Box<ThreadPool>>) -> ! {
	let time = time::precise_time_ns();

	let mut last_times: Vec<u64> = Vec::with_capacity(contexts.len());
	for _ in 0..contexts.len() { last_times.push(time) }

	loop {
		let time = time::precise_time_ns();

		for (context, last_time) in contexts.iter().zip(last_times.iter_mut()) {
			let rate = context.rate();

			while time - *last_time > rate {
				*last_time = *last_time + rate;

				let local = context.clone();
				pool.post(Box::new(move || {
					local.tick();
				}));
			}
		}
		thread::yield_now();
	}
}
