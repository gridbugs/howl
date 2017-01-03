#![allow(dead_code)]

#[macro_use]
extern crate itertools;
extern crate rand;
extern crate num;

#[cfg(unix)]
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
mod coord;

fn main() {
    game().expect("Game ended unexpectedly");
    println!("Bye.");
}

fn game() -> game::Result<()> {

#[cfg(unix)]
    game::frontends::ansi::launch()?;

    Ok(())
}
