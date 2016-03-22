use std::sync::{Arc};

use crossbeam::sync::{MsQueue};
use glium::glutin::{WindowBuilder};
use glium::{DisplayBuild};

use render::{RenderContext, RenderProcessor};
use input_context::{InputContext};
use physics_context::{PhysicsContext};


// TODO: maybe rename ContextType->Context and Context->IsContext or something like that?
//
pub type ContextType = Arc<Vec<Arc<Context + Send + Sync + 'static>>>;

pub trait Context {
	fn rate(&self) -> u64;
	fn tick(&self);
}

pub fn create(width: usize, height: usize) -> (ContextType, RenderProcessor) {
	let context = WindowBuilder::new()
		.with_dimensions(width as u32, height as u32)
		.with_title(format!("SG"))
		.build_glium().unwrap();

	let q = Arc::new(MsQueue::new());

	let contexts: ContextType = Arc::new(vec![
		Arc::new(InputContext::new()),
		Arc::new(PhysicsContext::new()),
		Arc::new(RenderContext::new(q.clone())),
	]);

	(
		contexts,
		RenderProcessor::new(q.clone(), context, width, height),
	)
}
