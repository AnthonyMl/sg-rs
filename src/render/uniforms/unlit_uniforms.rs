use render::uniform_wrappers::{UMatrix4};
use render::uniforms::{ShadowUniforms};


#[derive(Clone)]
pub struct UnlitUniforms {
	pub shadow:                Option<UMatrix4>,
	pub model_view_projection: UMatrix4,
}

impl ShadowUniforms for UnlitUniforms {
	fn shadow_matrix(&self) -> &UMatrix4 { self.shadow.as_ref().unwrap() }
}
