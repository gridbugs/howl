use ecs::*;
use game::*;
use game::action::actions;
use direction::Direction;

#[derive(Debug, Clone, Copy)]
pub enum External {
    Quit,
}

#[derive(Debug, Clone, Copy)]
pub enum MetaAction {
    ActionArgs(ActionArgs),
    External(External),
}

#[derive(Debug, Clone, Copy)]
pub enum ActionArgs {
    Walk(EntityId, Direction),
    OpenDoor(EntityId),
    CloseDoor(EntityId),
    Close(EntityId, Direction),
    FireBullet(EntityId, Direction),
}

impl ActionArgs {
    pub fn to_action(self, action: &mut EcsAction, ecs: &EcsCtx, entity_ids: &EntityIdReserver) -> Result<()> {
        match self {
            ActionArgs::Walk(entity_id, direction) => {
                actions::walk(action, ecs.entity(entity_id), direction)?;
            }
            ActionArgs::OpenDoor(entity_id) => {
                actions::open_door(action, ecs.entity(entity_id))?;
            }
            ActionArgs::CloseDoor(entity_id) => {
                actions::close_door(action, ecs.entity(entity_id))?;
            }
            ActionArgs::Close(entity_id, direction) => {
                actions::close(action, entity_id, direction)?;
            }
            ActionArgs::FireBullet(entity_id, direction) => {
                actions::fire_bullet(action, ecs.entity(entity_id), direction, entity_ids)?;
            }
        }
        Ok(())
    }
}
