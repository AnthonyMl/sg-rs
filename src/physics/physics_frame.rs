use cgmath::{Point3};

use camera::{Camera};


pub struct PhysicsFrame {
	pub camera: Camera,
	pub player_position: Point3<f64>,
}
