#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate ecs_core;
extern crate ecs_content;
extern crate math;

mod generated;
mod coord;
pub use self::generated::*;
