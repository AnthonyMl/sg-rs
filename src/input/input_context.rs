use std::sync::{Mutex};

use crossbeam::sync::{MsQueue};

use input::input_event::{InputEvent};
use input::input_frame::{InputFrame};
use input::input_map::{InputMap};


pub struct InputContext {
	pub input_q:   MsQueue<InputEvent>,
	pub output_q:  MsQueue<InputFrame>,
	pub input_map: InputMap,
	drain_lock:    Mutex<()>,
}

impl InputContext {
	pub fn new() -> InputContext {
		InputContext {
			input_q:    MsQueue::new(),
			output_q:   MsQueue::new(),
			input_map:  InputMap{},
			drain_lock: Mutex::new(()),
		}
	}

	// TODO: can this be done from two different threads (two tasks at the same time for some reason)
	//
	pub fn post_input_events<T: IntoIterator<Item=InputEvent>>(&self, input_events: T) {
		for event in input_events { self.input_q.push(event) }
	}

	pub fn get_input_frames(&self) -> Vec<InputFrame> {
		let mut out = Vec::new();
		let _ = self.drain_lock.lock().unwrap();
		while let Some(frame) = self.output_q.try_pop() { out.push(frame) }
		out
	}
}

unsafe impl Send for InputContext {}
unsafe impl Sync for InputContext {}
