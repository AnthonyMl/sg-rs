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
mod input_context;
mod thread_pool;
mod vertex3;
mod camera;
mod uniform_wrappers;
mod game_loop;
mod frame_counter;
mod model;
mod scene;
mod input_event;
mod input_frame;
mod input_map;
mod action_state;
mod constants;
mod keyboard_state;
mod context_state;


fn main() {
	game_loop::init();
}
