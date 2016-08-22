use std::sync::{Arc};

use cgmath::{Vector2};
use mioco;

use input::keyboard_state::{KeyboardState};
use input::input_event::{InputEvent};
use context::{Context};


#[derive(Clone)]
pub struct InputFrame {
	pub frame_counter: u64,
	pub movement_delta: Vector2<f32>,
	pub view_angles_delta: Vector2<f32>,
	pub keyboard_state: KeyboardState,
}

impl InputFrame {
	pub fn frame_zero() -> InputFrame {
		InputFrame {
			frame_counter:     0,
			movement_delta:    Vector2::new(0.0, 0.0),
			view_angles_delta: Vector2::new(0.0, 0.0),
			keyboard_state:    Default::default(),
		}
	}

	pub fn new(context: Arc<Context>, frame: Arc<InputFrame>) -> InputFrame {
		let ic = &context.input;

		let mut keyboard_state = frame.keyboard_state.clone();
		let mut mouse_movement = Vector2::new(0f32, 0f32);

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
				},
				InputEvent::Quit => {
					mioco::shutdown();
				},
			}
		}

		const FORWARD: Vector2<f32> = Vector2{ x: 1f32, y: 0f32};
		const RIGHT:   Vector2<f32> = Vector2{ x: 0f32, y: 1f32};

		let mut direction = Vector2::new(0f32, 0f32);

		if keyboard_state.forward  { direction = direction + FORWARD }
		if keyboard_state.backward { direction = direction - FORWARD }
		if keyboard_state.right    { direction = direction + RIGHT   }
		if keyboard_state.left     { direction = direction - RIGHT   }

		InputFrame {
			frame_counter: frame.frame_counter + 1,
			movement_delta: direction,
			view_angles_delta: mouse_movement,
			keyboard_state: keyboard_state,
		}
	}
}
