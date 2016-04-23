use std::f64::consts::{FRAC_PI_3};

use cgmath::{Matrix4, Point, Point3, Vector3, PerspectiveFov, Rad, EuclideanVector};


#[derive(Clone)]
pub struct Camera {
	pub view:       Matrix4<f64>,
	pub projection: Matrix4<f64>
}

impl Camera {
	pub fn new(center: Point3<f64>, view_direction: Vector3<f64>, (width, height): (u32, u32)) -> Camera {
		const FIELD_OF_VIEW: f64 = FRAC_PI_3;
		const DISTANCE: f64 = 10f64;
		const CENTER_OFFSET: Vector3<f64> = Vector3{ x: 0f64, y: 5f64, z: 0f64};

		let center  = center + CENTER_OFFSET;
		let eye		= Point3::from_vec(center.to_vec() - (view_direction * DISTANCE));
		let forward	= (center - eye).normalize();
		let right	= forward.cross(Vector3::new(0f64, 1f64, 0f64));
		let up		= right.cross(forward);
		let view	= Matrix4::look_at(eye, center, up);

		let projection = Matrix4::from(PerspectiveFov{
			fovy: Rad{ s: FIELD_OF_VIEW },
			aspect: (width as f64) / (height as f64),
			near: 1f64,
			far: 100f64,
		});

		Camera {
			view: view,
			projection: projection,
		}
	}
}
