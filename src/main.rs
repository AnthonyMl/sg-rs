#![feature(fnbox)]

#[macro_use]
extern crate glium;
extern crate crossbeam;
extern crate time;
extern crate cgmath;
extern crate tobj;

mod context;
mod render;
mod physics;
mod input;
mod thread_pool;
mod camera;
mod game_loop;
mod frame_counter;
mod model;
mod scene;
mod action_state;
mod frame;


fn main() {
	game_loop::init();
}
