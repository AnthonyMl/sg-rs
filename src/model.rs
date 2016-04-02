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
			model.mesh.normals.chunks(3).map(|v| [v[0], v[1], v[2]]).collect()
		};

		// TODO: can we do a Vec::from_raw_parts or a transmute here
		//
		let mut vertices: Vec<Vertex3> = Vec::with_capacity(model.mesh.positions.len()/3);
		let iterator = model.mesh.positions.chunks(3).zip(normals);

		for (v, n) in iterator {
			vertices.push(Vertex3 {
				position: [v[0], v[1], v[2]],
				normal: n,
			});
		}

		let indices = model.mesh.indices;

		Model {
			vertex_buffer: VertexBuffer::new(context, &vertices).unwrap(),
			index_buffer: IndexBuffer::new(context, PrimitiveType::TrianglesList, &indices).unwrap(),
		}
	}

	fn calculate_normals(vertices: &[f32], indices: &[u32]) -> Vec<[f32;3]> {
		let mut face_normals: Vec<Vector3<f32>> = Vec::with_capacity(indices.len()/3);
		let mut associated_tris: Vec<Vec<u32>> = (0..(vertices.len()/3)).map(|_| Vec::new()).collect();

		for (i, tri) in indices.chunks(3).enumerate() {
			for vertex in tri {
				associated_tris[*vertex as usize].push(i as u32);
				// model.mesh.positions.chunks(3)[vertex] is used by model.mesh.indices.chunks(3)[i]
			}

			let points: Vec<Point3<f32>> = tri.iter().map(|&idx| {
				let i = idx as usize;
				let v = &vertices[(i*3)..((i+1)*3)];
				Point3::new(v[0], v[1], v[2])
			}).collect();
			let ba = points[1] - points[0];
			let ca = points[2] - points[0];

			face_normals.push(ba.cross(ca).normalize());
		}

		let mut normals = Vec::with_capacity(vertices.len());

		for index in 0..(vertices.len()/3) {
			let normal = associated_tris[index].iter().fold(
				Vector3::new(0f32, 0f32, 0f32), |acc, &i| {
				acc + face_normals[i as usize]
			});

			let normal = normal.normalize();

			normals.push([normal.x, normal.y, normal.z]);
		}
		normals
	}
}
