#[derive(Copy, Clone)]
pub struct Vertex3 {
	pub position: [f32; 3],
	pub normal: [f32; 3],
}

implement_vertex!(Vertex3, position, normal);
