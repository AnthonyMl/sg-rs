use glium::{VertexBuffer, IndexBuffer};
use render::vertices::{ForwardVertex, UnlitVertex};


pub enum VertexBufferContainer<'a> {
	Forward { vertex_buffer: &'a VertexBuffer<ForwardVertex> },
	Unlit   { vertex_buffer: &'a VertexBuffer<UnlitVertex> },
}

pub trait CastsShadow { // <T> -> &VertexBuffer<T>
	fn buffers(&self) -> (VertexBufferContainer, &IndexBuffer<u32>);
}
