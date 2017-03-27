#[macro_use]
extern crate serde_derive;
extern crate serde;

mod behaviour;
pub use behaviour::*;

#[cfg(test)]
mod tests;
