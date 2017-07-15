use std::collections::{HashMap};
use std::path::{Path};
use std::sync::{Arc};

use crossbeam::sync::{MsQueue};
use glium::backend::{Facade};

use debug::{gnomon, indicator};
use inverse_kinematics::{Chain};
use model::{Model};
use unlit_model::{UnlitModel};
use render::render_frame::{RenderFrame};


pub const DEPTH_DIMENSION: u32 = 2048;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum ModelId {
	Player,
	Scene,
	IKModel, // TODO: we are going to need more of these / a dynamic way to generate ids and load at a later time
	Tree,

	// DEBUG
	Gnomon,
	Indicator,
}

pub struct RenderContext {
	pub q: Arc<MsQueue<RenderFrame>>, // TODO: make private and provide minimal decent api
	window_size: (u32, u32), // TODO: maybe this should be a per RenderFrame parameter
	pub models: HashMap<ModelId, Arc<Model>>,

	// DEBUG
	pub unlit_models: HashMap<ModelId, Arc<UnlitModel>>,
}

impl RenderContext {
	pub fn new<F: Facade>(facade: &F, q: Arc<MsQueue<RenderFrame>>, window_size: (u32, u32), ik_chains: &[Chain]) -> RenderContext {
		let model_map = load_initial_models(facade, ik_chains);

		// DEBUG
		let mut unlit_models = HashMap::new();
		unlit_models.insert(ModelId::Gnomon, Arc::new(gnomon::model(facade)));
		unlit_models.insert(ModelId::Indicator, Arc::new(indicator::model(facade)));

		RenderContext {
			q: q,
			window_size: window_size,
			models: model_map,

			// DEBUG
			unlit_models: unlit_models,
		}
	}

	pub fn aspect_ratio(&self) -> f32 {
		(self.window_size.0 as f32) / (self.window_size.1 as f32)
	}
}

// TODO: don't pass in chains but make something like IntoModel
//
fn load_initial_models<F: Facade>(facade: &F, ik_chains: &[Chain]) -> HashMap<ModelId, Arc<Model>> {
	let mut map = HashMap::new();

	const MODEL_PATH_STRINGS: [(ModelId, &'static str); 3] = [
		(ModelId::Player, "./data/player.obj"),
		(ModelId::Scene,  "./data/level.obj"),
		(ModelId::Tree,   "./data/tree.obj")
	];
	for &(model_id, path) in &MODEL_PATH_STRINGS {
		let model = Arc::new(Model::new(facade, &Path::new(path)));
		map.insert(model_id, model);
	}
	for chain in ik_chains {
		map.insert(ModelId::IKModel, Arc::new(chain.model(facade)));
	}
	map
}

unsafe impl Send for RenderContext {}
unsafe impl Sync for RenderContext {}
