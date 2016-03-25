use std::path::{Path};

use glium::{VertexBuffer, IndexBuffer};
use glium::index::{PrimitiveType};
use glium::backend::glutin_backend::{GlutinFacade};
use tobj;
use cgmath::{Point3, Vector3, EuclideanVector};

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

		let normals = if model.mesh.normals.is_empty() {
			Model::calculate_normals(&model.mesh.positions, &model.mesh.indices)
		} else {
			model.mesh.normals
		};

		// TODO: can we do a Vec::from_raw_parts or a transmute here
		//
		let mut vertices: Vec<Vertex3> = Vec::with_capacity(model.mesh.positions.len()/3);
		let iterator = model.mesh.positions.chunks(3).zip(normals.chunks(3));

		for (v, n) in iterator {
			vertices.push(Vertex3 {
				position: [v[0], v[1], v[2]],
				normal: [n[0], n[1], n[2]],
			});
		}

		let indices = model.mesh.indices;

		Model {
			vertex_buffer: VertexBuffer::new(context, &vertices).unwrap(),
			index_buffer: IndexBuffer::new(context, PrimitiveType::TrianglesList, &indices).unwrap(),
		}
	}

	fn calculate_normals(vertices: &[f32], indices: &[u32]) -> Vec<f32> {
		let mut associated_tris: Vec<Vec<u32>> = Vec::with_capacity(vertices.len()/3);
		for _ in vertices { associated_tris.push(Vec::new()) } // TODO: look up if this can be avoided
		for (i, triangle) in indices.chunks(3).enumerate() {
			for vertex in triangle {
				associated_tris[*vertex as usize].push(i as u32);
			}
		}

		let mut normals = Vec::with_capacity(vertices.len());

		for index in 0..(vertices.len()/3) {
			let mut normal = Vector3::new(0f32, 0f32, 0f32);

			for tri_index in &associated_tris[index] {
				let tri =
					&indices[
						((tri_index   *3) as usize)..
						(((tri_index+1)*3) as usize)];

				let points: Vec<Point3<f32>> = tri.into_iter().map(|&p| {
					let idx = p as usize;
					let v = &vertices[(idx*3)..((idx+1)*3)];
					Point3::new(v[0], v[1], v[2])
				}).collect();

				let (a, b, c) = (points[0], points[1], points[2]);

				let ba = b - a;
				let ca = c - a;

				let addition = ca.cross(ba); // TODO: do we want to normalize here

				normal = normal + addition;
			}
			normal = normal.normalize();

			normals.push(normal.x);
			normals.push(normal.y);
			normals.push(normal.z);
		}
		normals
	}
}
