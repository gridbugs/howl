use ecs::*;
use game::*;
use game::action::actions;
use direction::Direction;

#[derive(Debug, Clone, Copy)]
pub enum Control {
    Quit,
}

#[derive(Debug, Clone, Copy)]
pub enum MetaAction {
    ActionArgs(ActionArgs),
    Control(Control),
}

#[derive(Debug, Clone, Copy)]
pub enum ActionArgs {
    Walk(EntityId, Direction),
}

impl ActionArgs {
    pub fn to_action(self, action: &mut EcsAction, ecs: &EcsCtx) -> Result<()> {
        match self {
            ActionArgs::Walk(entity_id, direction) => {
                actions::walk(action, ecs.entity(entity_id), direction)?;
            }
        }
        Ok(())
    }
}
