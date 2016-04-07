use glium::glutin::{VirtualKeyCode};


pub struct InputMap {}

impl InputMap {
	pub fn forward(&self)  -> VirtualKeyCode { VirtualKeyCode::W }
	pub fn backward(&self) -> VirtualKeyCode { VirtualKeyCode::S }
	pub fn right(&self)    -> VirtualKeyCode { VirtualKeyCode::D }
	pub fn left(&self)     -> VirtualKeyCode { VirtualKeyCode::A }
}
