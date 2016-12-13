#![allow(dead_code)]

#[macro_use]
extern crate itertools;
extern crate rand;
extern crate num;
extern crate rustty;

#[macro_use]
mod debug;
mod frontends;
mod ecs;
mod math;
mod game;
mod util;
mod direction;
mod grid;
mod behaviour;
mod search;
mod perlin;

use frontends::ansi;

const GAME_WINDOW_WIDTH: usize = 80;
const GAME_WINDOW_HEIGHT: usize = 30;

const ANSI_GAME_WINDOW_X: usize = 1;
const ANSI_GAME_WINDOW_Y: usize = 1;

const DEBUG_WINDOW_WIDTH: usize = 80;
const DEBUG_WINDOW_HEIGHT: usize = 10;

const DEBUG_WINDOW_RIGHT_X: usize = 0;
const DEBUG_WINDOW_BOTTOM_Y: usize = 0;

const DEBUG_WINDOW_BORDER_X: usize = 2;
const DEBUG_WINDOW_BORDER_Y: usize = 1;

fn main() {
    game().expect("Game ended unexpectedly");
    println!("Bye.");
}

fn game() -> game::Result<()> {
    let mut window_allocator = ansi::WindowAllocator::new().unwrap();

    let input_source = window_allocator.make_input_source();

    let game_window = window_allocator.make_window(
        ANSI_GAME_WINDOW_X as isize,
        ANSI_GAME_WINDOW_Y as isize,
        GAME_WINDOW_WIDTH,
        GAME_WINDOW_HEIGHT,
        ansi::BufferType::Double);

    let mut game = game::GameCtx::new(game_window, input_source);

    let mut debug_buffer = window_allocator.make_window_buffer(
        (window_allocator.width() - DEBUG_WINDOW_WIDTH - DEBUG_WINDOW_BORDER_X - DEBUG_WINDOW_RIGHT_X) as isize,
        (window_allocator.height() - DEBUG_WINDOW_HEIGHT - DEBUG_WINDOW_BORDER_Y - DEBUG_WINDOW_BOTTOM_Y) as isize,
        DEBUG_WINDOW_WIDTH,
        DEBUG_WINDOW_HEIGHT,
        DEBUG_WINDOW_BORDER_X,
        DEBUG_WINDOW_BORDER_Y);

    debug_buffer.draw_borders();

    debug::init(&mut debug_buffer);

    game.run()
}
