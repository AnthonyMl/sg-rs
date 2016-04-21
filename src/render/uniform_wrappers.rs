use cgmath::{Matrix4};
use glium::uniforms::{AsUniformValue, UniformValue};


#[derive(Clone)]
pub struct UMatrix4(pub Matrix4<f64>);

impl AsUniformValue for UMatrix4 {
	fn as_uniform_value(&self) -> UniformValue {
		UniformValue::Mat4([
			[self.0.x.x as f32, self.0.x.y as f32, self.0.x.z as f32, self.0.x.w as f32],
			[self.0.y.x as f32, self.0.y.y as f32, self.0.y.z as f32, self.0.y.w as f32],
			[self.0.z.x as f32, self.0.z.y as f32, self.0.z.z as f32, self.0.z.w as f32],
			[self.0.w.x as f32, self.0.w.y as f32, self.0.w.z as f32, self.0.w.w as f32],
		])
	}
}
