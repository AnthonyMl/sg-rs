use cgmath::{Matrix4, Rad, SquareMatrix, Vector3};
use glium::{IndexBuffer, VertexBuffer};
use glium::backend::{Facade};
use glium::index::{PrimitiveType};

use inverse_kinematics::{State};
use model::{Model};
use render::vertices::{ForwardVertex};


#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum Axis {
	X = 0,
	Y = 1,
	Z = 2,
}

impl Axis {
	pub fn to_vector3(&self) -> Vector3<f32> {
		match *self {
			Axis::X => Vector3::unit_x(),
			Axis::Y => Vector3::unit_y(),
			Axis::Z => Vector3::unit_z(),
		}
	}
}

#[derive(Copy, Clone)]
pub struct Joint {
	pub length: f32,
	pub axis:   Axis,
}

pub struct Chain {
	pub joints:   Vec<Joint>,
	pub angles:   Vec<f32>,
	pub state:    State,
	pub position: Vector3<f32>,
	pub ik_fun:   fn(&Chain, Vector3<f32>) -> Vec<f32>,
}

impl Clone for Chain {
	fn clone(&self) -> Self {
		Self {
			joints: self.joints.clone(),
			angles: self.angles.clone(),
			state: self.state.clone(),
			position: self.position,
			ik_fun: self.ik_fun
		}
	}
}

impl Chain {
	pub fn visible_joint_transforms(&self) -> Vec<Matrix4<f32>> {
		let mut models = Vec::with_capacity(self.joints.len());
		let mut parent: Matrix4<f32> = Matrix4::identity();

		for (joint, angle) in self.joints.iter().zip(self.angles.iter()) {
			let r = Matrix4::from_axis_angle(joint.axis.to_vector3(), Rad(*angle));
			let t = Matrix4::from_translation(Vector3::new(0.0, joint.length, 0.0));

			let pr = parent * r;
			if joint.length != 0.0 { models.push(pr) }
			parent = pr * t;
		}
		models
	}

	pub fn cumulative_transforms(&self) -> Vec<Matrix4<f32>> {
		self.cumulative_transforms_with_angles(&self.angles)
	}

	pub fn cumulative_transforms_with_angles(&self, angles: &[f32]) -> Vec<Matrix4<f32>> {
		let mut models = Vec::with_capacity(self.joints.len());
		let mut accumulator: Matrix4<f32> = Matrix4::identity();

		for (joint, angle) in self.joints.iter().zip(angles.iter()) {
			let r = Matrix4::from_axis_angle(joint.axis.to_vector3(), Rad(*angle));
			let t = Matrix4::from_translation(Vector3::new(0.0, joint.length, 0.0));

			accumulator = accumulator * r * t;
			models.push(accumulator);
		}
		models
	}

	pub fn model<F: Facade>(&self, facade: &F) -> Model {
		const S: f32 = 0.4f32;
		const NUM_FACES: usize = 6;
		const NUM_VERTS_PER_FACE: usize = 4;
		const NUM_INDICES_PER_FACE: usize = 6;

		let num_visible_joints = self.joints.iter().filter(|j| j.length != 0.0).count();

		let mut vertices = Vec::with_capacity(num_visible_joints * NUM_FACES * NUM_VERTS_PER_FACE);
		let mut indices  = Vec::with_capacity(num_visible_joints * NUM_FACES * NUM_INDICES_PER_FACE);

		for joint in self.joints.iter().filter(|j| j.length != 0.0) {
			let len = joint.length;
			let base = vertices.len();

			vertices.push(ForwardVertex { position: [-S, 0f32,  S], normal: [ 0f32,  0f32,  1f32] });
			vertices.push(ForwardVertex { position: [ S, 0f32,  S], normal: [ 0f32,  0f32,  1f32] });
			vertices.push(ForwardVertex { position: [-S,  len,  S], normal: [ 0f32,  0f32,  1f32] });
			vertices.push(ForwardVertex { position: [ S,  len,  S], normal: [ 0f32,  0f32,  1f32] });

			vertices.push(ForwardVertex { position: [ S, 0f32,  S], normal: [ 1f32,  0f32,  0f32] });
			vertices.push(ForwardVertex { position: [ S, 0f32, -S], normal: [ 1f32,  0f32,  0f32] });
			vertices.push(ForwardVertex { position: [ S,  len,  S], normal: [ 1f32,  0f32,  0f32] });
			vertices.push(ForwardVertex { position: [ S,  len, -S], normal: [ 1f32,  0f32,  0f32] });

			vertices.push(ForwardVertex { position: [ S, 0f32, -S], normal: [ 0f32,  0f32, -1f32] });
			vertices.push(ForwardVertex { position: [-S, 0f32, -S], normal: [ 0f32,  0f32, -1f32] });
			vertices.push(ForwardVertex { position: [ S,  len, -S], normal: [ 0f32,  0f32, -1f32] });
			vertices.push(ForwardVertex { position: [-S,  len, -S], normal: [ 0f32,  0f32, -1f32] });

			vertices.push(ForwardVertex { position: [-S, 0f32, -S], normal: [-1f32,  0f32,  0f32] });
			vertices.push(ForwardVertex { position: [-S, 0f32,  S], normal: [-1f32,  0f32,  0f32] });
			vertices.push(ForwardVertex { position: [-S,  len, -S], normal: [-1f32,  0f32,  0f32] });
			vertices.push(ForwardVertex { position: [-S,  len,  S], normal: [-1f32,  0f32,  0f32] });

			vertices.push(ForwardVertex { position: [-S,  len,  S], normal: [ 0f32,  1f32,  0f32] });
			vertices.push(ForwardVertex { position: [ S,  len,  S], normal: [ 0f32,  1f32,  0f32] });
			vertices.push(ForwardVertex { position: [-S,  len, -S], normal: [ 0f32,  1f32,  0f32] });
			vertices.push(ForwardVertex { position: [ S,  len, -S], normal: [ 0f32,  1f32,  0f32] });

			vertices.push(ForwardVertex { position: [-S, 0f32, -S], normal: [ 0f32, -1f32,  0f32] });
			vertices.push(ForwardVertex { position: [ S, 0f32, -S], normal: [ 0f32, -1f32,  0f32] });
			vertices.push(ForwardVertex { position: [-S, 0f32,  S], normal: [ 0f32, -1f32,  0f32] });
			vertices.push(ForwardVertex { position: [ S, 0f32,  S], normal: [ 0f32, -1f32,  0f32] });

			for i in 0..NUM_INDICES_PER_FACE {
				let base = (base + i * NUM_VERTS_PER_FACE) as u32;
				indices.push(base + 0);
				indices.push(base + 1);
				indices.push(base + 2);
				indices.push(base + 2);
				indices.push(base + 1);
				indices.push(base + 3);
			}
		}

		Model {
			vertex_buffer: VertexBuffer::new(facade, &vertices).unwrap(),
			index_buffer:  IndexBuffer ::new(facade, PrimitiveType::TrianglesList, &indices).unwrap(),
		}
	}
}
