use render::uniform_wrappers::{UMatrix4};


#[derive(Clone)]
pub struct RenderUniforms {
	pub model:                 UMatrix4,
	pub model_view_projection: UMatrix4,
}
