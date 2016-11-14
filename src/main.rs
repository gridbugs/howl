#![allow(dead_code)]

#[macro_use]
extern crate itertools;
extern crate num;
extern crate rand;
extern crate rustty;

#[macro_use]
mod debug;
#[macro_use]
mod table;
#[macro_use]
mod game;
mod perlin;
mod tile;
mod geometry;
mod schedule;
mod grid;
mod colour;
mod terminal;
mod best;
mod clear;
mod object_pool;
mod vision;
mod reserver;
mod math;
mod search;
mod behaviour;
mod ecs;

use std::io;

use terminal::{Window, InputSource, WindowAllocator, BufferType, WindowBuffer};

const DEBUG_WINDOW_WIDTH: usize = 80;
const DEBUG_WINDOW_HEIGHT: usize = 10;

fn main() {
    window_session();
}

fn window_session() {
    let mut wa = WindowAllocator::new().unwrap();

    let input = wa.make_input_source();
    let window = wa.make_window(0, 0, 80, 30, BufferType::Double);

    // Initialise debug window
    let mut debug_buffer = make_debug_window(&wa, DEBUG_WINDOW_WIDTH, DEBUG_WINDOW_HEIGHT);
    debug::debug::init(&mut debug_buffer as &mut io::Write);

    game(input, window);
}

fn game<'a>(_input_source: InputSource, _game_window: Window<'a>) {
}

fn make_debug_window<'a>(wa: &'a WindowAllocator, width: usize, height: usize) -> WindowBuffer<'a> {
    let mut debug_buffer = wa.make_window_buffer((wa.width() - width) as isize,
                                                 (wa.height() - height) as isize,
                                                 width,
                                                 height,
                                                 2,
                                                 1);


    debug_buffer.draw_borders();

    debug_buffer
}
