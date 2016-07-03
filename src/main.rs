#![allow(dead_code)]

#[macro_use] extern crate itertools;
extern crate num;
extern crate rand;

mod renderer;
mod geometry;
mod grid;
mod colour;
#[macro_use] mod ecs;

fn main() {
    println!("Hello, world!");
}
