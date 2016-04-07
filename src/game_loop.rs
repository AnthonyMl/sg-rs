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
	const WIDTH: u32 = 1280;
	const HEIGHT: u32 = 800;

	let (context, mut rp) = context::create(WIDTH, HEIGHT); // main thread is ui thread

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
	loop {
		match rp.handle_system_events() {
			Some(es) => context.context_input().post_input_events(es),
			None     => break,
		};

		rp.handle_render_commands();

		// TODO: only do this if we have no/few render commands (or low time spent on render commands)
		//
		if let Some(f) = pool.steal() { f() }
	}
//	pool.wait(); // TODO: figure out how to sync this so that we can grab ownership of the pool safely
}

fn game_loop(contexts: Arc<ContextType + Send + Sync>, pool: Arc<Box<ThreadPool>>) -> ! {
	let time = time::precise_time_ns();

	// TODO: can this be refactored into one line
	//
	let mut last_times: Vec<u64> = Vec::with_capacity(contexts.len());
	for _ in 0..contexts.len() { last_times.push(time) }

	loop {
		let time = time::precise_time_ns();

		'next_context: for (context, last_time) in contexts.contexts().into_iter().zip(last_times.iter_mut()) {
			let rate = context.rate();

			while time - *last_time > rate {
				if !context.is_ready() { continue 'next_context}

				*last_time = *last_time + rate;

				let local_context  = context.clone();
				let local_contexts = contexts.clone();
				pool.post(Box::new(move || {
					local_context.pre_tick();
					local_context.tick(local_contexts);
					local_context.post_tick();
				}));
			}
		}
		thread::yield_now();
	}
}
