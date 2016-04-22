use std::sync::{Arc};

use crossbeam::sync::{MsQueue};
use glium::glutin::{WindowBuilder, CursorState, get_primary_monitor};
use glium::{DisplayBuild};

use render::{RenderContext, RenderProcessor, RenderCommand, RenderFrame};
use input::{InputContext, InputFrame};
use physics::{PhysicsContext, PhysicsFrame};
use context::context_state::{ContextState};


// TODO: try to remove Arc dependency
// TODO: maybe rename ContextType->Context and Context->IsContext or something like that?
//

impl ContextType {
	fn new(q: Arc<MsQueue<RenderCommand>>, window_size: (u32, u32)) -> ContextType {
		let ai = Arc::new(InputContext::new());
		let ap = Arc::new(PhysicsContext::new(window_size));
		let ar = Arc::new(RenderContext::new(q, window_size));

		ContextType {
			input:   ai.clone(),
			physics: ap.clone(),
			render:  ar.clone(),
		}
	}
}
unsafe impl Send for ContextType {}
unsafe impl Sync for ContextType {}

pub trait Context: Send + Sync {
	fn frequency(&self) -> u64;     // TODO: can this be static?
	fn state(&self) -> &ContextState;
	fn tick(&self, Arc<ContextType>) -> Frame;

	fn do_tick(&self, contexts: Arc<ContextType>) {
		self.state().tick_enter();
		let f = self.tick(contexts);
		self.state().tick_exit(f);
	}
}

pub fn create() -> (Arc<ContextType>, RenderProcessor) {
	let window_size = get_primary_monitor().get_dimensions();
	let window_size = (window_size.0/2, window_size.1/2);

	let glium_context = WindowBuilder::new()
		.with_dimensions(window_size.0, window_size.1)
		.with_title(format!("SG"))
		.with_depth_buffer(24)
		.with_decorations(false)
		.build_glium().unwrap();

	glium_context.get_window().unwrap().set_cursor_state(CursorState::Grab).ok();

	let q = Arc::new(MsQueue::new());

	(
		Arc::new(ContextType::new(q.clone(), window_size)),
		RenderProcessor::new(q, glium_context),
	)
}

pub trait IsFrame: Sized {
	fn to_frame(self) -> Frame;
}

macro_rules! mega_context {
	( $({ $context_type:ty, $frame_type:ident, $erased_frame_type:ident, $name:ident, $frequency:expr }),* ) => {
		enum ContextKind {
			$( $erased_frame_type(Arc<$context_type>), )*
		}

		fn to_context(context: &ContextKind) -> Arc<Context> {
			match *context {
				$( ContextKind::$erased_frame_type(ref $name) => $name.clone(), )*
			}
		}

		#[derive(Clone)]
		pub enum Frame {
			$( $erased_frame_type(Arc<$frame_type>), )*
		}

		pub struct ContextType {
			$( $name: Arc<$context_type>, )*
		}

		$(
			impl IsFrame for $frame_type {
				fn to_frame(self) -> Frame {
					Frame::$erased_frame_type(Arc::new(self))
				}
			}
		)*

		impl ContextType {
			pub fn contexts(&self) -> Box<[Arc<Context>]> {
				[$(
					ContextKind::$erased_frame_type(self.$name.clone()),
				)*].into_iter().map(to_context).collect::<Vec<Arc<Context>>>().into_boxed_slice()
			}

			$(
				pub fn $name(&self) -> Arc<$context_type> { self.$name.clone() }
			)*
		}

		$(
			impl Context for $context_type {
				fn frequency(&self) -> u64 { $frequency }

				fn tick(&self, contexts: Arc<ContextType>) -> Frame {
					let last_frame = (match self.state().frame() {
						Frame::$erased_frame_type(f) => Some(f),
						_ => None,
					}).unwrap();

					Frame::$erased_frame_type(Arc::new($frame_type::new(contexts, last_frame)))
				}

				fn state(&self) -> &ContextState { &self.state }
			}
		)*
	};
}

mega_context!(
	{ InputContext,   InputFrame,   Input,   input,   120 },
	{ PhysicsContext, PhysicsFrame, Physics, physics, 120 },
	{ RenderContext,  RenderFrame,  Render,  render,   60 }
);
