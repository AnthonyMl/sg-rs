use cgmath::{Vector2};
use std::default::{Default};


#[derive(Clone)]
pub struct ActionState {
	pub movement_direction: Vector2<f64>,
	pub view_direction:     Vector2<f64>,
}

impl Default for ActionState {
	fn default() -> ActionState {
		ActionState {
			movement_direction: Vector2{ x: 0f64, y: 0f64 },
			view_direction:     Vector2{ x: 0f64, y: 0f64 },
		}
	}
}
