#[derive(Copy, Clone)]
pub struct UnlitVertex {
	pub position: [f32; 3],
	pub color:    [f32; 3],
}

implement_vertex!(UnlitVertex, position, color);
