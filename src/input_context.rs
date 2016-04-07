use std::sync::{Arc, Mutex, RwLock};

use crossbeam::sync::{MsQueue};
use cgmath::{Vector2};

use frame_counter::{FrameCounter};
use context::{Context, ContextType};
use input_event::{InputEvent};
use input_frame::{InputFrame};
use input_map::{InputMap};
use action_state::{ActionState};
use constants::{NANOSECONDS_PER_SECOND};
use keyboard_state::{KeyboardState};


const FREQUENCY: u64 = 120;

pub struct InputContext {
	frame_counter: FrameCounter,
	input_q: MsQueue<InputEvent>,
	output_q: MsQueue<InputFrame>,
	drain_lock: Mutex<()>,
	input_map: InputMap,
	frame: RwLock<Arc<InputFrame>>,
}

impl InputContext {
	pub fn new() -> InputContext {
		let frame_counter = FrameCounter::new(0);
		let frame_number = frame_counter.get();

		InputContext {
			frame_counter: frame_counter,
			input_q: MsQueue::new(),
			output_q: MsQueue::new(),
			drain_lock: Mutex::new(()),
			input_map: InputMap{},
			// TODO: this initialization is wrong because the input devices might be in a different state (buttons/sticks held)
			//
			frame: RwLock::new(Arc::new(InputFrame {
				frame_counter: frame_number,
				action_state: ActionState {
					movement_direction: Vector2::new(0f64, 0f64),
					view_direction:     Vector2::new(0f64, 0f64),
				},
				keyboard_state: KeyboardState {
					_left:     false,
					_right:    false,
					forward:   false,
					_backward: false,
				},
			})),
		}
	}

	pub fn post_input_events<T: IntoIterator<Item=InputEvent>>(&self, input_events: T) {
		for event in input_events { self.input_q.push(event) }
	}

	// TODO: change this to something more generic than Vec
	//
	pub fn get_input_frames(&self) -> Vec<InputFrame> {
		let mut out = Vec::new();
		{
			let _ = self.drain_lock.lock().unwrap();

			loop {
				match self.output_q.try_pop() {
					Some(frame) => out.push(frame),
					None => { break },
				}
			}
		}
		out
	}
}

impl Context for InputContext {
	fn rate(&self) -> u64 {
		NANOSECONDS_PER_SECOND / FREQUENCY
	}

	fn tick(&self, _contexts: Arc<ContextType>) {
		let frame_counter = self.frame_counter.increment();

		let last_frame = self.frame.read().unwrap().clone();
		let mut keyboard_state = last_frame.keyboard_state;

		loop {
			let event = match self.input_q.try_pop() {
				Some(e) => e,
				None    => break,
			};

			match event {
				InputEvent::KeyboardInput{ pressed, id } => {
					if id == self.input_map.forward() { keyboard_state.forward = pressed }
				},
				_ => {}
			}
		}

		let frame = InputFrame {
			frame_counter: frame_counter,
			action_state: ActionState {
				movement_direction: if keyboard_state.forward { Vector2::new(1f64, 0f64) } else { Vector2::new(0f64, 0f64) },
				view_direction: Vector2::new(0f64, 0f64),
			},
			keyboard_state: keyboard_state,
		};

		self.output_q.push(frame);
	}

	fn ready_to_tick(&self) -> bool { true }
}
