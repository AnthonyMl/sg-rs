use std::sync::{Arc};

use physics::{PhysicsFrame};
use render::render_uniforms::{RenderUniforms};


pub struct RenderFrame {
	pub frame_counter: u64,
	pub physics_frame: Arc<PhysicsFrame>, // TODO: find a way to avoid all these arcs
	pub uniforms: RenderUniforms,
}
