use std::collections::{HashMap};
use std::path::{Path};
use std::sync::{Arc};

use crossbeam::sync::{MsQueue};
use glium::backend::{Facade};

use debug::{gnomon, UnlitModel};
use inverse_kinematics::{Axis, Chain};
use model::{Model};
use render::render_frame::{RenderFrame};


pub const DEPTH_DIMENSION: u32 = 2048;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum ModelId {
	Player,
	Scene,
	IKModel,
	Tree,

	// DEBUG
	Gnomon,
}

pub struct RenderContext {
	pub q: Arc<MsQueue<RenderFrame>>, // TODO: make private and provide minimal decent api
	window_size: (u32, u32), // TODO: maybe this should be a per RenderFrame parameter
	pub models: HashMap<ModelId, Arc<Model>>,
	pub ik_chain: Chain, // TODO: don't keep this here

	// DEBUG
	pub unlit_models: HashMap<ModelId, Arc<UnlitModel>>,
}

impl RenderContext {
	pub fn new<F: Facade>(facade: &F, q: Arc<MsQueue<RenderFrame>>, window_size: (u32, u32)) -> RenderContext {
		let (model_map, chain) = load_initial_models(facade);

		// DEBUG
		let mut unlit_models = HashMap::new();
		unlit_models.insert(ModelId::Gnomon, Arc::new(gnomon::model(facade)));

		RenderContext {
			q: q,
			window_size: window_size,
			models: model_map,
			ik_chain: chain,

			// DEBUG
			unlit_models: unlit_models,
		}
	}

	pub fn aspect_ratio(&self) -> f64 {
		(self.window_size.0 as f64) / (self.window_size.1 as f64)
	}
}

fn load_initial_models<F: Facade>(facade: &F) -> (HashMap<ModelId, Arc<Model>>, Chain) {
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
	let chain = {
		let (chain, model) = Chain::new(facade, &[
			(0.0, Axis::Y),
			(3.0, Axis::Z),
			(3.0, Axis::Z),
			(3.0, Axis::Z)
		]);
		map.insert(ModelId::IKModel, Arc::new(model));
		chain
	};

	(map, chain)
}

unsafe impl Send for RenderContext {}
unsafe impl Sync for RenderContext {}
