use std::process;
use std::sync::{Arc};
use std::path::{Path};
use std::collections::{HashMap};

use crossbeam::sync::{MsQueue};
use glium::{Surface, Program, DrawParameters, Depth, Frame};
use glium::glutin::{Event, VirtualKeyCode, ElementState};
use glium::backend::glutin_backend::{GlutinFacade};
use glium::draw_parameters::{DepthTest};

use model::{Model};
use render::render_command::{RenderCommand};
use scene::{Scene};
use input::{InputEvent};


// TODO: We can remove clear calls if we clear in the swapbuffers (and replace our old frame objec with a new one)
//
pub struct RenderProcessor {
	q:               Arc<MsQueue<RenderCommand>>,
	context:         GlutinFacade, // TODO: can we use a better type here
	frames:          HashMap<u64, Frame>,
	player:          Model,
	scene:           Scene,
	program:         Program,
	draw_parameters: DrawParameters<'static>,
}

impl RenderProcessor {
	pub fn new(q: Arc<MsQueue<RenderCommand>>, context: GlutinFacade) -> RenderProcessor {
		const PLAYER_PATH_STRING: &'static str = "./data/player.obj";
		let player = Model::new(&context, &Path::new(PLAYER_PATH_STRING));

		let scene = Scene::new(&context);

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

			void main() {
				float value = dot(v_normal, vec3(0.707, 0.707, 0.0));
				float intensity = max(0.0, value);
				color = vec4(intensity, intensity, intensity, 1.0);
//				color = value > 0.0 ? vec4(0.5, 0.25, 0.125, 1.0) : vec4(0.9, 0.45, 0.45, 1.0);
			}
		"#;
		// TODO: make sure constants are right

		let program = match Program::from_source(&context, vertex_source, fragment_source, None) {
			Ok(p) => p,
			Err(e) => {
				println!("Unable to compile shaders: {}", e);
				process::exit(-3);
			},
		};

		RenderProcessor {
			q: q,
			context: context,
			frames: HashMap::new(),
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

		let (width, height) = self.context.get_window().unwrap().get_inner_size().unwrap();
		let (width, height) = (width as i32, height as i32);

		for event in self.context.poll_events() {
			match event {
				Event::Closed => {
					println!("Exiting due to quit event");
					out.push(InputEvent::Quit);
					return None;
				},
				Event::KeyboardInput(_, _, Some(VirtualKeyCode::Escape)) => {
					println!("Exiting due to escape key");
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

					self.context.get_window().unwrap().set_cursor_position(cx, cy).ok();
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

		// macro required because we want to pass different models and use mut member variables
		//
		macro_rules! draw_model {
			($model:expr, $frame_counter:expr, $uniforms:expr) => {{
				// TODO: do something about this terrible syntax
				// and try to dump the macro
				//
				let mut frame = self.frames.get_mut(&$frame_counter).unwrap();

				let uniform_buffer = uniform! {
					model:                 $uniforms.model,
					model_view_projection: $uniforms.model_view_projection,
				};

				frame.draw(
					&$model.vertex_buffer,
					&$model.index_buffer,
					&self.program,
					&uniform_buffer,
					&self.draw_parameters).unwrap();
			}}
		}

		while let Some(job) = self.q.try_pop() {
			match job {
				RenderCommand::ClearScreen{ frame_counter } => {
					let mut frame = self.context.draw();
					frame.clear_color_and_depth((0.125f32, 0.25f32, 0.5f32, 1.0f32), 1.0);
					self.frames.insert(frame_counter, frame);
				},
				RenderCommand::SwapBuffers{ frame_counter } => {
					let mut frame = self.frames.remove(&frame_counter).unwrap();

					frame.set_finish().unwrap();
				},
				RenderCommand::DrawScene{ frame_counter, uniforms } => {
					draw_model!(&self.scene.model, frame_counter, uniforms);
				},
				RenderCommand::DrawPlayer{ frame_counter, uniforms } => {
					draw_model!(&self.player, frame_counter, uniforms);
				},
			}
		}
	}
}
