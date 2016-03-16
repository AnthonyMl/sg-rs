#![feature(fnbox)]

#[macro_use]
extern crate glium;
extern crate crossbeam;
extern crate time;

use std::sync::{Arc};

mod render_context;
use render_context::{RenderContext};
mod physics_context;
use physics_context::{PhysicsContext};
mod thread_pool;
use thread_pool::{ThreadPool};


// TODO:
// job should have priority
// job should have estimated length
// 		s.t. the gl thread only does short, high priority tasks
//

// --- THREADING MODEL ---
// 1 gl / worker thread (minimize gl tasks to actual gl calls)
// (n-1) [ 3 ] worker threads
// 1 game loop thread (constantly yielding/sleeping)
//
fn main() {
	let (rc, mut rp) = render_context::create(); // main thread is ui thread
	let pc = PhysicsContext::new();

	const NUM_WORKER_THREADS: usize = 3;
	let pool = Arc::new(Box::new(ThreadPool::new(NUM_WORKER_THREADS)));
	let pool_ref = pool.clone();

	std::thread::spawn(move || {
		game_loop((rc, pc), pool_ref); // lightweight game_loop thread
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

fn game_loop((render_context, physics_context): (RenderContext, PhysicsContext), pool: Arc<Box<ThreadPool>>) {
	const RATE_INPUT:   u64 =  8333333; // 120 hz
	const RATE_PHYSICS: u64 =  8333333; // 120 hz
	const RATE_RENDER:  u64 = 16666666; // 60 hz

	// <frame_state>
	let mut last_time_input = time::precise_time_ns();
	let mut last_time_physics = last_time_input;
	let mut last_time_render = last_time_input;
	let mut frame_number_input   = 0;
	let mut frame_number_physics = 0;
	let mut frame_number_render  = 0;
	// <\frame_state>

	let rc = Arc::new(render_context);
	let pc = Arc::new(physics_context);

	loop {
		let time = time::precise_time_ns();

		while time - last_time_input > RATE_INPUT {
			last_time_input += RATE_INPUT;
			frame_number_input += 1;
			pool.post(Box::new(move || {
				handle_input(frame_number_input);
			}));
		}
		while time - last_time_physics > RATE_PHYSICS {
			last_time_physics += RATE_PHYSICS;
			frame_number_physics += 1;
			let local_pc = pc.clone();
			pool.post(Box::new(move || {
				local_pc.tick(frame_number_physics);
			}));
		}
		while time - last_time_render > RATE_RENDER {
			last_time_render += RATE_RENDER;
			frame_number_render += 1;
			let local_rc = rc.clone();
			pool.post(Box::new(move || {
				local_rc.tick(frame_number_render);
 			}));
		}

		std::thread::yield_now();
	}
}

fn handle_input(_frame_number: usize) {

}
