extern crate math;
extern crate grid;

#[macro_use]
extern crate serde_derive;
extern crate serde;

mod grid_search;
pub use grid_search::*;

#[cfg(test)]
mod tests;
