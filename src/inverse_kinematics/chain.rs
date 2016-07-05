use std::f64::consts::{PI};

use cgmath::{Matrix4, Rad, SquareMatrix, Vector3};
use glium::backend::{Facade};

use inverse_kinematics;
use model::{Model};


#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum Axis {
	X = 0,
	Y = 1,
	Z = 2,
}

impl Axis {
	pub fn to_vector3(&self) -> Vector3<f64> {
		match *self {
			Axis::X => Vector3::unit_x(),
			Axis::Y => Vector3::unit_y(),
			Axis::Z => Vector3::unit_z(),
		}
	}
}

struct Joint {
	angle:  f64,
	length: f64,
	axis:   Axis,
}

pub struct Chain {
	joints: Vec<Joint>,
	pub model: Model,
}

impl Chain {
	// TODO: don't hardcode angles but take them as parameters
	pub fn new<F: Facade>(facade: &F, lengths_and_axes: &[(f64, Axis)]) -> Chain {
		let joints = lengths_and_axes.iter().map(|&(length, axis)| {
			Joint { angle: PI/8.0, length: length, axis: axis }
		}).collect::<Vec<Joint>>();

		let model = inverse_kinematics::model::model(facade, lengths_and_axes);

		Chain {
			joints: joints,
			model: model,
		}
	}

	pub fn joint_transforms(&self) -> Vec<Matrix4<f64>> {
		let mut models = Vec::with_capacity(self.joints.len());
		let mut parent: Matrix4<f64> = Matrix4::identity();

		for joint in &self.joints {
			let r = Matrix4::from_axis_angle(joint.axis.to_vector3(), Rad::new(joint.angle));
			let t = Matrix4::from_translation(Vector3::new(0.0, joint.length as f64, 0.0));

			let pr = parent * r;
			if joint.length != 0.0 { models.push(pr) }
			parent = pr * t;
		}
		models
	}
}
