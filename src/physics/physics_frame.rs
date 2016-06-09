use std::sync::{Arc};
use std::f64::consts::{PI};

use cgmath::{Point3, Vector3, InnerSpace};

use camera::{Camera};
use context::{Context};
use input::{InputFrame};


// TODO: put in a soft cap on elevation with a slow drift
//
pub struct PhysicsFrame {
	pub frame_counter:   u64,
	pub camera:          Camera,
	pub player_position: Point3<f64>,
	pub azimuth:         f64,
	pub elevation:       f64,
}

impl PhysicsFrame {
	pub fn frame_zero(aspect_ratio: f64) -> PhysicsFrame {
		let player_position = Point3::new(0f64, 1f64, 0f64);
		let azimuth   = 0f64;
		let elevation = 0f64;
		let view_direction = PhysicsFrame::view_direction(azimuth, elevation);

		PhysicsFrame {
			frame_counter:   0,
			camera:          Camera::new(player_position, view_direction, aspect_ratio),
			player_position: player_position,
			azimuth:         azimuth,
			elevation:       elevation,
		}
	}

	pub fn new(context: Arc<Context>, frame: Arc<PhysicsFrame>, input_frame: Arc<InputFrame>) -> PhysicsFrame {
		const ELEVATION_LIMIT: f64 = 0.95;
		let angles_delta = -input_frame.action_state.view_direction; // TODO: scale

		let azimuth   = frame.azimuth   + angles_delta.x;
		let elevation = frame.elevation + angles_delta.y;
		let elevation = elevation.min(PI * ELEVATION_LIMIT).max(PI * (1f64 - ELEVATION_LIMIT));
		let view_direction = PhysicsFrame::view_direction(azimuth, elevation);
		let right = view_direction.cross(Vector3::new(0f64, 1f64, 0f64)).normalize();

		let input_direction = input_frame.action_state.movement_direction; // TODO: scale

		let flat_view_direction = (Vector3 { y: 0f64, .. view_direction }).normalize();
		let flat_right          = (Vector3 { y: 0f64, ..          right }).normalize();

		// TODO: generalize and factor out all integration
		//
		const FUDGE: f64 = 0.1f64;
		let acceleration
			= flat_view_direction * input_direction.x * FUDGE
			+ flat_right          * input_direction.y * FUDGE;

		let player_position = frame.player_position + acceleration;

		// TODO: this can be kicked up in a task at the start (if we use last frame's position)
		//
		let camera = Camera::new(player_position, view_direction, context.render.aspect_ratio());

		PhysicsFrame {
			frame_counter: frame.frame_counter + 1,
			camera: camera,
			player_position: player_position,
			azimuth: azimuth,
			elevation: elevation,
		}
	}

	fn view_direction(azimuth: f64, elevation: f64) -> Vector3<f64> {
		Vector3 {
			x:  elevation.sin() * azimuth.cos(),
			y: -elevation.cos(),
			z: -elevation.sin() * azimuth.sin(),
		}
	}

	pub fn get_view_direction(&self) -> Vector3<f64> {
		PhysicsFrame::view_direction(self.azimuth, self.elevation)
	}
}
