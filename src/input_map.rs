use glium::glutin::{VirtualKeyCode};


pub struct InputMap {}

impl InputMap {
	pub fn forward(&self) -> VirtualKeyCode {
		VirtualKeyCode::W
	}
}
