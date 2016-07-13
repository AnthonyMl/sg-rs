use cgmath::{Vector2};
use std::default::{Default};


#[derive(Clone)]
pub struct ActionState {
	pub movement_direction: Vector2<f32>,
	pub view_direction:     Vector2<f32>,
}

impl Default for ActionState {
	fn default() -> ActionState {
		ActionState {
			movement_direction: Vector2{ x: 0f32, y: 0f32 },
			view_direction:     Vector2{ x: 0f32, y: 0f32 },
		}
	}
}
