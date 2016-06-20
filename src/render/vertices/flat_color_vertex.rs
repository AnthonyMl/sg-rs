#[derive(Copy, Clone)]
pub struct FlatColorVertex {
	pub position: [f32; 4],
	pub color:    [f32; 3],
}

implement_vertex!(FlatColorVertex, position, color);
