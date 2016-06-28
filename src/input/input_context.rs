use crossbeam::sync::{MsQueue};

use input::input_event::{InputEvent};
use input::input_map::{InputMap};


pub struct InputContext {
	pub input_q:   MsQueue<InputEvent>,
	pub input_map: InputMap,
}

impl InputContext {
	pub fn new() -> InputContext {
		InputContext {
			input_q:   MsQueue::new(),
			input_map: InputMap{},
		}
	}

	// TODO: can this be done from two different threads (two tasks at the same time for some reason)
	//
	pub fn post_input_events<T: IntoIterator<Item=InputEvent>>(&self, input_events: T) {
		for event in input_events { self.input_q.push(event) }
	}
}

unsafe impl Send for InputContext {}
unsafe impl Sync for InputContext {}
