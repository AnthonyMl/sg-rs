use render::uniform_wrappers::{UMatrix4};


#[derive(Clone)]
pub struct RenderUniforms {
	pub mvp: UMatrix4,
}
