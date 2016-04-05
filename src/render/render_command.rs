use render::render_frame::{RenderFrame};


pub enum RenderCommand {
	ClearScreen { render_frame: RenderFrame },
	SwapBuffers { frame_counter: u64 },
	DrawScene   { frame_counter: u64 },
	DrawPlayer  { frame_counter: u64 },
}
