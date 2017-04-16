use math::Direction;
use ecs_core::EntityId;

#[derive(Debug, Clone, Copy)]
pub enum ActionArgs {
    Null,
    Walk(EntityId, Direction),
}
