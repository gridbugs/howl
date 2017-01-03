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
    if let Err(message) = game() {
        println!("Error: {}", message);
    }
}

fn game() -> game::ExternalResult<()> {

#[cfg(unix)]
    game::frontends::ansi::launch()?;

    Ok(())
}
