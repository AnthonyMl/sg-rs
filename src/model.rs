use glium;
use glium::{VertexBuffer};
use glium::index::{PrimitiveType, NoIndices};
use glium::backend::glutin_backend::{GlutinFacade};

use vertex3::{Vertex3};


pub struct Model {
	pub vertex_buffer: VertexBuffer<Vertex3>,
	pub index_buffer: NoIndices,
}

impl Model {
	pub fn new(context: &GlutinFacade) -> Model {
		let triangle = vec![
			Vertex3{ position: [-0.5, -0.5 , 0.0] },
			Vertex3{ position: [ 0.0,  0.5 , 0.0] },
			Vertex3{ position: [ 0.5, -0.25, 0.0] },
		];

		Model {
			vertex_buffer: VertexBuffer::new(context, &triangle).unwrap(),
			index_buffer: glium::index::NoIndices(PrimitiveType::TrianglesList),
		}
	}
}
