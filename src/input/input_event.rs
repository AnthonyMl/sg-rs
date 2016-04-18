use glium::glutin::{VirtualKeyCode};


pub enum InputEvent {
	Quit, // TODO: does anything use this?
	KeyboardInput {
		pressed: bool,
		id: VirtualKeyCode,
	},
}
