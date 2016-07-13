#![allow(dead_code)]

#[macro_use] extern crate itertools;
extern crate num;
extern crate rand;

mod perlin;
mod renderer;
mod geometry;
mod grid;
mod colour;
#[macro_use] mod ecs;

use ecs::ecs_context::EcsContext;

fn main() {
    let a = EcsContext::new();
    println!("{:?}", a);
}
