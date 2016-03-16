use cgmath::{Matrix4};
use glium::uniforms::{AsUniformValue, UniformValue};


pub struct UMatrix4(pub Matrix4<f32>);

impl AsUniformValue for UMatrix4 {
	fn as_uniform_value(&self) -> UniformValue {
		UniformValue::Mat4([
			[self.0.x.x, self.0.x.y, self.0.x.z, self.0.x.w],
			[self.0.y.x, self.0.y.y, self.0.y.z, self.0.y.w],
			[self.0.z.x, self.0.z.y, self.0.z.z, self.0.z.w],
			[self.0.w.x, self.0.w.y, self.0.w.z, self.0.w.w],
		])
	}
}
