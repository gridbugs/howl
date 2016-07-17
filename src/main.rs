#![allow(dead_code)]

#[macro_use] extern crate itertools;
extern crate num;
extern crate rand;
extern crate rustty;

#[macro_use] mod ecs;
mod perlin;
mod renderer;
mod geometry;
mod grid;
mod colour;
mod game;
mod terminal;
mod allocator;
mod tests;

fn main() {
    println!("Hello, World!");
}
