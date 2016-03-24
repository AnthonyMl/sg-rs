use std::path::{Path};

use glium::{VertexBuffer, IndexBuffer};
use glium::index::{PrimitiveType};
use glium::backend::glutin_backend::{GlutinFacade};
use tobj;

use vertex3::{Vertex3};


pub struct Model {
	pub vertex_buffer: VertexBuffer<Vertex3>,
	pub index_buffer: IndexBuffer<u32>,
}

impl Model {
	pub fn new(context: &GlutinFacade) -> Model {
		const MODEL_PATH_STRING: &'static str = "../data/buddha.obj";
		let path = Path::new(MODEL_PATH_STRING);

		let model = match tobj::load_obj(path) {
			Ok((ref mut models, _)) => {
				match models.pop() {
					Some(m) => m,
					None => { panic!("Unable to load Model({})", MODEL_PATH_STRING) },
				}
			},
			Err(_) => { panic!("Unable to load Model({})", MODEL_PATH_STRING) },
		};

		// TODO: can we do a Vec::from_raw_parts or a transmute here
		//
		let mut vertices: Vec<Vertex3> = Vec::with_capacity(model.mesh.positions.len());
		for v in model.mesh.positions.chunks(3) {
			vertices.push(Vertex3 {
				position: [v[0], v[1], v[2]],
			});
		}

		let indices = model.mesh.indices;

		Model {
			vertex_buffer: VertexBuffer::new(context, &vertices).unwrap(),
			index_buffer: IndexBuffer::new(context, PrimitiveType::TrianglesList, &indices).unwrap(),
		}
	}
}
