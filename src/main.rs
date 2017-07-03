#[macro_use]
extern crate glium;
extern crate crossbeam;
extern crate time;
extern crate cgmath;
extern crate tobj;
extern crate rand;
extern crate mioco;

mod context;
mod render;
mod physics;
mod input;
mod camera;
mod model;
mod scheduler;
mod inverse_kinematics;

mod debug;


fn main() {
	context::init();
}
