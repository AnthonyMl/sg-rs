#[derive(Copy, Clone)]
pub struct ForwardVertex {
	pub position: [f32; 3],
	pub normal:   [f32; 3],
}

implement_vertex!(ForwardVertex, position, normal);
