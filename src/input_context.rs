use std::sync::{Arc, Mutex};

use crossbeam::sync::{MsQueue};
use cgmath::{Vector2};

use frame_counter::{FrameCounter};
use context::{Context, ContextType};
use input_event::{InputEvent};
use input_frame::{InputFrame};
use input_map::{InputMap};
use action_state::{ActionState};
use context_state::{ContextState, ContextStateProxy};


pub struct InputContext {
	state: ContextState<InputFrame>,
	input_q: MsQueue<InputEvent>,
	output_q: MsQueue<InputFrame>,
	drain_lock: Mutex<()>,
	input_map: InputMap,
}

impl InputContext {
	pub fn new() -> InputContext {
		let frame_counter = FrameCounter::new(0);
		let frame_number = frame_counter.get();

		InputContext {
			state: ContextState::new(InputFrame {
				frame_counter: frame_number,
				..Default::default()
			}),
			input_q: MsQueue::new(),
			output_q: MsQueue::new(),
			drain_lock: Mutex::new(()),
			input_map: InputMap{},
		}
	}

	// TODO: can this be done from two different threads (two tasks at the same time for some reason)
	//
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
					None => break,
				}
			}
		}
		out
	}
}

impl Context for InputContext {
	fn frequency(&self) -> u64 { 120 }

	fn tick(&self, _contexts: Arc<ContextType>) {
		let last_frame = self.state.frame();
		let mut keyboard_state = last_frame.keyboard_state;

		loop {
			let event = match self.input_q.try_pop() {
				Some(e) => e,
				None    => break,
			};

			match event {
				InputEvent::KeyboardInput{ pressed, id } => {
					if id == self.input_map.forward()  { keyboard_state.forward  = pressed }
					if id == self.input_map.backward() { keyboard_state.backward = pressed }
					if id == self.input_map.right()    { keyboard_state.right    = pressed }
					if id == self.input_map.left()     { keyboard_state.left     = pressed }
				},
				_ => {}
			}
		}

		const FORWARD: Vector2<f64> = Vector2{ x: 1f64, y: 0f64};
		const RIGHT:   Vector2<f64> = Vector2{ x: 0f64, y: 1f64};

		let mut direction = Vector2::new(0f64, 0f64);

		if keyboard_state.forward  { direction = direction + FORWARD }
		if keyboard_state.backward { direction = direction - FORWARD }
		if keyboard_state.right    { direction = direction + RIGHT   }
		if keyboard_state.left     { direction = direction - RIGHT   }

		let frame = InputFrame {
			frame_counter: last_frame.frame_counter + 1,
			action_state: ActionState {
				movement_direction: direction,
				view_direction: Vector2::new(0f64, 0f64),
			},
			keyboard_state: keyboard_state,
		};

		self.state.set_frame(Arc::new(frame.clone()));
		self.output_q.push(frame);
	}

	fn state(&self) -> &ContextStateProxy { &self.state }
}
