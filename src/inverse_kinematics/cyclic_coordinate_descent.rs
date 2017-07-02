use cgmath::{InnerSpace, Matrix4, Rad, SquareMatrix, Vector3, Vector4};

use inverse_kinematics::{Chain};


pub fn cyclic_coordinate_descent(chain: &Chain, target: Vector3<f32>) -> Vec<f32> {
	const DISTANCE_THRESHOLD: f32 = 0.01; // ADD DYNOMISM
	const PERP_LENGTH_THRESHOLD: f32 = 0.0001;
	const MAX_ITERATIONS: usize = 60;

	if chain.joints.is_empty() { return Vec::new() }

	let len = chain.joints.len();

	let mut cumulative_transforms = chain.cumulative_transforms();

	let mut angles: Vec<f32> = chain.angles.to_vec();

	let mut end = (cumulative_transforms.last().unwrap() * Vector4::unit_w()).truncate();

	for _ in 0..MAX_ITERATIONS {
		if (end - target).magnitude() < DISTANCE_THRESHOLD { break }

		let mut reverse_accumulator: Matrix4<f32> = Matrix4::identity();

		for i in (0..len).rev() {
			let (joint, prev_transform, mut current_transform, mut angle) = unsafe {
				(
					chain.joints.get_unchecked(i),
					if i > 0 { cumulative_transforms.get_unchecked(i - 1).clone() }
					else     { Matrix4::identity() },
					cumulative_transforms.get_unchecked_mut(i),
					angles.get_unchecked_mut(i),
				)
			};
			let root = (prev_transform * Vector4::unit_w()).truncate();
			let axis = (prev_transform * joint.axis.to_vector3().extend(0.0)).truncate();

			let target_dir = target - root;
			let target_perp = target_dir - (axis * axis.dot(target_dir));
			let target_perp_len = target_perp.magnitude();

			if target_perp_len < PERP_LENGTH_THRESHOLD { continue }

			let target_perp_norm = target_perp / target_perp_len;

			let end_dir = end - root;
			let end_perp = end_dir - (axis * axis.dot(end_dir));
			let end_perp_len = end_perp.magnitude();

			if end_perp_len < PERP_LENGTH_THRESHOLD { continue }

			let end_perp_norm = end_perp / end_perp_len;

			let cross = end_perp_norm.cross(target_perp_norm);
			let magnitude = cross.magnitude();
			let sign = if cross.dot(axis) > 0.0 { 1.0 } else { -1.0 };
			*angle += sign * magnitude.asin();

			// TODO: this assumption about Y being our offset direction needs to be put in a single place
			//
			let translation = Matrix4::from_translation(Vector3::new(0.0, joint.length, 0.0));
			let rotation    = Matrix4::from_axis_angle(joint.axis.to_vector3(), Rad(*angle));
			let transform   = rotation * translation;

			*current_transform = prev_transform * transform;
			reverse_accumulator = transform * reverse_accumulator;

			let total = prev_transform * reverse_accumulator;
			end = (total * Vector4::unit_w()).truncate();
		}
	}

	angles
}
