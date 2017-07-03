use glium::{IndexBuffer, VertexBuffer};

use render::vertices::{UnlitVertex};
use render::casts_shadow::{CastsShadow, VertexBufferContainer};


pub struct UnlitModel {
	pub vertex_buffer: VertexBuffer<UnlitVertex>,
	pub index_buffer: IndexBuffer<u32>,
}

impl CastsShadow for UnlitModel {
	fn buffers(&self) -> (VertexBufferContainer, &IndexBuffer<u32>) {
		(VertexBufferContainer::Unlit{ vertex_buffer: &self.vertex_buffer }, &self.index_buffer)
	}
}
