use std::f32::consts::{PI, FRAC_PI_3};

use cgmath::{EuclideanSpace, InnerSpace, Matrix, Matrix4, PerspectiveFov, Point3, Rad, Vector3};


pub const NEAR_PLANE:    f32 = 1.0;
pub const FAR_PLANE:     f32 = 60.0;
pub const FIELD_OF_VIEW: f32 = FRAC_PI_3;

#[derive(Clone)]
pub struct Camera {
	pub azimuth:      f32,
	pub elevation:    f32,
	pub aspect_ratio: f32,
	pub view:         Matrix4<f32>,
	pub projection:   Matrix4<f32>,
}

impl Camera {
	pub fn new(center: Point3<f32>, azimuth: f32, elevation: f32, aspect_ratio: f32) -> Camera {
		const DISTANCE: f32 = 10.0;
		const CENTER_OFFSET: Vector3<f32> = Vector3{ x: 0.0, y: 5.0, z: 0.0};

		let view_direction = to_view_direction(azimuth, elevation);
		let center  = center + CENTER_OFFSET;
		let eye		= Point3::from_vec(center.to_vec() - (view_direction * DISTANCE));
		let forward	= (center - eye).normalize();
		let right	= forward.cross(Vector3::new(0.0, 1.0, 0.0));
		let up		= right.cross(forward);
		let view	= Matrix4::look_at(eye, center, up);

		let projection = Matrix4::from(PerspectiveFov{
			fovy: Rad{ s: FIELD_OF_VIEW },
			aspect: aspect_ratio,
			near: NEAR_PLANE,
			far: FAR_PLANE,
		});

		Camera {
			azimuth: azimuth,
			elevation: elevation,
			view: view,
			projection: projection,
			aspect_ratio: aspect_ratio,
		}
	}

	pub fn update(&self, center: Point3<f32>, azimuth_delta: f32, elevation_delta: f32, aspect_ratio: f32) -> Camera {
		const ELEVATION_LIMIT: f32 = 0.95;

		let azimuth   = self.azimuth   + azimuth_delta;
		let elevation = self.elevation + elevation_delta;
		let elevation = elevation.min(PI * ELEVATION_LIMIT).max(PI * (1f32 - ELEVATION_LIMIT));

		Camera::new(center, azimuth, elevation, aspect_ratio)
	}

	pub fn view_corners(&self) -> Vec<Vector3<f32>> {
		let view = self.view.transpose();
		let view_origin = (view * self.view.w.clone()).truncate() * -1.0;
		let right = view.x.truncate();
		let up    = view.y.truncate();
		let look  = view.z.truncate() * -1.0;

		let tan = (0.5 * FIELD_OF_VIEW).tan();
		vec![
			(-1.0, -1.0, FAR_PLANE),
			( 1.0,  1.0, FAR_PLANE),
			( 1.0, -1.0, FAR_PLANE),
			(-1.0,  1.0, FAR_PLANE),
			(-1.0, -1.0, NEAR_PLANE),
			( 1.0,  1.0, NEAR_PLANE),
			( 1.0, -1.0, NEAR_PLANE),
			(-1.0,  1.0, NEAR_PLANE)
		].iter().map(|v|{
			let vertical_length = v.2 * tan;

			view_origin
			+ right * (v.0 * vertical_length * self.aspect_ratio)
			+ up    * (v.1 * vertical_length)
			+ look  *  v.2
		}).collect()
	}

	pub fn view_direction(&self) -> Vector3<f32> {
		to_view_direction(self.azimuth, self.elevation)
	}
}

pub fn to_view_direction(azimuth: f32, elevation: f32) -> Vector3<f32> {
	Vector3 {
		x:  elevation.sin() * azimuth.cos(),
		y: -elevation.cos(),
		z: -elevation.sin() * azimuth.sin(),
	}
}
