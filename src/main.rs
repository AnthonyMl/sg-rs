#[macro_use]
extern crate glium;
extern crate crossbeam;
extern crate time;
extern crate cgmath;
extern crate tobj;
#[macro_use]
extern crate mioco;

mod context;
mod render;
mod physics;
mod input;
mod camera;
mod model;
mod scene;
mod action_state;
mod scheduler;


fn main() {
	context::init();
}
