use std::fmt::{Display, Formatter, Result};

use cgmath::{Matrix4, Vector3};
use glium::uniforms::{AsUniformValue, UniformValue};


#[derive(Clone)]
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

impl Display for UMatrix4 {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "{} {} {} {}\n{} {} {} {}\n{} {} {} {}\n{} {} {} {}",
			self.0.x.x, self.0.x.y, self.0.x.z, self.0.x.w,
			self.0.y.x, self.0.y.y, self.0.y.z, self.0.y.w,
			self.0.z.x, self.0.z.y, self.0.z.z, self.0.z.w,
			self.0.w.x, self.0.w.y, self.0.w.z, self.0.w.w,
		)
	}
}

#[derive(Clone)]
pub struct UVector3(pub Vector3<f32>);

impl AsUniformValue for UVector3 {
	fn as_uniform_value(&self) -> UniformValue {
		UniformValue::Vec3([self.0.x, self.0.y, self.0.z])
	}
}
