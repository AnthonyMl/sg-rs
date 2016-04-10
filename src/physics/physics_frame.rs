use std::sync::{Arc, RwLock};

use cgmath::{Point3};

use camera::{Camera};
use input_frame::{InputFrame};


pub struct PhysicsFrame {
	pub camera: Camera,
	pub player_position: Point3<f64>,
	pub last_input_frame: Arc<RwLock<InputFrame>>,
}
