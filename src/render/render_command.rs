pub enum RenderCommand {
	ClearScreen {
		frame_counter: u64,
	},
	SwapBuffers,
	DrawTriangle,
}
