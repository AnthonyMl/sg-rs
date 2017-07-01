use cgmath::{Vector3, Vector4, InnerSpace, Matrix4, SquareMatrix};

use inverse_kinematics::{Chain};


pub fn jacobian_transpose(chain: &Chain, target: Vector3<f32>) -> Vec<f32> {
	const DISTANCE_THRESHOLD: f32 = 0.01; // ADD DYNOMISM
	const MAX_ITERATIONS: usize = 60;
	const FUDGE: f32 = 0.01;

	if chain.joints.is_empty() { return Vec::new() }

	let mut angles = chain.angles.clone();

	for _ in 0..MAX_ITERATIONS {
		let mut cumulative_transforms = vec![Matrix4::identity()];
		cumulative_transforms.extend(chain.cumulative_transforms_with_angles(&angles).into_iter());

		let end = (cumulative_transforms.last().unwrap() * Vector4::unit_w()).truncate();

		if (end - target).magnitude() < DISTANCE_THRESHOLD { break }

		let desired_change = target - end;

		for (idx, (transform, joint)) in cumulative_transforms.iter().zip(chain.joints.iter()).enumerate() {
			let position = (transform * Vector4::unit_w()).truncate();

			let axis = (transform * joint.axis.to_vector3().extend(0f32)).truncate();

			let j = axis.cross(target - position);

			angles[idx] += FUDGE * j.dot(desired_change);
		}
	}
	angles
}
