#![allow(dead_code)]

#[macro_use] extern crate itertools;
extern crate num;
extern crate rand;

#[macro_use] mod ecs;
mod perlin;
mod renderer;
mod geometry;
mod grid;
mod colour;
mod game;
mod tests;

fn main() {
    println!("Hello, World!");
}
