use std::sync::{Arc};

use physics::{PhysicsFrame};
use render::uniform_wrappers::{UMatrix4};
use render::render_uniforms::{RenderUniforms};


pub struct RenderFrame {
	pub frame_counter: u64,
	pub physics_frame: Arc<PhysicsFrame>, // TODO: find a way to avoid all these arcs
	pub uniforms: RenderUniforms,
}

impl RenderFrame {
	pub fn new(frame_counter: u64, physics_frame: Arc<PhysicsFrame>) -> RenderFrame {
		let mvp = UMatrix4(physics_frame.camera.mtx_full);

		RenderFrame {
			frame_counter: frame_counter,
			physics_frame: physics_frame,
			uniforms: RenderUniforms {
				mvp: mvp,
			},
		}
	}
}
