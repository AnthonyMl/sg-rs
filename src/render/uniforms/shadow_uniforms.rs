use render::uniform_wrappers::{UMatrix4};


pub trait ShadowUniforms {
	fn shadow_matrix(&self) -> &UMatrix4;
}
