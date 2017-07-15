use cgmath::{Vector3, Vector4, InnerSpace, Matrix4, SquareMatrix};
use nalgebra::{Matrix, MatrixVec, Dynamic, U3, DVector};
use nalgebra::Vector3 as NALGVector3;

use inverse_kinematics::{Chain};


type JMatrix = Matrix<f32, U3, Dynamic, MatrixVec<f32, U3, Dynamic>>;

pub fn jacobian_pseudo_inverse(chain: &Chain, target: Vector3<f32>) -> Vec<f32> {
	const DISTANCE_THRESHOLD: f32 = 0.01; // ADD DYNOMISM
	const MAX_ITERATIONS: usize = 60;

	if chain.joints.is_empty() { return Vec::new() }

	let mut angles: DVector<f32> = DVector::from_column_slice(chain.angles.len(), chain.angles.as_slice());

	for _ in 0..MAX_ITERATIONS {
		let mut cumulative_transforms = vec![Matrix4::identity()];
		cumulative_transforms.extend(chain.cumulative_transforms_with_angles(angles.as_slice()).into_iter());

		let mut jacobian = JMatrix::from_element(chain.joints.len(), 0f32);

		let end = (cumulative_transforms.last().unwrap() * Vector4::unit_w()).truncate();

		if (end - target).magnitude() < DISTANCE_THRESHOLD { break }

		for (idx, (transform, joint)) in cumulative_transforms.iter().zip(chain.joints.iter()).enumerate() {
			let position = (transform * Vector4::unit_w()).truncate();

			let axis = (transform * joint.axis.to_vector3().extend(0f32)).truncate();

			let j = axis.cross(target - position);
			unsafe {
				*jacobian.get_unchecked_mut(0, idx) = j.x;
				*jacobian.get_unchecked_mut(1, idx) = j.y;
				*jacobian.get_unchecked_mut(2, idx) = j.z;
			}
		}
		let desired_change = target - end;

		let dc: NALGVector3<f32> = NALGVector3::new(desired_change.x, desired_change.y, desired_change.z);

		let transpose = jacobian.transpose();

		let inverse = (jacobian * transpose.clone()).try_inverse().unwrap();

		let pseudo_inverse = transpose * inverse;

		angles += pseudo_inverse * dc;
	}
	Vec::from(angles.as_slice())
}
