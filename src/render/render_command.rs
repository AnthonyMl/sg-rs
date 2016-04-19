use std::sync::{Arc};

use render::render_frame::{RenderFrame};
use render::render_uniforms::{RenderUniforms};


pub enum RenderCommand {
	ClearScreen { render_frame: Arc<RenderFrame> },
	SwapBuffers { frame_counter: u64 },
	DrawScene {
		frame_counter: u64,
		uniforms: RenderUniforms,
	},
	DrawPlayer {
		frame_counter: u64,
		uniforms: RenderUniforms,
	},
}
