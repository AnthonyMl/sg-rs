use std::process;
use std::sync::{Arc};
use std::path::{Path};
use std::collections::{HashMap};

use crossbeam::sync::{MsQueue};
use glium::{Surface, Program, DrawParameters, Depth, Frame};
use glium::backend::glutin_backend::{GlutinFacade};
use glium::draw_parameters::{DepthTest};

use model::{Model};
use render::render_command::{RenderCommand};
use render::render_frame::{RenderFrame};
use scene::{Scene};


// TODO: We can remove clear calls if we clear in the swapbuffers (and replace our old frame objec with a new one)
//
pub struct RenderProcessor {
	q: Arc<MsQueue<RenderCommand>>,
	context: GlutinFacade, // TODO: can we use a better type here
	frames: HashMap<u64, (RenderFrame, Frame)>,
	player: Model,
	scene: Scene,
	program: Program,
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

			uniform mat4 mvp;

			void main() {
				v_normal = normalize((mvp * vec4(normal, 0.0)).xyz);
				gl_Position = mvp * vec4(position, 1.0);
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
	pub fn handle_system_events(&self) -> bool {
		use glium::glutin::{Event, VirtualKeyCode};

		for event in self.context.poll_events() {
			match event {
				Event::Closed => {
					println!("Exiting due to quit event");
					return true;
				},
				Event::KeyboardInput(_, _, Some(VirtualKeyCode::Escape)) => {
					println!("Exiting due to escape key");
					return true;
				},
				_ => ()
			}
		}
		false
	}

	pub fn handle_render_commands(&mut self) {
		loop {
			let job = self.q.try_pop();

			if job.is_none() { break }

			match job.unwrap() {
				RenderCommand::ClearScreen{ render_frame } => {
					let mut frame = self.context.draw();
					frame.clear_color_and_depth((0.125f32, 0.25f32, 0.5f32, 1.0f32), 1.0);
					self.frames.insert(render_frame.frame_counter, (render_frame, frame));
				},
				RenderCommand::SwapBuffers{ frame_counter } => {
					let (_, mut dc) = self.frames.remove(&frame_counter).unwrap();

					dc.set_finish().unwrap();
				},
				RenderCommand::DrawScene{ frame_counter } => {
					// TODO: do something about this repetition and terrible syntax
					//
					let mut a = self.frames.get_mut(&frame_counter).unwrap();
					let rf = &a.0;
					let mut dc = &mut a.1;

					let u = rf.uniforms.clone();
					let uniforms = uniform! {
						mvp: u.mvp,
					};

					dc.draw(
						&self.scene.model.vertex_buffer,
						&self.scene.model.index_buffer,
						&self.program,
						&uniforms,
						&self.draw_parameters).unwrap();
				},
				RenderCommand::DrawPlayer{ frame_counter } => {
					let mut a = self.frames.get_mut(&frame_counter).unwrap();
					let rf = &a.0;
					let mut dc = &mut a.1;

					let u = rf.uniforms.clone();
					let uniforms = uniform! {
						mvp: u.mvp,
					};

					dc.draw(
						&self.player.vertex_buffer,
						&self.player.index_buffer,
						&self.program,
						&uniforms,
						&self.draw_parameters).unwrap();
				},
			}
		}
	}
}
