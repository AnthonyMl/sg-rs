use render::uniform_wrappers::{UMatrix4, UVector3};


#[derive(Clone)]
pub struct RenderUniforms {
	pub shadow:                  UMatrix4,
	pub model:                   UMatrix4,
	pub model_view_projection:   UMatrix4,
	pub reverse_light_direction: UVector3,
}
