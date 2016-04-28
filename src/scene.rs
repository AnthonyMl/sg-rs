use std::path::{Path};

use model::{Model};
use glium::backend::{Facade};


pub struct Scene {
	pub model: Model,
}

impl Scene {
	pub fn new<T: Facade>(facade: &T) -> Scene {
		const SCENE_PATH_STRING: &'static str = "./data/level.obj";

		Scene {
			model: Model::new(facade, &Path::new(SCENE_PATH_STRING)),
		}
	}
}
