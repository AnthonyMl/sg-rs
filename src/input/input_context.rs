use std::sync::{Arc, Mutex};

use crossbeam::sync::{MsQueue};

use frame_counter::{FrameCounter};
use context::{Context, ContextType, ContextState};
use input::input_event::{InputEvent};
use input::input_frame::{InputFrame};
use input::input_map::{InputMap};
use frame::{Frame};


pub struct InputContext {
	pub input_q: MsQueue<InputEvent>,
	pub input_map: InputMap,

	state: ContextState,
	output_q: MsQueue<InputFrame>,
	drain_lock: Mutex<()>,
}

impl InputContext {
	pub fn new() -> InputContext {
		let frame_counter = FrameCounter::new(0);
		let frame_number = frame_counter.get();

		InputContext {
			state: ContextState::new(Frame::Input(Arc::new(InputFrame {
				frame_counter: frame_number,
				..Default::default()
			}))),
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

	pub fn get_input_frames(&self) -> Vec<InputFrame> {
		let mut out = Vec::new();
		let _ = self.drain_lock.lock().unwrap();
		while let Some(frame) = self.output_q.try_pop() { out.push(frame) }
		out
	}

	fn get_frame(&self) -> Arc<InputFrame> {
		(match self.state().frame() {
			Frame::Input(f) => Some(f),
			_ => None,
		}).unwrap()
	}
}

unsafe impl Send for InputContext {}
unsafe impl Sync for InputContext {}

impl Context for InputContext {
	fn frequency(&self) -> u64 { 120 }

	fn tick(&self, contexts: Arc<ContextType>) -> Frame {
		let frame = self.get_frame();

		let new_frame = InputFrame::new(contexts, frame);
		self.output_q.push(new_frame.clone());
		Frame::Input(Arc::new(new_frame))
	}

	fn state(&self) -> &ContextState { &self.state }
}
