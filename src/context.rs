use std::sync::{Arc};

use crossbeam::sync::{MsQueue};
use glium::glutin::{WindowBuilder};
use glium::{DisplayBuild};

use render::{RenderContext, RenderProcessor, RenderCommand};
use input_context::{InputContext};
use physics::{PhysicsContext};
use context_state::{ContextStateProxy};


// TODO: maybe rename ContextType->Context and Context->IsContext or something like that?
//
//type ContextSubType = Context + Send + Sync + 'static;
//pub type ContextType = Arc<ContextType_ + Send + Sync + 'static>;

pub trait ContextType {
	fn context_input(&self)   -> Arc<InputContext>;
	fn context_physics(&self) -> Arc<PhysicsContext>;
	fn context_render(&self)  -> Arc<RenderContext>;
	fn contexts(&self)        -> &[Arc<Context + Send + Sync>];
	fn len(&self)             -> usize;
}

struct ContextType_ {
	context_input:   Arc<InputContext>,
	context_physics: Arc<PhysicsContext>,
	context_render:  Arc<RenderContext>,
	contexts: [Arc<Context + Send + Sync>; 3],
}
impl ContextType_ {
	fn new(q: Arc<MsQueue<RenderCommand>>,width: u32, height: u32) -> ContextType_ {
		let ai = Arc::new(InputContext::new());
		let ap = Arc::new(PhysicsContext::new(width, height));
		let ar = Arc::new(RenderContext::new(q));

		ContextType_ {
			context_input:   ai.clone(),
			context_physics: ap.clone(),
			context_render:  ar.clone(),
			contexts: [ai.clone(), ap.clone(), ar.clone()]
		}
	}
}
impl ContextType for ContextType_ {
	fn context_input  (&self) -> Arc<InputContext>   { self.context_input  .clone() }
	fn context_physics(&self) -> Arc<PhysicsContext> { self.context_physics.clone() }
	fn context_render (&self) -> Arc<RenderContext>  { self.context_render .clone() }
	fn contexts       (&self) -> &[Arc<Context + Send + Sync>] { self.contexts.as_ref() }
	fn len            (&self) -> usize { self.contexts.len() }
}
unsafe impl Send for ContextType_ {}
unsafe impl Sync for ContextType_ {}

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
pub fn create(width: u32, height: u32) -> (Arc<ContextType + Send + Sync>, RenderProcessor) {
	let glium_context = WindowBuilder::new()
		.with_dimensions(width, height)
		.with_title(format!("SG"))
		.with_depth_buffer(24)
		.build_glium().unwrap();

	let q = Arc::new(MsQueue::new());

	(
		Arc::new(
			ContextType_::new(
				q.clone(),
				width,
				height
			)
		),
		RenderProcessor::new(q.clone(), glium_context),
	)
}
