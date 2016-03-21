#![feature(fnbox)]

#[macro_use]
extern crate glium;
extern crate crossbeam;
extern crate time;
extern crate cgmath;

mod context;
mod render;
mod physics_context;
mod input_context;
mod thread_pool;
mod vertex3;
mod camera;
mod uniform_wrappers;

use std::sync::{Arc};

use input_context::{InputContext};
use physics_context::{PhysicsContext};
use thread_pool::{ThreadPool};
use context::{Context};


// TODO:
// job should have priority
// job should have estimated length
// 		s.t. the gl thread only does short, high priority tasks
//

type ContextType = Arc<Vec<Arc<Context + Send + Sync + 'static>>>;

// --- THREADING MODEL ---
// 1 gl / worker thread (minimize gl tasks to actual gl calls)
// (n-1) worker threads where n is the number of cores on the system
// 1 lightweight game loop thread (constantly yielding/sleeping)
//
fn main() {
	const WIDTH: usize = 640;
	const HEIGHT: usize = 480;

	let (rc, mut rp) = render::create(WIDTH, HEIGHT); // main thread is ui thread

	const NUM_WORKER_THREADS: usize = 3;
	let pool = Arc::new(Box::new(ThreadPool::new(NUM_WORKER_THREADS)));
	let pool_ref = pool.clone();

	let contexts: ContextType = Arc::new(vec![
		Arc::new(InputContext::new()),
		Arc::new(PhysicsContext::new()),
		Arc::new(rc),
	]);

	std::thread::spawn(move || {
		game_loop(contexts, pool_ref); // lightweight game_loop thread
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

		let iter = contexts.iter().zip(last_times.iter_mut());

		for (context, last_time) in iter {
			let rate = context.rate();

			while time - *last_time > rate {
				*last_time = *last_time + rate;

				let local = context.clone();
				pool.post(Box::new(move || {
					local.tick();
				}));
			}
		}
		std::thread::yield_now();
	}
}
