extern crate math;
extern crate ecs_core;

use math::Direction;
use ecs_core::EntityId;

#[derive(Debug, Clone, Copy)]
pub enum ActionArgs {
    Null,
    Walk(EntityId, Direction),
}

#[derive(Clone, Copy)]
pub struct Reaction {
    pub action: ActionArgs,
    pub delay: u64,
}

impl Reaction {
    pub fn new(action: ActionArgs, delay: u64) -> Self {
        Reaction {
            action: action,
            delay: delay,
        }
    }
}
