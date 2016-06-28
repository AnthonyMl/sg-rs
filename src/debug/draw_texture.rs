use glium::{Surface, VertexBuffer};
use glium::index::{NoIndices, PrimitiveType};
use glium::texture::{DepthTexture2d};

use render::render_processor::{RenderProcessor};


#[allow(dead_code)]
pub fn draw_texture<S: Surface>(processor: &RenderProcessor, surface: &mut S, texture: &DepthTexture2d) {
	#[derive(Copy, Clone)]
	struct ImageVertex {
		position: [f32; 2],
		texture_coordinates: [f32; 2],
	}
	implement_vertex!(ImageVertex, position, texture_coordinates);

	let vertices: Vec<ImageVertex> = vec![
		ImageVertex{ position: [-1f32,  1f32], texture_coordinates: [0f32, 1f32] },
		ImageVertex{ position: [ 1f32,  1f32], texture_coordinates: [1f32, 1f32] },
		ImageVertex{ position: [-1f32, -1f32], texture_coordinates: [0f32, 0f32] },
		ImageVertex{ position: [ 1f32, -1f32], texture_coordinates: [1f32, 0f32] }
	];

	let uniform_buffer = uniform! {
		texture_sampler: texture,
	};

	surface.draw(
		&VertexBuffer::new(&processor.facade, &vertices).unwrap(),
		&NoIndices(PrimitiveType::TriangleStrip),
		&processor.image_program.program,
		&uniform_buffer,
		&processor.image_program.parameters,
	).unwrap();
}
