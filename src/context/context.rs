use std::collections::{HashMap};
use std::thread;
use std::sync::{Arc};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use crossbeam::sync::{MsQueue};
use glium::glutin::{WindowBuilder, CursorState, get_primary_monitor};
use glium::{DisplayBuild};
use mioco;
use mioco::{Mioco, Config};
use mioco::sync::{Mutex, RwLock};
use mioco::sync::mpsc::{Receiver, Sender, channel};
use time;

use input::{InputContext, InputFrame};
use physics::{PhysicsContext, PhysicsFrame};
use render::{RenderContext, RenderFrame, RenderProcessor, RenderToken};
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
	const INPUT_FREQUENCY: u64 = 120;
	const RENDER_FREQUENCY: u64 = 60;

	let window_size = get_primary_monitor().get_dimensions();
	let window_size = (window_size.0/2, window_size.1/2); // FIXME: macbook scaling bs
	let aspect_ratio = (window_size.0 as f64) / (window_size.1 as f64);

	let glium_context = WindowBuilder::new()
		.with_dimensions(window_size.0, window_size.1)
		.with_title(format!("SG"))
		.with_depth_buffer(24)
		.with_decorations(false)
		.build_glium().unwrap();

	glium_context.get_window().unwrap().set_cursor_state(CursorState::Grab).ok();

	let q = Arc::new(MsQueue::new());

	let (render_tokens_sender, render_tokens_receiver) = channel::<RenderToken>();
	let context = Arc::new(
		Context {
			exit: AtomicBool::new(false),
			input_senders: MsQueue::new(),
			last_physics_frame: RwLock::new(Arc::new(PhysicsFrame::frame_zero(aspect_ratio))),
			render_tokens_length: AtomicUsize::new(0),
			physics_continuations: Arc::new(Mutex::new(HashMap::new())),

			input:   InputContext  ::new(),
			physics: PhysicsContext::new(),
			render:  RenderContext ::new(q.clone(), window_size),
		}
	);

	{
		let context = context.clone();
		thread::spawn(move || { spawn_coroutines(context, render_tokens_receiver); });
	}

	let mut render_processor = RenderProcessor::new(q, glium_context);
	let mut last_input_time = time::precise_time_ns() / 1_000_000;
	let mut last_render_time = last_input_time;

	// TODO: may need to be refactored to handle system events more frequently/(lower max latency)
	//
	while let Some(events) = render_processor.handle_system_events() {
		if context.exit.load(Ordering::Relaxed) { break }

		context.input.post_input_events(events);

		let time = time::precise_time_ns() / 1_000_000;

		while last_input_time < time {
			if let Some(sender) = context.input_senders.try_pop() {
				sender.send(()).unwrap();
				last_input_time += 1_000 / INPUT_FREQUENCY;
			} else {
				break;
			}
		}

		while last_render_time < time {
			const FIF_RENDER: usize = 1;
			let length = context.render_tokens_length.load(Ordering::Acquire);
			if length < FIF_RENDER {
				context.render_tokens_length.fetch_add(1, Ordering::Release);
				render_tokens_sender.send(render_processor.generate_token()).unwrap();
				last_render_time += 1_000 / RENDER_FREQUENCY;
			}
		}

		render_processor.handle_render_commands();

		thread::yield_now();
	}
}

struct Continuation {
	id: u64,
	req_count: AtomicUsize,
	reqs: MsQueue<Arc<Result>>,
}

enum Result { InputFrame(Arc<InputFrame>), PhysicsFrame(Arc<PhysicsFrame>) }

pub struct Context {
	exit: AtomicBool,
	input_senders: MsQueue<Sender<()>>,
	last_physics_frame: RwLock<Arc<PhysicsFrame>>,
	physics_continuations: Arc<Mutex<HashMap<u64, Arc<Continuation>>>>,
	render_tokens_length: AtomicUsize,

	pub input: InputContext,
	pub physics: PhysicsContext,
	pub render: RenderContext,
}

impl Context {
	pub fn input_signal(&self) -> Receiver<()> {
		let (sender, receiver) = channel();
		self.input_senders.push(sender);
		receiver
	}
}

