use std::sync::{Arc};

use crossbeam::sync::{MsQueue};
use glium::glutin::{WindowBuilder};
use glium::{DisplayBuild};

use render::{RenderContext, RenderProcessor, RenderCommand};
use input::{InputContext};
use physics::{PhysicsContext};
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
	fn new(q: Arc<MsQueue<RenderCommand>>,width: u32, height: u32) -> ContextType {
		let ai = Arc::new(InputContext::new());
		let ap = Arc::new(PhysicsContext::new(width, height));
		let ar = Arc::new(RenderContext::new(q, ap.state().frame()));

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
	pub fn _context_render(&self) -> Arc<RenderContext>  { self.context_render .clone() }
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
	fn frequency(&self) -> u64;
	fn state(&self) -> &ContextState;
	fn tick(&self, Arc<ContextType>, Frame) -> Frame;

	fn do_tick(&self, contexts: Arc<ContextType>) {
		self.state().tick_enter();
		let f = self.tick(contexts, self.state().frame());
		self.state().tick_exit(f);
	}
}

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
