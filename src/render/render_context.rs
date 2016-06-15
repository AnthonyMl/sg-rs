use std::sync::{Arc};

use cgmath::{InnerSpace, Vector3};
use crossbeam::sync::{MsQueue};

use render::render_frame::{RenderFrame};


pub struct RenderContext {
	pub q: Arc<MsQueue<RenderFrame>>, // TODO: make private and provide minimal decent api
	window_size: (u32, u32), // TODO: maybe this should be a per RenderFrame parameter
}

impl RenderContext {
	pub fn new(q: Arc<MsQueue<RenderFrame>>, window_size: (u32, u32)) -> RenderContext {
		RenderContext {
			q: q,
			window_size: window_size,
		}
	}

	pub fn aspect_ratio(&self) -> f64 {
		(self.window_size.0 as f64) / (self.window_size.1 as f64)
	}

	pub fn light_direction(&self) -> Vector3<f64> {
		Vector3::new(1.5, -1.0, -1.0).normalize()
	}
}

unsafe impl Send for RenderContext {}
unsafe impl Sync for RenderContext {}
