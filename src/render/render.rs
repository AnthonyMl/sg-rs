use std::sync::{Arc};
use crossbeam::sync::{MsQueue};
use glium::glutin::{WindowBuilder};
use glium::{DisplayBuild};

use render::render_context::{RenderContext};
use render::render_processor::{RenderProcessor};


pub fn create(width: usize, height: usize) -> (RenderContext, RenderProcessor) {
	let context = WindowBuilder::new()
		.with_dimensions(width as u32, height as u32)
		.with_title(format!("SG"))
		.build_glium().unwrap();

	let q = Arc::new(MsQueue::new());

	(
		RenderContext::new(q.clone()),
		RenderProcessor::new(q.clone(), context, width, height),
	)
}
