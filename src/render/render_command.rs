use render::render_uniforms::{RenderUniforms};


pub enum RenderCommand {
	ClearScreen { frame_counter: u64 },
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
