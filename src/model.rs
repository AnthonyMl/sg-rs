use std::path::{Path};

use glium::{VertexBuffer, IndexBuffer};
use glium::index::{PrimitiveType};
use glium::backend::glutin_backend::{GlutinFacade};
use tobj;
use cgmath::{Point3, Vector3, InnerSpace};

use render::{Vertex3};


pub struct Model {
	pub vertex_buffer: VertexBuffer<Vertex3>,
	pub index_buffer: IndexBuffer<u32>,
}

impl Model {
	pub fn new(context: &GlutinFacade, path: &Path) -> Model {
		let error_message = &format!("Unable to load Model({})", path.to_str().unwrap());

		let model: tobj::Model = tobj::load_obj(path).expect(error_message).0.pop().expect(error_message);

		let normals: Vec<[f32;3]> = if model.mesh.normals.is_empty() {
			let (face_normals, associated_tris) = Model::calculate_face_normals_and_associated_triangles(model.mesh.positions.as_slice(), model.mesh.indices.as_slice());

			(0..(model.mesh.positions.len()/3)).map(move |index| {
				let n = associated_tris[index].iter().fold(
					Vector3::new(0f32, 0f32, 0f32),
					|acc, &i| { acc + face_normals[i as usize] }
				).normalize();

				[n.x, n.y, n.z]
			}).into_iter().collect()
		} else {
			model.mesh.normals.chunks(3).map(|v| [v[0], v[1], v[2]]).collect()
		};

		let vertices: Vec<Vertex3> = model.mesh.positions.chunks(3).zip(normals).map(
			|(v, n)| Vertex3{ position: [v[0], v[1], v[2]], normal: n }
		).collect();

		Model {
			vertex_buffer: VertexBuffer::new(context, &vertices).unwrap(),
			index_buffer: IndexBuffer::new(context, PrimitiveType::TrianglesList, &model.mesh.indices).unwrap(),
		}
	}

	fn calculate_face_normals_and_associated_triangles(vertices: &[f32], indices: &[u32]) -> (Vec<Vector3<f32>>, Vec<Vec<u32>>) {
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
		(face_normals, associated_tris)
	}
}
