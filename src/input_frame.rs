use action_state::{ActionState};
use keyboard_state::{KeyboardState};


pub struct InputFrame {
	pub frame_counter: u64,
	pub action_state: ActionState,
	pub keyboard_state: KeyboardState,
}
