use std::sync::{Arc};

use crossbeam::sync::{MsQueue};
use glium::{Surface};
use glium::backend::glutin_backend::{GlutinFacade};
use glium::framebuffer::{SimpleFrameBuffer};
use glium::glutin::{Event, VirtualKeyCode, ElementState};
use glium::texture::{DepthFormat, DepthTexture2d, MipmapsOption, Texture2d};

use input::{InputEvent};
use render::shaders::{UnlitProgram, ForwardProgram, ImageProgram, ShadowProgram};
use render::render_context::{DEPTH_DIMENSION};
use render::render_frame::{RenderFrame};
use render::casts_shadow::{VertexBufferContainer};


pub struct RenderProcessor {
	pub facade: GlutinFacade,
	pub unlit_program: UnlitProgram,
	pub image_program: ImageProgram,

	q:               Arc<MsQueue<RenderFrame>>,
	forward_program: ForwardProgram,
	shadow_program:  ShadowProgram,
	shadow_texture:  DepthTexture2d,
	shadow_color:    Texture2d,
}

impl RenderProcessor {
	pub fn new(q: Arc<MsQueue<RenderFrame>>, facade: GlutinFacade) -> RenderProcessor {
		let unlit_program = UnlitProgram::new(&facade);
		let forward_program = ForwardProgram::new(&facade);
		let image_program = ImageProgram::new(&facade);
		let shadow_program = ShadowProgram::new(&facade);

		let shadow_texture = DepthTexture2d::empty_with_format(
			&facade,
			DepthFormat::I24,
			MipmapsOption::NoMipmap,
			DEPTH_DIMENSION,
			DEPTH_DIMENSION
		).unwrap(); // TODO: handle error instead
		let shadow_color = Texture2d::empty(&facade, DEPTH_DIMENSION, DEPTH_DIMENSION).unwrap();

		RenderProcessor {
			q: q,
			facade: facade,
			forward_program: forward_program,
			shadow_program: shadow_program,
			unlit_program: unlit_program,
			image_program: image_program,
			shadow_texture: shadow_texture,
			shadow_color: shadow_color,
		}
	}

	// returns true to signal caller to exit program and event loop
	// TODO: should std::process::exit(i32) be used instead?
	//
	pub fn handle_system_events(&self) -> Option<Vec<InputEvent>> {
		let mut out = Vec::new();

		let (width, height) = self.facade.get_window().unwrap().get_inner_size().unwrap();
		let (width, height) = (width as i32, height as i32);

		for event in self.facade.poll_events() {
			match event {
				Event::Closed => {
					println!("Exiting due to quit event");
					out.push(InputEvent::Quit);
					return None;
				},
				Event::KeyboardInput(_, _, Some(VirtualKeyCode::Escape)) => {
					println!("Exiting due to escape key");
					out.push(InputEvent::Quit);
					return None;
				},
				Event::KeyboardInput(state, _, Some(key_code)) => {
					out.push(InputEvent::KeyboardInput {
						pressed: state == ElementState::Pressed,
						id: key_code,
					});
				},
				Event::MouseMoved(x, y) => {
					let (cx, cy) = (width / 2, height / 2);

					if x == cx && y == cy { continue }

					self.facade.get_window().unwrap().set_cursor_position(cx, cy).ok();
					out.push(InputEvent::MouseMoved {
						dx: ((x - cx) as f32) / (cx as f32),
						dy: ((y - cy) as f32) / (cy as f32),
					});
				},
				Event::Resized(_width, _height) => {
					// TODO: implement and event bus it or something
				},
				_ => ()
			}
		}
		Some(out)
	}

	pub fn handle_render_commands(&mut self) {
		while let Some(render_frame) = self.q.try_pop() {
			{
				let mut frame_buffer = SimpleFrameBuffer::with_depth_buffer(&self.facade, &self.shadow_color, &self.shadow_texture).unwrap();

				frame_buffer.clear_depth(1.0);
				for (shadow_caster, uniforms) in render_frame.shadow_casters {
					let uniform_buffer = uniform! {
						shadow: uniforms.shadow_matrix().clone(),
					};

					let (vbuffer, index_buffer) = shadow_caster.buffers();
					match vbuffer {
						// TODO: can these two cases be unified
						VertexBufferContainer::Forward{ vertex_buffer } => {
							frame_buffer.draw(
								vertex_buffer,
								index_buffer,
								&self.shadow_program.program,
								&uniform_buffer,
								&self.shadow_program.parameters
							).unwrap();
						},
						VertexBufferContainer::Unlit{ vertex_buffer } => {
							frame_buffer.draw(
								vertex_buffer,
								index_buffer,
								&self.shadow_program.program,
								&uniform_buffer,
								&self.shadow_program.parameters
							).unwrap();
						},
					};
				}
			}

			let mut frame = self.facade.draw();
			frame.clear_color_and_depth((0.125f32, 0.25f32, 0.5f32, 1.0f32), 1.0);
			{
				for &(ref model, ref uniforms) in &render_frame.models {
					let uniform_buffer = uniform! {
						shadow:                  uniforms.shadow.clone(),
						shadow_map:              self.shadow_texture.sampled(),
						model:                   uniforms.model.clone(),
						model_view_projection:   uniforms.model_view_projection.clone(),
						reverse_light_direction: render_frame.reverse_light_direction.clone(),
					};
					frame.draw(
						&model.vertex_buffer,
						&model.index_buffer,
						&self.forward_program.program,
						&uniform_buffer,
						&self.forward_program.parameters
					).unwrap();
				}
			}
			{
				for &(ref model, ref uniforms) in &render_frame.unlit_models {
					let uniform_buffer = uniform! {
						model_view_projection: uniforms.model_view_projection.clone()
					};

					frame.draw(
						&model.vertex_buffer,
						&model.index_buffer,
						&self.unlit_program.program,
						&uniform_buffer,
						&self.unlit_program.parameters
					).unwrap();
				}
			}
			frame.set_finish().unwrap();
		}
	}
}
