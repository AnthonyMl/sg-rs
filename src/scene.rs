use std::path::{Path};

use model::{Model};
use glium::backend::glutin_backend::{GlutinFacade};


pub struct Scene {
	pub model: Model,
}

impl Scene {
	pub fn new(context: &GlutinFacade) -> Scene {
		const SCENE_PATH_STRING: &'static str = "./data/level.obj";

		Scene {
			model: Model::new(context, &Path::new(SCENE_PATH_STRING)),
		}
	}
}
