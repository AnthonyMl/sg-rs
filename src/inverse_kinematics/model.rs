use glium::{VertexBuffer, IndexBuffer};
use glium::backend::{Facade};
use glium::index::{PrimitiveType};

use inverse_kinematics::{Axis};
use model::{Model};
use render::vertices::{ForwardVertex};


// TODO: instance these if it becomes a promlem
//
pub fn model<F: Facade>(facade: &F, lengths_and_axes: &[(f32, Axis)]) -> Model {
	const S: f32 = 0.4f32;
	const NUM_FACES: usize = 6;
	const NUM_VERTS_PER_FACE: usize = 4;
	const NUM_INDICES_PER_FACE: usize = 6;

	let num_joints = lengths_and_axes.iter().fold(0, |s, &(len, _)| if len != 0.0 { s + 1 }else{ s });

	let mut vertices = Vec::with_capacity(num_joints * NUM_FACES * NUM_VERTS_PER_FACE);
	let mut indices  = Vec::with_capacity(num_joints * NUM_FACES * NUM_INDICES_PER_FACE);

	for &(len, _) in lengths_and_axes {
		let len = len as f32;
		let base = vertices.len();

		vertices.push(ForwardVertex { position: [-S, 0f32,  S], normal: [ 0f32,  0f32,  1f32] });
		vertices.push(ForwardVertex { position: [ S, 0f32,  S], normal: [ 0f32,  0f32,  1f32] });
		vertices.push(ForwardVertex { position: [-S,  len,  S], normal: [ 0f32,  0f32,  1f32] });
		vertices.push(ForwardVertex { position: [ S,  len,  S], normal: [ 0f32,  0f32,  1f32] });

		vertices.push(ForwardVertex { position: [ S, 0f32,  S], normal: [ 1f32,  0f32,  0f32] });
		vertices.push(ForwardVertex { position: [ S, 0f32, -S], normal: [ 1f32,  0f32,  0f32] });
		vertices.push(ForwardVertex { position: [ S,  len,  S], normal: [ 1f32,  0f32,  0f32] });
		vertices.push(ForwardVertex { position: [ S,  len, -S], normal: [ 1f32,  0f32,  0f32] });

		vertices.push(ForwardVertex { position: [ S, 0f32, -S], normal: [ 0f32,  0f32, -1f32] });
		vertices.push(ForwardVertex { position: [-S, 0f32, -S], normal: [ 0f32,  0f32, -1f32] });
		vertices.push(ForwardVertex { position: [ S,  len, -S], normal: [ 0f32,  0f32, -1f32] });
		vertices.push(ForwardVertex { position: [-S,  len, -S], normal: [ 0f32,  0f32, -1f32] });

		vertices.push(ForwardVertex { position: [-S, 0f32, -S], normal: [-1f32,  0f32,  0f32] });
		vertices.push(ForwardVertex { position: [-S, 0f32,  S], normal: [-1f32,  0f32,  0f32] });
		vertices.push(ForwardVertex { position: [-S,  len, -S], normal: [-1f32,  0f32,  0f32] });
		vertices.push(ForwardVertex { position: [-S,  len,  S], normal: [-1f32,  0f32,  0f32] });

		vertices.push(ForwardVertex { position: [-S,  len,  S], normal: [ 0f32,  1f32,  0f32] });
		vertices.push(ForwardVertex { position: [ S,  len,  S], normal: [ 0f32,  1f32,  0f32] });
		vertices.push(ForwardVertex { position: [-S,  len, -S], normal: [ 0f32,  1f32,  0f32] });
		vertices.push(ForwardVertex { position: [ S,  len, -S], normal: [ 0f32,  1f32,  0f32] });

		vertices.push(ForwardVertex { position: [-S, 0f32, -S], normal: [ 0f32, -1f32,  0f32] });
		vertices.push(ForwardVertex { position: [ S, 0f32, -S], normal: [ 0f32, -1f32,  0f32] });
		vertices.push(ForwardVertex { position: [-S, 0f32,  S], normal: [ 0f32, -1f32,  0f32] });
		vertices.push(ForwardVertex { position: [ S, 0f32,  S], normal: [ 0f32, -1f32,  0f32] });

		for i in 0..NUM_INDICES_PER_FACE {
			let base = (base + i * NUM_VERTS_PER_FACE) as u32;
			indices.push(base + 0);
			indices.push(base + 1);
			indices.push(base + 2);
			indices.push(base + 2);
			indices.push(base + 1);
			indices.push(base + 3);
		}
	}

	Model {
		vertex_buffer: VertexBuffer::new(facade, &vertices).unwrap(),
		index_buffer:  IndexBuffer ::new(facade, PrimitiveType::TrianglesList, &indices).unwrap(),
	}
}
