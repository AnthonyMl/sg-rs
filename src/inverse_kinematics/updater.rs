use cgmath::{Vector3};

use inverse_kinematics::{Chain};
use inverse_kinematics::cyclic_coordinate_descent::{cyclic_coordinate_descent};


#[derive(Clone, PartialEq)]
pub enum State {
	Seeking {
		base_angles: Vec<f32>,
		target_angles: Vec<f32>,
		num_transition_frames: u16,
		current_frame: u16,
		target: Vector3<f32>,
	},
	Done,
}

pub enum Transition {
	NewTarget { target: Vector3<f32>, num_transition_frames: u16 },
	Update,
}

pub fn update(chain: &Chain, transition: Transition) -> Chain {
	match transition {
		Transition::Update => match chain.state {
			State::Done => chain.clone(),
			State::Seeking { ref base_angles, ref target_angles, num_transition_frames, current_frame, target } => {
				let current_frame = current_frame + 1;

				let t = current_frame as f32 / num_transition_frames as f32;

				// TODO: maybe take the shorter angle and not the positive one
				//
				let angles = base_angles.iter().zip(target_angles.iter()).map(|(base, target)| { base + t * (target - base) }).collect();

				let state = if current_frame == num_transition_frames {
					State::Done
				} else {
					State::Seeking {
						base_angles: base_angles.to_vec(),
						target_angles: target_angles.to_vec(),
						num_transition_frames: num_transition_frames,
						current_frame: current_frame,
						target: target,
					}
				};

				Chain {
					angles: angles,
					joints: chain.joints.to_vec(),
					state: state,
					position: chain.position,
				}
			},
		},
		Transition::NewTarget{ target, num_transition_frames } => {
			let target_angles = cyclic_coordinate_descent(&chain, target);

			let chain = Chain {
				joints: chain.joints.to_vec(),
				angles: chain.angles.to_vec(),
				state: State::Seeking {
					base_angles: chain.angles.to_vec(),
					target_angles: target_angles,
					num_transition_frames: num_transition_frames,
					current_frame: 0,
					target: target,
				},
				position: chain.position,
			};
			update(&chain, Transition::Update)
		},
	}
}
