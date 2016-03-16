
pub enum RenderCommand {
	ClearScreen {
		frame_number: usize,
	},
	SwapBuffers,
	DrawTriangle,
}
