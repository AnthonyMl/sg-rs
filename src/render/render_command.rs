use std::sync::{Arc};

use physics::{PhysicsFrame};


pub enum RenderCommand {
	ClearScreen {
		frame_counter: u64,
		physics_frame: Arc<PhysicsFrame>,
	},
	SwapBuffers,
	DrawTriangle,
}
