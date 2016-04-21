use std::sync::{Arc};

use crossbeam::sync::{MsQueue};
use glium::glutin::{WindowBuilder, CursorState};
use glium::{DisplayBuild};

use render::{RenderContext, RenderProcessor, RenderCommand, RenderFrame};
use input::{InputContext, InputFrame};
use physics::{PhysicsContext, PhysicsFrame};
use context::context_state::{ContextState};
use frame::{Frame};


// TODO: try to remove Arc dependency
// TODO: maybe rename ContextType->Context and Context->IsContext or something like that?
//
pub struct ContextType {
	context_input:   Arc<InputContext>,
	context_physics: Arc<PhysicsContext>,
	context_render:  Arc<RenderContext>,
}

impl ContextType {
	fn new(q: Arc<MsQueue<RenderCommand>>, window_size: (u32, u32)) -> ContextType {
		let ai = Arc::new(InputContext::new());
		let ap = Arc::new(PhysicsContext::new(window_size));
		let ar = Arc::new(RenderContext::new(q, window_size));

		ContextType {
			context_input:   ai.clone(),
			context_physics: ap.clone(),
			context_render:  ar.clone(),
		}
	}

	pub fn contexts(&self) -> Box<[Arc<Context>]> {
		[ ContextKind::Input(  self.context_input  .clone())
		, ContextKind::Physics(self.context_physics.clone())
		, ContextKind::Render( self.context_render .clone())
		].into_iter().map(to_context).collect::<Vec<Arc<Context>>>().into_boxed_slice()
	}

	pub fn context_input  (&self) -> Arc<InputContext>   { self.context_input  .clone() }
	pub fn context_physics(&self) -> Arc<PhysicsContext> { self.context_physics.clone() }
	pub fn context_render (&self) -> Arc<RenderContext>  { self.context_render .clone() }
}
unsafe impl Send for ContextType {}
unsafe impl Sync for ContextType {}

enum ContextKind {
	Input(  Arc<InputContext>),
	Physics(Arc<PhysicsContext>),
	Render( Arc<RenderContext>),
}

fn to_context(context: &ContextKind) -> Arc<Context> {
	match *context {
		ContextKind::Input(  ref ic) => ic.clone(),
		ContextKind::Physics(ref pc) => pc.clone(),
		ContextKind::Render( ref rc) => rc.clone(),
	}
}

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

pub fn create(window_size: (u32, u32)) -> (Arc<ContextType>, RenderProcessor) {
	let glium_context = WindowBuilder::new()
		.with_dimensions(window_size.0, window_size.1)
		.with_title(format!("SG"))
		.with_depth_buffer(24)
		.build_glium().unwrap();

	glium_context.get_window().unwrap().set_cursor_state(CursorState::Grab).ok();

	let q = Arc::new(MsQueue::new());

	(
		Arc::new(ContextType::new(q.clone(), window_size)),
		RenderProcessor::new(q, glium_context),
	)
}

macro_rules! register_context {
	($context_type:ty, $frame_type:ident, $erased_frame_type:ident, $frequency:expr) => {
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
	}
}
register_context!(InputContext,   InputFrame,   Input,   120);
register_context!(PhysicsContext, PhysicsFrame, Physics, 120);
register_context!(RenderContext,  RenderFrame,  Render,   60);
