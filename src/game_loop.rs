use std::thread;
use std::sync::{Arc};

use time;
use context;
use thread_pool::{ThreadPool};
use context::{ContextType};


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
	let (context, mut rp) = context::create(); // main thread is ui thread

	const NUM_WORKER_THREADS: usize = 3;
	let pool = Arc::new(Box::new(ThreadPool::new(NUM_WORKER_THREADS)));

	{
		let pool_ref = pool.clone();
		let context_ref = context.clone();

		thread::spawn(move || {
			game_loop(context_ref, pool_ref); // lightweight game_loop thread
		});
	}

	// TODO: may need to be refactored to handle system events more frequently/(lower max potential latency)
	//
	while let Some(events) = rp.handle_system_events() {
		context.input().post_input_events(events);

		rp.handle_render_commands();

		// TODO: only do this if we have no/few render commands (or low time spent on render commands)
		//
		if let Some(f) = pool.steal() { f() }
	}
//	pool.wait(); // TODO: figure out how to sync this so that we can grab ownership of the pool safely
}

fn game_loop(contexts: Arc<ContextType>, pool: Arc<Box<ThreadPool>>) -> ! {
	let mut last_times: Vec<u64> = vec![time::precise_time_ns(); contexts.contexts().len()];

	loop {
		let time = time::precise_time_ns();

		'next_context: for (context, last_time) in contexts.contexts().into_iter().zip(last_times.iter_mut()) {
			const NANOSECONDS_PERS_SECOND: u64 = 1000000000;
			let rate = NANOSECONDS_PERS_SECOND / context.frequency();

			while time - *last_time > rate {
				let local_context  = context.clone();
				let local_contexts = contexts.clone();

				if !local_context.is_ready(contexts.clone()) { continue 'next_context}

				*last_time = *last_time + rate;

				pool.post(Box::new(move || {
					local_context.tick(local_contexts.clone());
				}));
			}
		}
		thread::yield_now();
	}
}
