use std::sync::{Arc};

use cgmath::{Matrix4, Point};

use render::render_uniforms::{RenderUniforms};
use render::uniform_wrappers::{UMatrix4};
use context::{ContextType};


pub struct RenderFrame { }

impl RenderFrame {
	pub fn new(context: Arc<ContextType>, _last_frame: Arc<RenderFrame>) -> RenderFrame {
		let physics_frame = context.physics().get_frame();

		// TODO: do something about passing the frame counter on every
		// a guard/builder pattern that sends a command queue to base on drop or something
		//

		let rc = context.render();

		let frame_counter = rc.frame_counter();

		rc.clear_screen(frame_counter);

		let view_projection = physics_frame.camera.mtx_full.clone();

		rc.draw_scene(frame_counter, RenderUniforms {
			mvp: UMatrix4(view_projection),
		});

		let translation = physics_frame.player_position.to_vec();

		let mvp = view_projection * Matrix4::from_translation(translation);

		rc.draw_player(frame_counter, RenderUniforms {
			mvp: UMatrix4(mvp),
		});

		rc.swap_buffers(frame_counter);

		RenderFrame{ }
	}
}
