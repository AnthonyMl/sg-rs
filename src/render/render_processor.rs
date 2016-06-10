use std::process;
use std::sync::{Arc};
use std::path::{Path};

use crossbeam::sync::{MsQueue};
use glium::{Surface, Program, DrawParameters, Depth, BackfaceCullingMode};
use glium::backend::glutin_backend::{GlutinFacade};
use glium::draw_parameters::{DepthTest};
use glium::framebuffer::{SimpleFrameBuffer};
use glium::glutin::{Event, VirtualKeyCode, ElementState};
use glium::texture::{DepthFormat, DepthTexture2d, MipmapsOption, Texture2d};

use input::{InputEvent};
use model::{Model};
use render::render_frame::{RenderFrame};
use scene::{Scene};


const DEPTH_DIMENSION: u32 = 1024;


pub struct RenderProcessor {
	q:                 Arc<MsQueue<RenderFrame>>,
	facade:            GlutinFacade,
	player:            Model,
	scene:             Scene,
	program:           Program,
	draw_parameters:   DrawParameters<'static>,
	shadow_program:    Program,
	shadow_parameters: DrawParameters<'static>,
}

impl RenderProcessor {
	pub fn new(q: Arc<MsQueue<RenderFrame>>, facade: GlutinFacade) -> RenderProcessor {
		const PLAYER_PATH_STRING: &'static str = "./data/player.obj";
		let player = Model::new(&facade, &Path::new(PLAYER_PATH_STRING));

		let scene = Scene::new(&facade);

		let program = {
			let vertex_source = r#"
				#version 140

				in vec3 position;
				in vec3 normal;

				out vec3 v_normal;
				out vec4 v_shadow_pos;

				uniform mat4 shadow;
				uniform mat4 model;
				uniform mat4 model_view_projection;

				void main() {
					v_normal = normalize((model * vec4(normal, 0.0)).xyz);

					vec4 v4_position = vec4(position, 1.0);
					v_shadow_pos = shadow                * v4_position;
					gl_Position  = model_view_projection * v4_position;
				}
			"#;
			let fragment_source = r#"
				#version 140

				in vec3 v_normal;
				in vec4 v_shadow_pos;

				out vec4 color;

				uniform vec3 reverse_light_direction;

				uniform sampler2D shadow_map;

				void main() {
					vec3 shadow_pos = 0.5 + 0.5 * (v_shadow_pos.xyz / v_shadow_pos.w); // TODO: may not be necessary
					float closest_depth = texture(shadow_map, shadow_pos.xy).r;
					float shadow = (shadow_pos.z + 0.0005) > closest_depth ? 0.1 : 1.0;

					float value = dot(v_normal, reverse_light_direction);
					float intensity = shadow * max(0.1, 0.9 * value);
//					float intensity = shadow;
					color = vec4(intensity, intensity, intensity, 1.0);
				}
			"#;
			match Program::from_source(&facade, vertex_source, fragment_source, None) {
				Ok(p) => p,
				Err(e) => {
					println!("Unable to compile shaders:\n{}", e);
					process::exit(-3);
				},
			}
		};

		let shadow_program = {
			let vertex_source = r#"
				#version 140

				in vec3 position;

				uniform mat4 shadow;

				void main() {
					gl_Position = shadow * vec4(position, 1.0);
				}
			"#;
			let fragment_source = r#"
				#version 140
				void main() { }
			"#;
			match Program::from_source(&facade, vertex_source, fragment_source, None) {
				Ok(program) => program,
				Err(e) => {
					println!("Unable to compile shadow shader:\n{}", e);
					process::exit(-4);
				},
			}
		};

		RenderProcessor {
			q: q,
			facade: facade,
			player: player,
			scene: scene,
			program: program,
			draw_parameters: DrawParameters {
				depth: Depth {
					test: DepthTest::IfLess,
					write: true,
					.. Default::default()
				},
				.. Default::default()
			},
			shadow_program: shadow_program,
			shadow_parameters: DrawParameters {
				depth: Depth {
					test: DepthTest::IfLess,
					write: true,
					.. Default::default()
				},
				color_mask: (false, false, false, false),
				backface_culling: BackfaceCullingMode::CullCounterClockwise,
				.. Default::default()
			},
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
			let shadow_map = DepthTexture2d::empty_with_format(
				&self.facade,
				DepthFormat::I24,
				MipmapsOption::NoMipmap,
				DEPTH_DIMENSION,
				DEPTH_DIMENSION
			).unwrap(); // TODO: handle error instead

			{
				let color = Texture2d::empty(
					&self.facade,
					DEPTH_DIMENSION, // TODO: can we use 0/1
					DEPTH_DIMENSION,
				).unwrap();

				let mut frame_buffer = SimpleFrameBuffer::with_depth_buffer(&self.facade, &color, &shadow_map).unwrap();

				frame_buffer.clear_depth(1.0);

				{
					let uniform_buffer = uniform! {
						// TODO: doesn't seem like the right place(thread) for these clones
						shadow: render_frame.scene_uniforms.shadow.clone(),
					};
					frame_buffer.draw(
						&self.scene.model.vertex_buffer,
						&self.scene.model.index_buffer,
						&self.shadow_program,
						&uniform_buffer,
						&self.shadow_parameters,
					).unwrap();
				}
				{
					let uniform_buffer = uniform! {
						shadow: render_frame.player_uniforms.shadow.clone(),
					};
					frame_buffer.draw(
						&self.player.vertex_buffer,
						&self.player.index_buffer,
						&self.shadow_program,
						&uniform_buffer,
						&self.shadow_parameters,
					).unwrap();
				}
			}

			let mut frame = self.facade.draw();
			frame.clear_color_and_depth((0.125f32, 0.25f32, 0.5f32, 1.0f32), 1.0);
			{
				let uniform_buffer = uniform! {
					shadow:                  render_frame.scene_uniforms.shadow.clone(),
					shadow_map:              shadow_map.sampled(),
					model:                   render_frame.scene_uniforms.model,
					model_view_projection:   render_frame.scene_uniforms.model_view_projection,
					reverse_light_direction: render_frame.scene_uniforms.reverse_light_direction,
				};
				frame.draw(
					&self.scene.model.vertex_buffer,
					&self.scene.model.index_buffer,
					&self.program,
					&uniform_buffer,
					&self.draw_parameters
				).unwrap();
			}
			{
				let uniform_buffer = uniform! {
					shadow:                  render_frame.player_uniforms.shadow.clone(),
					shadow_map:              shadow_map.sampled(),
					model:                   render_frame.player_uniforms.model,
					model_view_projection:   render_frame.player_uniforms.model_view_projection,
					reverse_light_direction: render_frame.player_uniforms.reverse_light_direction,
				};
				frame.draw(
					&self.player.vertex_buffer,
					&self.player.index_buffer,
					&self.program,
					&uniform_buffer,
					&self.draw_parameters
				).unwrap();
			}
			frame.set_finish().unwrap();
		}
	}
}



