use std::process;
use std::sync::{Arc};
use std::path::{Path};

use crossbeam::sync::{MsQueue};
use glium::{Surface, Program, DrawParameters, Depth};
use glium::backend::glutin_backend::{GlutinFacade};
use glium::draw_parameters::{DepthTest};
use glium::glutin::{Event, VirtualKeyCode, ElementState};

use input::{InputEvent};
use model::{Model};
use render::render_frame::{RenderFrame};
use scene::{Scene};


pub struct RenderProcessor {
	q:               Arc<MsQueue<RenderFrame>>,
	facade:          GlutinFacade,
	player:          Model,
	scene:           Scene,
	program:         Program,
	draw_parameters: DrawParameters<'static>,
}

impl RenderProcessor {
	pub fn new(q: Arc<MsQueue<RenderFrame>>, facade: GlutinFacade) -> RenderProcessor {
		const PLAYER_PATH_STRING: &'static str = "./data/player.obj";
		let player = Model::new(&facade, &Path::new(PLAYER_PATH_STRING));

		let scene = Scene::new(&facade);

		let vertex_source = r#"
			#version 140

			in vec3 position;
			in vec3 normal;

			out vec3 v_normal;

			uniform mat4 model;
			uniform mat4 model_view_projection;

			void main() {
				v_normal = normalize((model * vec4(normal, 0.0)).xyz);
				gl_Position = model_view_projection * vec4(position, 1.0);
			}
		"#;
		let fragment_source = r#"
			#version 140

			in vec3 v_normal;

			out vec4 color;

			uniform vec3 reverse_light_direction;

			void main() {
				float value = dot(v_normal, reverse_light_direction);
				float intensity = max(0.1, 0.9 * value);
				color = vec4(intensity, intensity, intensity, 1.0);
			}
		"#;

		let program = match Program::from_source(&facade, vertex_source, fragment_source, None) {
			Ok(p) => p,
			Err(e) => {
				println!("Unable to compile shaders: {}", e);
				process::exit(-3);
			},
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
			let mut frame = self.facade.draw();

			frame.clear_color_and_depth((0.125f32, 0.25f32, 0.5f32, 1.0f32), 1.0);

			{
				let uniform_buffer = uniform! {
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



