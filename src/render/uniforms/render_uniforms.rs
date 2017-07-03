use render::uniform_wrappers::{UMatrix4};
use render::uniforms::{ShadowUniforms};


#[derive(Clone)]
pub struct RenderUniforms {
	pub shadow:                UMatrix4,
	pub model:                 UMatrix4,
	pub model_view_projection: UMatrix4,
}

impl ShadowUniforms for RenderUniforms {
	fn shadow_matrix(&self) -> &UMatrix4 { &self.shadow }
}
