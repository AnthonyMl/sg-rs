use std::f32::consts::{PI};

use cgmath::{Vector3};

use inverse_kinematics::{Chain};


#[derive(Clone, PartialEq)]
pub enum State {
	Seeking {
		base_angles: Vec<f32>,
		target_angles: Vec<f32>,
		frames_to_wait: u16,
		num_transition_frames: u16,
		current_frame: u16,
		target: Vector3<f32>,
	},
	Waiting {
		target: Vector3<f32>,
		frames_to_wait: u16,
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
			State::Seeking { ref base_angles, ref target_angles, frames_to_wait, num_transition_frames, current_frame, target } => {
				let current_frame = current_frame + 1;

				let t = (current_frame + 1) as f32 / num_transition_frames as f32;

				let angles = base_angles.iter().zip(target_angles.iter()).map(
					|(base, target)| { base + t * (target - base) }
				).collect();

				let state = if current_frame == num_transition_frames {
					State::Waiting {
						target: target,
						frames_to_wait: frames_to_wait,
					}
				} else {
					State::Seeking {
						base_angles: base_angles.to_vec(),
						target_angles: target_angles.to_vec(),
						frames_to_wait: frames_to_wait,
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
					ik_fun: chain.ik_fun,
				}
			},
			State::Waiting { target, frames_to_wait } => {
				Chain {
					angles: chain.angles.to_vec(),
					joints: chain.joints.to_vec(),
					state:
					if frames_to_wait == 0 { State::Done }
					else {                   State::Waiting { target: target, frames_to_wait: frames_to_wait - 1 } }
					,
					position: chain.position,
					ik_fun: chain.ik_fun,
				}
			},
		},
		Transition::NewTarget{ target, num_transition_frames } => {
			let mut target_angles = (chain.ik_fun)(&chain, target);

			for (base, mut target) in chain.angles.iter().zip(target_angles.iter_mut()) {
				const TWO_PI: f32 = 2.0 * PI;
				let difference = *target - *base;
				if difference.abs() > PI { *target -= TWO_PI * (difference / TWO_PI).round(); }
			}

			let chain = Chain {
				joints: chain.joints.to_vec(),
				angles: chain.angles.to_vec(),
				state: State::Seeking {
					base_angles: chain.angles.to_vec(),
					target_angles: target_angles,
					frames_to_wait: 60,
					num_transition_frames: num_transition_frames,
					current_frame: 0,
					target: target,
				},
				position: chain.position,
				ik_fun: chain.ik_fun,
			};
			update(&chain, Transition::Update)
		},
	}
}
