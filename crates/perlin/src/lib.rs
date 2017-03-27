extern crate rand;

#[macro_use]
extern crate serde_derive;
extern crate serde;

#[macro_use]
extern crate itertools;

extern crate grid;
extern crate math;

mod perlin;

pub use perlin::*;

#[cfg(test)]
mod tests;
