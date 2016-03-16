use std::sync::{Arc};
use crossbeam::sync::{MsQueue};
use glium;
use glium::{DisplayBuild, Surface, Frame, VertexBuffer};
use glium::glutin::{WindowBuilder};
use glium::backend::glutin_backend::{GlutinFacade};
use glium::index::{PrimitiveType};
use glium::uniforms::{AsUniformValue, UniformValue};
use cgmath;

use camera::{Camera};


struct UMatrix4(cgmath::Matrix4<f32>);

impl AsUniformValue for UMatrix4 {
	fn as_uniform_value(&self) -> UniformValue {
		UniformValue::Mat4([
			[self.0.x.x, self.0.x.y, self.0.x.z, self.0.x.w],
			[self.0.y.x, self.0.y.y, self.0.y.z, self.0.y.w],
			[self.0.z.x, self.0.z.y, self.0.z.z, self.0.z.w],
			[self.0.w.x, self.0.w.y, self.0.w.z, self.0.w.w],
		])
	}
}

pub enum RenderCommand {
	ClearScreen {
		frame_number: usize,
	},
	SwapBuffers,
	DrawTriangle,
}

pub fn create() -> (RenderContext, RenderProcessor) {
	const WIDTH:  usize = 640;
	const HEIGHT: usize = 480;

	let context = WindowBuilder::new()
		.with_dimensions(WIDTH as u32, HEIGHT as u32)
		.with_title(format!("SG"))
		.build_glium().unwrap();

	let triangle = vec![
		Vertex{ position: [-0.5, -0.5 , 0.0] },
		Vertex{ position: [ 0.0,  0.5 , 0.0] },
		Vertex{ position: [ 0.5, -0.25, 0.0] },
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

	let q = Arc::new(MsQueue::new());
	(	RenderContext {
			q: q.clone()
 		},
		RenderProcessor {
			q: q.clone(),
			frame: None,
			camera: Camera::new(WIDTH, HEIGHT),
			vertex_buffer: vertex_buffer,
			program: program,
			context: context,
		}
	)
}

pub struct RenderContext {
	q: Arc<MsQueue<RenderCommand>>,
}
// RP can internally track the frame_number by the number of swap_buffers calls
// We can also remove clear calls if we clear in the swapbuffers (and replace our old frame objec with a new one)
//
pub struct RenderProcessor {
	q: Arc<MsQueue<RenderCommand>>,
	context: GlutinFacade, // TODO: can we use a better type here
	frame: Option<RenderFrame>, // maybe some sort of multiproc command q in the future
	vertex_buffer: VertexBuffer<Vertex>,
	camera: Camera,
	program: glium::Program,
}

impl RenderContext {
	pub fn swap_buffers(&self) {
		self.q.push(RenderCommand::SwapBuffers);
	}
	pub fn clear_screen(&self, frame_number: usize) {
		self.q.push(RenderCommand::ClearScreen{ frame_number: frame_number });
	}
	pub fn draw_garbage(&self) {
		self.q.push(RenderCommand::DrawTriangle);
	}

	pub fn tick(&self, frame_number: usize) {
		self.clear_screen(frame_number);

		self.draw_garbage();

		self.swap_buffers();
	}
}

impl RenderProcessor {
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

#[derive(Copy, Clone)]
struct Vertex {
	position: [f32; 3],
}
implement_vertex!(Vertex, position);

struct RenderFrame {
	_frame_number: usize,
	draw_context: Frame,
}
impl RenderFrame {
	pub fn new(frame_number: usize, draw_context: Frame) -> RenderFrame {
		RenderFrame {
			_frame_number: frame_number,
			draw_context: draw_context,
		}
	}
}
