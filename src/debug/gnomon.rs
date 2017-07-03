use glium::{IndexBuffer, VertexBuffer};
use glium::backend::{Facade};
use glium::index::{PrimitiveType};

use render::vertices::{UnlitVertex};
use render::casts_shadow::{CastsShadow, VertexBufferContainer};


// TODO: move these out of here
//
pub struct UnlitModel {
	pub vertex_buffer: VertexBuffer<UnlitVertex>,
	pub index_buffer: IndexBuffer<u32>,
}

impl CastsShadow for UnlitModel {
	fn buffers(&self) -> (VertexBufferContainer, &IndexBuffer<u32>) {
		(VertexBufferContainer::Unlit{ vertex_buffer: &self.vertex_buffer }, &self.index_buffer)
	}
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
