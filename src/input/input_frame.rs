use std::sync::{Arc};

use cgmath::{Vector2};

use action_state::{ActionState};
use input::keyboard_state::{KeyboardState};
use input::input_event::{InputEvent};
use context::{ContextType};


#[derive(Clone, Default)]
pub struct InputFrame {
	pub frame_counter: u64,
	pub action_state: ActionState,
	pub keyboard_state: KeyboardState,
}

impl InputFrame {
	pub fn new(contexts: Arc<ContextType>, frame: Arc<InputFrame>) -> InputFrame {
		let mut keyboard_state = frame.keyboard_state;
		let mut mouse_movement = Vector2::new(0f64, 0f64);
		let ic = contexts.context_input();

		while let Some(event) = ic.input_q.try_pop() {
			match event {
				InputEvent::KeyboardInput{ pressed, id } => {
					if id == ic.input_map.forward()  { keyboard_state.forward  = pressed }
					if id == ic.input_map.backward() { keyboard_state.backward = pressed }
					if id == ic.input_map.right()    { keyboard_state.right    = pressed }
					if id == ic.input_map.left()     { keyboard_state.left     = pressed }
				},
				InputEvent::MouseMoved{ dx, dy } => {
					mouse_movement = mouse_movement + Vector2::new(dx, dy);
				}
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
			frame_counter: frame.frame_counter + 1,
			action_state: ActionState {
				movement_direction: direction,
				view_direction: mouse_movement,
			},
			keyboard_state: keyboard_state,
		};
		ic.output_q.push(frame.clone());
		frame
	}
}
