#![allow(dead_code)]

#[macro_use] extern crate itertools;
extern crate num;
extern crate rand;
extern crate rustty;

#[macro_use] mod ecs;
#[macro_use] mod debug;
mod perlin;
mod renderer;
mod geometry;
mod grid;
mod colour;
mod game;
mod terminal;
mod allocator;
mod tests;

use std::io;
use terminal::window_manager::WindowManager;
use terminal::window_buffer::WindowBuffer;

const DEBUG_WINDOW_WIDTH: usize = 40;
const DEBUG_WINDOW_HEIGHT: usize = 8;

fn main() {

    let wm = terminal::window_manager::WindowManager::new().unwrap();

    let mut debug_buffer = make_debug_window(&wm, DEBUG_WINDOW_WIDTH,
                                                  DEBUG_WINDOW_HEIGHT);
    debug::debug::init(&mut debug_buffer as &mut io::Write);

    let input_source = wm.make_input_source();

    debug_println!("Hello, World!");

    input_source.get_event().unwrap();
}



fn make_debug_window<'a>(wm: &'a WindowManager, width: usize, height: usize)
    -> WindowBuffer<'a>
{
    let debug_buffer = wm.make_window_buffer(
        (wm.get_width() - width) as isize,
        (wm.get_height() - height) as isize,
        width, height, 2, 1);

    debug_buffer.draw_borders();

    debug_buffer
}
