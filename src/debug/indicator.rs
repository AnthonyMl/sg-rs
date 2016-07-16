use glium::{IndexBuffer, VertexBuffer};
use glium::backend::{Facade};
use glium::index::{PrimitiveType};

use debug::gnomon::{UnlitModel};
use render::vertices::{UnlitVertex};


pub fn model<F: Facade>(facade: &F) -> UnlitModel {
	const S: f32 = 0.3;
	let color: [f32;3] = [0.2, 0.8, 0.2];
	let vertices = vec![
		UnlitVertex { position: [ 0.0,   -S,  0.0], color: color },
		UnlitVertex { position: [ 0.0,    S,  0.0], color: color },
		UnlitVertex { position: [   S,  0.0,  0.0], color: color },
		UnlitVertex { position: [  -S,  0.0,  0.0], color: color },
		UnlitVertex { position: [ 0.0,  0.0,    S], color: color },
		UnlitVertex { position: [ 0.0,  0.0,   -S], color: color },
	];
	let indices = vec![
		0, 2, 5,
		0, 5, 3,
		0, 3, 4,
		0, 4, 2,
		1, 5, 2,
		1, 3, 5,
		1, 4, 3,
		1, 2, 4
	];
	UnlitModel {
		vertex_buffer: VertexBuffer::new(facade, &vertices).unwrap(),
		index_buffer:  IndexBuffer ::new(facade, PrimitiveType::TrianglesList, &indices).unwrap(),
	}
}
