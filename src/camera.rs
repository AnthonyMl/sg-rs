use std::f64::consts::{FRAC_PI_3};

use cgmath::{EuclideanSpace, InnerSpace, Matrix, Matrix4, PerspectiveFov, Point3, Rad, Vector3};


pub const NEAR_PLANE:    f64 = 1.0;
pub const FAR_PLANE:     f64 = 60.0;
pub const FIELD_OF_VIEW: f64 = FRAC_PI_3;

#[derive(Clone)]
pub struct Camera {
	pub view:         Matrix4<f64>,
	pub projection:   Matrix4<f64>,
	pub aspect_ratio: f64,
}

impl Camera {
	pub fn new(center: Point3<f64>, view_direction: Vector3<f64>, aspect_ratio: f64) -> Camera {
		const DISTANCE: f64 = 10.0;
		const CENTER_OFFSET: Vector3<f64> = Vector3{ x: 0.0, y: 5.0, z: 0.0};

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
			view: view,
			projection: projection,
			aspect_ratio: aspect_ratio,
		}
	}

	pub fn view_corners(&self) -> Vec<Vector3<f64>> {
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
}
