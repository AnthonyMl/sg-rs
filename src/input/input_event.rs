use glium::glutin::{VirtualKeyCode};


pub enum InputEvent {
	Quit,
	KeyboardInput {
		pressed: bool,
		id: VirtualKeyCode,
	},
	MouseMoved {
		dx: f64,
		dy: f64,
	},
}
