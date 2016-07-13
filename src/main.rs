#[macro_use]
extern crate glium;
extern crate crossbeam;
extern crate time;
extern crate cgmath;
extern crate tobj;
extern crate rand;
#[macro_use]
extern crate mioco;

mod context;
mod render;
mod physics;
mod input;
mod camera;
mod model;
mod action_state;
mod scheduler;
mod inverse_kinematics;

mod debug;


fn main() {
	context::init();
}
