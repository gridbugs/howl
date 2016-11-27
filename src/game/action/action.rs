use direction::Direction;
use ecs::*;

#[derive(Debug)]
pub enum Control {
    Quit,
}

#[derive(Debug)]
pub enum MetaAction {
    ActionArgs(ActionArgs),
    Control(Control),
}

#[derive(Debug)]
pub enum ActionArgs {
    Walk(EntityId, Direction),
}
