extern crate crossbeam;
extern crate glium;

use std::sync::{Arc};
use self::crossbeam::sync::{MsQueue};
use self::glium::{DisplayBuild, Surface};


pub enum RenderCommand {
	ClearScreen {
		frame_number: usize,
	},
	SwapBuffers,
	DrawTriangle,
}

pub fn create() -> (RenderContext, RenderProcessor) {
	let context = glium::glutin::WindowBuilder::new()
		.with_dimensions(640, 480)
		.with_title(format!("SG"))
		.build_glium().unwrap();

	let q = Arc::new(MsQueue::new());
	(	RenderContext {
			q: q.clone()
 		},
		RenderProcessor {
			q: q.clone(),
			context: context,
			frame: None,
		}
	)
}

pub struct RenderContext {
	q: Arc<MsQueue<RenderCommand>>,
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
}

pub struct RenderProcessor {
	q: Arc<MsQueue<RenderCommand>>,
	context: glium::backend::glutin_backend::GlutinFacade, // TODO: can we use a better type here
	frame: Option<RenderFrame>, // maybe some sort of multiproc command q in the future
}
impl RenderProcessor {
	// returns true to signal caller to exit program and event loop
	// TODO: should std::process::exit(i32) be used instead?
	//
	pub fn handle_system_events(&self) -> bool {
		use self::glium::glutin::{Event, VirtualKeyCode};

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
						Some(ref _f) => {
							// TODO: implement
						},
						None => ()
					}
				},
			}
		}
	}
}

struct RenderFrame {
	_frame_number: usize,
	draw_context: glium::Frame,
}
impl RenderFrame {
	pub fn new(frame_number: usize, draw_context: glium::Frame) -> RenderFrame {
		RenderFrame {
			_frame_number: frame_number,
			draw_context: draw_context,
		}
	}
}
