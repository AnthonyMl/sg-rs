use std::sync::{Arc};
use std::f32::consts::{PI};

use cgmath::{Point3, Vector3, InnerSpace};

use camera::{Camera};
use context::{Context};
use input::{InputFrame};


// TODO: put in a soft cap on elevation with a slow drift
//
pub struct PhysicsFrame {
	pub frame_counter:   u64,
	pub camera:          Camera,
	pub player_position: Point3<f32>,
	pub azimuth:         f32,
	pub elevation:       f32,

	pub light_direction: Vector3<f32>,
	pub aspect_ratio:    f32,
}

impl PhysicsFrame {
	pub fn frame_zero(aspect_ratio: f32) -> PhysicsFrame {
		let light_direction = Vector3::new(1.0, -1.0, -1.5).normalize();
		let player_position = Point3::new(0f32, 1f32, 0f32);
		let azimuth   = 0f32;
		let elevation = 0f32;
		let view_direction = PhysicsFrame::view_direction(azimuth, elevation);

		PhysicsFrame {
			frame_counter:   0,
			camera:          Camera::new(player_position, view_direction, aspect_ratio),
			player_position: player_position,
			azimuth:         azimuth,
			elevation:       elevation,
			light_direction: light_direction,
			aspect_ratio:    aspect_ratio,
		}
	}

	pub fn new(context: Arc<Context>, frame: Arc<PhysicsFrame>, input_frame: Arc<InputFrame>) -> PhysicsFrame {
		const ELEVATION_LIMIT: f32 = 0.95;
		let angles_delta = -input_frame.action_state.view_direction; // TODO: scale

		let azimuth   = frame.azimuth   + angles_delta.x;
		let elevation = frame.elevation + angles_delta.y;
		let elevation = elevation.min(PI * ELEVATION_LIMIT).max(PI * (1f32 - ELEVATION_LIMIT));
		let view_direction = PhysicsFrame::view_direction(azimuth, elevation);
		let right = view_direction.cross(Vector3::new(0f32, 1f32, 0f32)).normalize();

		let input_direction = input_frame.action_state.movement_direction; // TODO: scale

		let flat_view_direction = (Vector3 { y: 0f32, .. view_direction }).normalize();
		let flat_right          = (Vector3 { y: 0f32, ..          right }).normalize();

		// TODO: generalize and factor out all integration
		//
		const FUDGE: f32 = 0.1f32;
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
			light_direction: frame.light_direction,
			aspect_ratio: frame.aspect_ratio,
		}
	}

	fn view_direction(azimuth: f32, elevation: f32) -> Vector3<f32> {
		Vector3 {
			x:  elevation.sin() * azimuth.cos(),
			y: -elevation.cos(),
			z: -elevation.sin() * azimuth.sin(),
		}
	}

	pub fn get_view_direction(&self) -> Vector3<f32> {
		PhysicsFrame::view_direction(self.azimuth, self.elevation)
	}
}
