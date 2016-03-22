#![feature(fnbox)]

#[macro_use]
extern crate glium;
extern crate crossbeam;
extern crate time;
extern crate cgmath;

mod context;
mod render;
mod physics_context;
mod input_context;
mod thread_pool;
mod vertex3;
mod camera;
mod uniform_wrappers;
mod game_loop;
mod frame_counter;


fn main() {
	game_loop::init();
}
