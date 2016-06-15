use cgmath::{Matrix4, Vector3, Vector4};
use glium::{Surface, VertexBuffer};
use glium::index::{NoIndices, PrimitiveType};
use glium::uniforms::{EmptyUniforms};

use render::vertices::{FlatColorVertex};
use render::render_processor::{RenderProcessor};


pub fn draw<S: Surface>(processor: &RenderProcessor, surface: &mut S, transform: Matrix4<f64>) {
	let vertices = {
		let o = transform * Vector4::unit_w();
		let o = Vector4::new(o.x as f32, o.y as f32, o.z as f32, o.w as f32);
		vec![
			Vector3::unit_x(),
			Vector3::unit_y(),
			Vector3::unit_z()
		].into_iter().flat_map(|c| {
			let v = transform * c.extend(1.0);
			let v = Vector4::new(v.x as f32, v.y as f32, v.z as f32, v.w as f32);
			let c = Vector3::new(c.x as f32, c.y as f32, c.z as f32);
			vec![ // TODO: can we remove the vec
				FlatColorVertex { position: [o.x, o.y, o.z, o.w], color: [c.x, c.y, c.z] },
				FlatColorVertex { position: [v.x, v.y, v.z, v.w], color: [c.x, c.y, c.z] }
			].into_iter()
		}).collect::<Vec<FlatColorVertex>>()
	};

	surface.draw(
		&VertexBuffer::new(&processor.facade, &vertices).unwrap(),
		&NoIndices(PrimitiveType::LinesList),
		&processor.flat_color_program.program,
		&EmptyUniforms,
		&processor.flat_color_program.parameters,
	).unwrap();
}
