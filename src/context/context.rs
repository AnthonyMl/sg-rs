use std::thread;
use std::sync::{Arc};
use std::sync::atomic::{AtomicBool, Ordering};

use crossbeam::sync::{MsQueue};
use glium::glutin::{WindowBuilder, CursorState, get_primary_monitor};
use glium::{DisplayBuild};
use mioco;
use mioco::{Mioco, Config};
use mioco::sync::{RwLock};
use mioco::sync::mpsc::{Receiver, Sender, channel};
use time;

use render::{RenderContext, RenderProcessor, RenderFrame};
use input::{InputContext, InputFrame};
use physics::{PhysicsContext, PhysicsFrame};
use scheduler::{BalancingScheduler};


// TODO: try to remove Arc dependency
//
// job should have priority
// job should have estimated length
// 		s.t. the gl thread only does short, high priority tasks
//

// --- THREADING MODEL ---
// n worker threads where n is the number of cores on the system
// 1 lightweight event processing / frame_kicking / gl thread (TODO: minimize gl tasks to actual gl calls)
//

unsafe impl Send for Context {}
unsafe impl Sync for Context {}

pub fn init() {
	let window_size = get_primary_monitor().get_dimensions();
	let window_size = (window_size.0/2, window_size.1/2);

	let glium_context = WindowBuilder::new()
		.with_dimensions(window_size.0, window_size.1)
		.with_title(format!("SG"))
		.with_depth_buffer(24)
		.with_decorations(false)
		.build_glium().unwrap();

	glium_context.get_window().unwrap().set_cursor_state(CursorState::Grab).ok();

	let q = Arc::new(MsQueue::new());

	let context = Arc::new(Context::new(
		InputContext  ::new(),                       InputFrame    ::frame_zero(),
		PhysicsContext::new(),                       PhysicsFrame  ::frame_zero(window_size),
		RenderContext ::new(q.clone(), window_size), RenderFrame   ::frame_zero(),)
	);

	let mut render_processor = RenderProcessor::new(q, glium_context);

	let (senders, receivers) = (vec![
		channel::<()>(),
		channel::<()>(),
		channel::<()>(),
	]).into_iter().unzip();

	let mut frame_kicker = FrameKicker::new(senders);

	{
		let ready_flags = vec![
			frame_kicker.input  .ready.clone(),
			frame_kicker.physics.ready.clone(),
			frame_kicker.render .ready.clone(),
		];
		let context = context.clone();
		thread::spawn(move || { spawn_coroutines(context, receivers, ready_flags); });
	}

	// TODO: may need to be refactored to handle system events more frequently/(lower max latency)
	//
	while let Some(events) = render_processor.handle_system_events() {
		context.input().post_input_events(events);

		frame_kicker.tick();

		render_processor.handle_render_commands();

		thread::yield_now();
	}
}

macro_rules! register_contexts {
	($({ $context_type:ty, $frame_type:ident, $name:ident, $fn_frame:ident, $fn_counter:ident, $frequency:expr }),* ) => {
		pub struct Context {
			$( $name: (Arc<$context_type>, RwLock<Arc<$frame_type>>), )*
		}

		impl Context {
			pub fn new($( $name: $context_type, $fn_frame: $frame_type, )*) -> Context {
				Context { $( $name: (
					Arc::new($name),
					RwLock::new(Arc::new($fn_frame)),
				), )* }
			}

			$(
				#[allow(dead_code)]
				pub fn $name(&self) -> Arc<$context_type> { self.$name.0.clone() }

				#[allow(dead_code)]
				pub fn $fn_frame(&self) -> Arc<$frame_type> {
					(self.$name.1.read().unwrap()).clone()
				}
			)*
		}

		struct KickData {
			ready:     Arc<AtomicBool>,
			sender:    Sender<()>,
			last_time: u64,
		}
		struct FrameKicker { $( pub $name: KickData, )* }

		impl FrameKicker {
			pub fn new(mut senders: Vec<Sender<()>>) -> FrameKicker {
				let time = time::precise_time_ns() / 1_000_000;

				FrameKicker { $( $name: KickData {
					ready:     Arc::new(AtomicBool::new(true)),
					sender:    senders.pop().unwrap(),
					last_time: time,
				}, )* }
			}

			pub fn tick(&mut self) {
				let time = time::precise_time_ns() / 1_000_000;
				$(
					while self.$name.last_time < time && self.$name.ready.load(Ordering::Relaxed) {
						self.$name.last_time += 1000 / $frequency;
						self.$name.sender.send(()).unwrap();
					}
				)*
			}
		}

		pub fn spawn_coroutines(context: Arc<Context>, mut receivers: Vec<Receiver<()>>, mut ready_flags: Vec<Arc<AtomicBool>>) {
			const NUM_THREADS: usize = 4;

			let mut config = Config::new();
			config.set_thread_num(NUM_THREADS);
			config.set_scheduler(Box::new(BalancingScheduler::new(NUM_THREADS)));

			Mioco::new_configured(config).start(move || { $(
				fn $fn_frame(context: Arc<Context>, receiver: Receiver<()>, ready_flag: Arc<AtomicBool>) {
					let mut frame = { context.$name.1.read().unwrap().clone() };

					loop {
						if receiver.recv().is_err() { mioco::shutdown(); };

						frame = Arc::new($frame_type::new(context.clone(), frame));

						{ *(context.$name.1.write().unwrap()) = frame.clone(); }

						ready_flag.store(true, Ordering::Relaxed);
					}
				}

				{
					let context    = context.clone();
					let receiver   = receivers.pop().unwrap();
					let ready_flag = ready_flags.pop().unwrap();
					mioco::spawn(move || { $fn_frame(context, receiver, ready_flag); });
				}
			)*}).unwrap();
		}
	};
}

register_contexts!(
	{ InputContext,   InputFrame,   input,   frame_input,   counter_input,   120 },
	{ PhysicsContext, PhysicsFrame, physics, frame_physics, counter_physics, 120 },
	{ RenderContext,  RenderFrame,  render,  frame_render,  counter_render,   60 }
);
