use direction::Direction;
use ecs::*;

#[derive(Debug)]
pub enum ActionArgs {
    Walk(EntityId, Direction),
}