fn input_entry(context: Arc<Context>, coroutine: Arc<Continuation>) {
	let start_signal = context.input_signal();
	start_signal.recv().unwrap();

	let last_input_frame = {
		match *coroutine.reqs.pop() {
			Result::InputFrame(ref data) => data.clone(),
			_ => unreachable!()
		}
	};

	let input_frame = InputFrame::new(context.clone(), last_input_frame);
	let result = Arc::new(Result::InputFrame(Arc::new(input_frame)));

	{
		let q = MsQueue::new();
		q.push(result.clone());
		let context = context.clone();
		let continuation = Arc::new(Continuation {
			id:        coroutine.id + 1,
			req_count: AtomicUsize::new(0),
			reqs:      q,
		});
		mioco::spawn(move|| input_entry(context, continuation));
	}
	{
		let continuation = {
			let mut map = context.physics_continuations.lock().unwrap();
			map.entry(coroutine.id).or_insert(Arc::new(Continuation {
				id:        coroutine.id,
				req_count: AtomicUsize::new(2),
				reqs:      MsQueue::new(),
			})).clone()
		};
		continuation.reqs.push(result.clone());
		let req_count = continuation.req_count.fetch_sub(1, Ordering::Relaxed);
		if req_count == 1 {
			let context = context.clone();
			let continuation = context.physics_continuations.lock().unwrap().remove(&coroutine.id).unwrap();

			mioco::spawn(move||physics_entry(context, continuation));
		}
	}
}

fn physics_entry(context: Arc<Context>, coroutine: Arc<Continuation>) {
	let (last_input_frame, last_physics_frame) = {
		let mut input_frame = Arc::new(InputFrame::frame_zero());
		let mut physics_frame = Arc::new(PhysicsFrame::frame_zero(context.render.aspect_ratio()));

		for _ in 0..2 {
			match *coroutine.reqs.pop() {
				Result::InputFrame(ref data) => input_frame = data.clone(),
				Result::PhysicsFrame(ref data) => physics_frame = data.clone(),
			};
		}
		(input_frame, physics_frame)
	};

	let physics_frame = Arc::new(PhysicsFrame::new(context.clone(), last_physics_frame, last_input_frame));
	let result = Arc::new(Result::PhysicsFrame(physics_frame.clone()));

	{ // TODO: do something better (that doesnt potentially block the sender)
		let latest_frame = context.last_physics_frame.read().unwrap().clone();
		if physics_frame.frame_counter > latest_frame.frame_counter {
			let mut reference = context.last_physics_frame.write().unwrap();
			*reference = physics_frame.clone();
		}
	}

	{
		let continuation = {
			let mut map = context.physics_continuations.lock().unwrap();
			map.entry(coroutine.id + 1).or_insert(Arc::new(Continuation {
				id:        coroutine.id + 1,
				req_count: AtomicUsize::new(2),
				reqs:      MsQueue::new(),
			})).clone()
		};
		continuation.reqs.push(result.clone());
		let req_count = continuation.req_count.fetch_sub(1, Ordering::Relaxed);
		if req_count == 1 {
		let context = context.clone();
		let continuation = context.physics_continuations.lock().unwrap().remove(&(coroutine.id + 1)).unwrap();
			mioco::spawn(move||physics_entry(context, continuation));
		}
	}
}

fn render_entry(context: Arc<Context>, render_tokens: Receiver<RenderToken>) {
	while !context.exit.load(Ordering::Relaxed) {
		render_tokens.recv().unwrap();

		let physics_frame = context.last_physics_frame.read().unwrap().clone();

		// TODO: don't render the same physics_frame twice

		let _render_frame = RenderFrame::new(context.clone(), physics_frame);

		context.render_tokens_length.fetch_sub(1, Ordering::Release);
	}
}

fn spawn_coroutines(context: Arc<Context>, render_tokens: Receiver<RenderToken>) {
	const NUM_THREADS: usize = 4;

	let mut config = Config::new();
	config.set_thread_num(NUM_THREADS);
	config.set_scheduler(Box::new(BalancingScheduler::new(NUM_THREADS)));

	{
		let result = Arc::new(Result::PhysicsFrame(Arc::new(PhysicsFrame::frame_zero(context.render.aspect_ratio()))));
		let q = MsQueue::new();
		q.push(result);
			context.physics_continuations.lock().unwrap().insert(1, Arc::new(Continuation {
			id: 1,
			req_count: AtomicUsize::new(1),
			reqs: q,
		}));
	}

	Mioco::new_configured(config).start(move|| {
		{
			let result = Arc::new(Result::InputFrame(Arc::new(InputFrame::frame_zero())));
			let q = MsQueue::new();
			q.push(result);
			let continuation = Arc::new(Continuation {
				id:        1,
				req_count: AtomicUsize::new(0),
				reqs:      q,
			});
			let context = context.clone();
			mioco::spawn(move||input_entry(context, continuation));
		}
		{ // for each render frame in flight
			let context = context.clone();
			mioco::spawn(move||render_entry(context, render_tokens));
		}
	}).unwrap();
}


