use glium::{IndexBuffer, VertexBuffer};
use glium::backend::{Facade};
use glium::index::{PrimitiveType};

use render::vertices::{UnlitVertex};
use render::uniform_wrappers::{UMatrix4};


// TODO: move these out of here
//
pub struct UnlitModel {
	pub vertex_buffer: VertexBuffer<UnlitVertex>,
	pub index_buffer: IndexBuffer<u32>,
}

#[derive(Clone)]
pub struct UnlitUniforms {
	pub model_view_projection: UMatrix4,
}

pub fn model<F: Facade>(facade: &F) -> UnlitModel {
	let vertices = vec![
		UnlitVertex { position: [0f32, 0f32, 0f32], color: [1f32, 0f32, 0f32] },
		UnlitVertex { position: [1f32, 0f32, 0f32], color: [1f32, 0f32, 0f32] },
		UnlitVertex { position: [0f32, 0f32, 0f32], color: [0f32, 1f32, 0f32] },
		UnlitVertex { position: [0f32, 1f32, 0f32], color: [0f32, 1f32, 0f32] },
		UnlitVertex { position: [0f32, 0f32, 0f32], color: [0f32, 0f32, 1f32] },
		UnlitVertex { position: [0f32, 0f32, 1f32], color: [0f32, 0f32, 1f32] },
	];
	let indices = vec![0, 1, 2, 3, 4, 5];

	UnlitModel {
		vertex_buffer: VertexBuffer::new(facade, &vertices).unwrap(),
		index_buffer:  IndexBuffer ::new(facade, PrimitiveType::LinesList, &indices).unwrap(),
	}
}
