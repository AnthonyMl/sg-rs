use std::f32::consts::{FRAC_PI_3};

use cgmath::{Matrix4, Point3, Vector3, PerspectiveFov, Rad, EuclideanVector};


pub struct Camera {
	pub mtx_full: Matrix4<f32>,
}

impl Camera {
	pub fn new(width: usize, height: usize) -> Camera {
		const FIELD_OF_VIEW: f32 = FRAC_PI_3;
		const DISTANCE: f32 = 10f32;

		let eye		= Point3::new(0f32, 5f32, -DISTANCE);
		let center	= Point3::new(0f32, 5f32, 0f32);
		let forward	= (center - eye).normalize();
		let right	= forward.cross(Vector3::new(0f32, 1f32, 0f32));
		let up		= right.cross(forward);
		let view	= Matrix4::look_at(eye, center, up);

		let projection = Matrix4::from(PerspectiveFov{
			fovy: Rad{ s: FIELD_OF_VIEW },
			aspect: (width as f32) / (height as f32),
			near: 1f32,
			far: 100f32
		});

		let full = projection * view;

		Camera {
			mtx_full: full,
		}
	}
}
