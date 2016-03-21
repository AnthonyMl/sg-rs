use std::sync::{Arc};

use crossbeam::sync::{MsQueue};
use glium;
use glium::{VertexBuffer, Surface};
use glium::index::{PrimitiveType};
use glium::backend::glutin_backend::{GlutinFacade};

use camera::{Camera};
use vertex3::{Vertex3};
use render::render_command::{RenderCommand};
use render::render_frame::{RenderFrame};
use uniform_wrappers::{UMatrix4};


// RP can internally track the frame_number by the number of swap_buffers calls
// We can also remove clear calls if we clear in the swapbuffers (and replace our old frame objec with a new one)
//
pub struct RenderProcessor {
	q: Arc<MsQueue<RenderCommand>>,
	context: GlutinFacade, // TODO: can we use a better type here
	frame: Option<RenderFrame>, // maybe some sort of multiproc command q in the future
	vertex_buffer: VertexBuffer<Vertex3>,
	camera: Camera,
	program: glium::Program,
}

impl RenderProcessor {
	pub fn new(q: Arc<MsQueue<RenderCommand>>, context: GlutinFacade, width: usize, height: usize) -> RenderProcessor {
		let triangle = vec![
			Vertex3{ position: [-0.5, -0.5 , 0.0] },
			Vertex3{ position: [ 0.0,  0.5 , 0.0] },
			Vertex3{ position: [ 0.5, -0.25, 0.0] },
		];
		let vertex_buffer = VertexBuffer::new(&context, &triangle).unwrap();

		let vertex_source = r#"
			#version 140

			in vec3 position;

			uniform mat4 mvp;

			void main() {
				gl_Position = mvp * vec4(position, 1.0);
			}
		"#;
		let fragment_source = r#"
			#version 140

			out vec4 color;

			void main() {
				color = vec4(0.5, 0.25, 0.125, 1.0);
			}
		"#;
		let program = glium::Program::from_source(&context, vertex_source, fragment_source, None).unwrap();

		RenderProcessor {
			q: q,
			context: context,
			frame: None,
			vertex_buffer: vertex_buffer,
			camera: Camera::new(width, height), // TODO: these shouldn't need to be passed around
			program: program,
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
				RenderCommand::ClearScreen{ frame_number } => {
					let mut frame = RenderFrame::new(frame_number, self.context.draw());

					frame.draw_context.clear_color(0.125f32, 0.25f32, 0.5f32, 1.0f32);
					frame.draw_context.clear_depth(1.0);

					self.frame = Some(frame);
				},
				RenderCommand::SwapBuffers => {
					match self.frame {
						Some(ref mut f) => {
							f.draw_context.set_finish().unwrap();
						},
						None => ()
					}
					self.frame = None;
				},
				RenderCommand::DrawTriangle => {
					match self.frame {
						Some(ref mut rf) => {
							let uniforms = uniform! {
								mvp: UMatrix4(self.camera.mtx_full),
							};

							rf.draw_context.draw(
								&self.vertex_buffer,
								&glium::index::NoIndices(PrimitiveType::TrianglesList),
								&self.program,
								&uniforms,
								&Default::default()).unwrap();
						},
						None => ()
					}
				},
			}
		}
	}
}
