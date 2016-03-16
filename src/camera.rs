use cgmath::{Matrix4, Point3, Vector3, PerspectiveFov, Rad};


pub struct Camera {
	_mtx_view: Matrix4<f32>,
	_mtx_projection: Matrix4<f32>,
	pub mtx_full: Matrix4<f32>,
}

impl Camera {
	pub fn new(width: usize, height: usize) -> Camera {
//		const FIELD_OF_VIEW: f32 = std::f32::consts::FRAC_PI_3; TODO: why doesnt this work
		const FIELD_OF_VIEW: f32 = 1.0471975512f32;

		let eye = Point3::new(10.0f32, 10.0f32, 10.0f32);
		let center = Point3::new(0f32, 0f32, 0f32);
		let up = (center - eye).cross(Vector3::new(0f32, 1.0f32, 0f32));
		let view = Matrix4::look_at(eye, center, up);

		let projection = Matrix4::from(PerspectiveFov{
			fovy: Rad{ s: FIELD_OF_VIEW},
			aspect: (width as f32) / (height as f32),
			near: 1f32,
			far: 100f32
		});

		let full = projection * view;

		Camera {
			_mtx_view: view,
			_mtx_projection: projection,
			mtx_full: full,
		}
	}
}
