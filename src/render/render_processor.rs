use std::sync::{Arc};
use std::path::{Path};

use cgmath::{Matrix4};
use crossbeam::sync::{MsQueue};
use glium::{Surface};
use glium::backend::glutin_backend::{GlutinFacade};
use glium::framebuffer::{SimpleFrameBuffer};
use glium::glutin::{Event, VirtualKeyCode, ElementState};
use glium::texture::{DepthFormat, DepthTexture2d, MipmapsOption, Texture2d};

use debug::gnomon;
use input::{InputEvent};
use inverse_kinematics::{Chain, Axis};
use model::{Model};
use render::shaders::{FlatColorProgram, ForwardProgram, ImageProgram, ShadowProgram};
use render::render_context::{DEPTH_DIMENSION};
use render::render_frame::{RenderFrame};
use render::uniform_wrappers::{UMatrix4};
use scene::{Scene};


pub struct RenderProcessor {
	pub facade: GlutinFacade,
	pub flat_color_program: FlatColorProgram,
	pub image_program: ImageProgram,

	q:               Arc<MsQueue<RenderFrame>>,
	player:          Model,
	scene:           Scene,
	forward_program: ForwardProgram,
	shadow_program:  ShadowProgram,
	shadow_texture:  DepthTexture2d,
	shadow_color:    Texture2d,
	ik_chain:        Chain,
}

impl RenderProcessor {
	pub fn new(q: Arc<MsQueue<RenderFrame>>, facade: GlutinFacade) -> RenderProcessor {
		const PLAYER_PATH_STRING: &'static str = "./data/player.obj";
		let player = Model::new(&facade, &Path::new(PLAYER_PATH_STRING));

		let scene = Scene::new(&facade);

		let flat_color_program = FlatColorProgram::new(&facade);
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

		let ik_chain = Chain::new(&facade, &[
			(0.0, Axis::Y),
			(3.0, Axis::Z),
			(3.0, Axis::Z),
			(3.0, Axis::Z)
		]);

		RenderProcessor {
			q: q,
			facade: facade,
			player: player,
			scene: scene,
			forward_program: forward_program,
			shadow_program: shadow_program,
			flat_color_program: flat_color_program,
			image_program: image_program,
			shadow_texture: shadow_texture,
			shadow_color: shadow_color,
			ik_chain: ik_chain,
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
						dx: ((x - cx) as f64) / (cx as f64),
						dy: ((y - cy) as f64) / (cy as f64),
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
				{
					let uniform_buffer = uniform! {
						// TODO: doesn't seem like the right place(thread) for these clones
						shadow: render_frame.scene_uniforms.shadow.clone(),
					};
					frame_buffer.draw(
						&self.scene.model.vertex_buffer,
						&self.scene.model.index_buffer,
						&self.shadow_program.program,
						&uniform_buffer,
						&self.shadow_program.parameters,
					).unwrap();
				}
				{
					let uniform_buffer = uniform! {
						shadow: render_frame.player_uniforms.shadow.clone(),
					};
					frame_buffer.draw(
						&self.player.vertex_buffer,
						&self.player.index_buffer,
						&self.shadow_program.program,
						&uniform_buffer,
						&self.shadow_program.parameters,
					).unwrap();
				}
				// TODO: do not do all this work here
				//
				{
					let transforms = self.ik_chain.joint_transforms();
					for joint in &transforms {
						let vp = render_frame.scene_uniforms.shadow.clone();

						let uniform_buffer = uniform! {
							shadow: UMatrix4(vp.0 * joint),
						};
						frame_buffer.draw(
							&self.ik_chain.model.vertex_buffer,
							&self.ik_chain.model.index_buffer,
							&self.shadow_program.program,
							&uniform_buffer,
							&self.shadow_program.parameters
						).unwrap();
					}
				}
			}

			let mut frame = self.facade.draw();
			frame.clear_color_and_depth((0.125f32, 0.25f32, 0.5f32, 1.0f32), 1.0);
			{
				let uniform_buffer = uniform! {
					shadow:                  render_frame.scene_uniforms.shadow.clone(),
					shadow_map:              self.shadow_texture.sampled(),
					model:                   render_frame.scene_uniforms.model.clone(),
					model_view_projection:   render_frame.scene_uniforms.model_view_projection.clone(),
					reverse_light_direction: render_frame.scene_uniforms.reverse_light_direction.clone(),
				};
				frame.draw(
					&self.scene.model.vertex_buffer,
					&self.scene.model.index_buffer,
					&self.forward_program.program,
					&uniform_buffer,
					&self.forward_program.parameters
				).unwrap();
			}
			{
				let uniform_buffer = uniform! {
					shadow:                  render_frame.player_uniforms.shadow.clone(),
					shadow_map:              self.shadow_texture.sampled(),
					model:                   render_frame.player_uniforms.model.clone(),
					model_view_projection:   render_frame.player_uniforms.model_view_projection.clone(),
					reverse_light_direction: render_frame.player_uniforms.reverse_light_direction.clone(),
				};
				frame.draw(
					&self.player.vertex_buffer,
					&self.player.index_buffer,
					&self.forward_program.program,
					&uniform_buffer,
					&self.forward_program.parameters
				).unwrap();
			}
			{
				// TODO: do not do all this work here
				//
				let transforms = self.ik_chain.joint_transforms();
				for joint in transforms {
					let vp = render_frame.scene_uniforms.model_view_projection.clone();
					let shadow = render_frame.scene_uniforms.shadow.clone();

					let uniform_buffer = uniform! {
						shadow:                  UMatrix4(shadow.0 * joint),
						shadow_map:              self.shadow_texture.sampled(),
						model:                   UMatrix4(joint),
						model_view_projection:   UMatrix4(vp.0 * joint),
						reverse_light_direction: render_frame.scene_uniforms.reverse_light_direction.clone(),
					};
					frame.draw(
						&self.ik_chain.model.vertex_buffer,
						&self.ik_chain.model.index_buffer,
						&self.forward_program.program,
						&uniform_buffer,
						&self.forward_program.parameters
					).unwrap();
				}
			}
			{
				let s = Matrix4::from_scale(3.0);
				// TODO: wrap the Matrix4 extraction in a function
				let matrix = render_frame.scene_uniforms.model_view_projection.clone();
				gnomon::draw(self, &mut frame, matrix.0 * s);
				let matrix = render_frame.player_uniforms.model_view_projection.clone();
				gnomon::draw(self, &mut frame, matrix.0 * s);
			}
			frame.set_finish().unwrap();
		}
	}
}
