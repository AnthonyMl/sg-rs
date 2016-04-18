use std::sync::{Arc};

use crossbeam::sync::{MsQueue};
use glium::glutin::{WindowBuilder};
use glium::{DisplayBuild};

use render::{RenderContext, RenderProcessor, RenderCommand};
use input::{InputContext};
use physics::{PhysicsContext};
use context::context_state::{ContextStateProxy};


// TODO: maybe rename ContextType->Context and Context->IsContext or something like that?
//
pub struct ContextType {
	context_input:   Arc<InputContext>,
	context_physics: Arc<PhysicsContext>,
	_context_render:  Arc<RenderContext>,
	contexts: [Arc<Context + Send + Sync>; 3],
}
impl ContextType {
	fn new(q: Arc<MsQueue<RenderCommand>>,width: u32, height: u32) -> ContextType {
		let ai = Arc::new(InputContext::new());
		let ap = Arc::new(PhysicsContext::new(width, height));
		let ar = Arc::new(RenderContext::new(q));

		ContextType {
			context_input:   ai.clone(),
			context_physics: ap.clone(),
			_context_render:  ar.clone(),
			contexts: [ai.clone(), ap.clone(), ar.clone()]
		}
	}

	pub fn context_input  (&self) -> Arc<InputContext>   { self.context_input  .clone() }
	pub fn context_physics(&self) -> Arc<PhysicsContext> { self.context_physics.clone() }
	pub fn _context_render (&self) -> Arc<RenderContext>  { self._context_render .clone() }
	pub fn contexts       (&self) -> &[Arc<Context + Send + Sync>] { self.contexts.as_ref() }
	pub fn len            (&self) -> usize { self.contexts.len() }
}
unsafe impl Send for ContextType {}
unsafe impl Sync for ContextType {}

pub trait Context {
	fn frequency(&self) -> u64;
	fn state(&self) -> &ContextStateProxy;

	// TODO try to remove Arc dependency
	fn do_tick(&self, contexts: Arc<ContextType>) {
		self.state().pre_tick();
		self.tick(contexts);
		self.state().post_tick();
	}
	fn tick(&self, Arc<ContextType>); // TODO: make this private
}

// TODO: try to remove Arc dependency
//
pub fn create(width: u32, height: u32) -> (Arc<ContextType>, RenderProcessor) {
	let glium_context = WindowBuilder::new()
		.with_dimensions(width, height)
		.with_title(format!("SG"))
		.with_depth_buffer(24)
		.build_glium().unwrap();

	let q = Arc::new(MsQueue::new());

	(
		Arc::new(
			ContextType::new(
				q.clone(),
				width,
				height
			)
		),
		RenderProcessor::new(q.clone(), glium_context),
	)
}
