use ecs::*;
use game::*;
use game::data::*;
use game::actions;
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
    Null,
    Wait,
    Walk(EntityId, Direction),
    OpenDoor(EntityId),
    CloseDoor(EntityId),
    Close(EntityId, Direction),
    FireBullet(EntityId, Direction),
    ExplodeBullets(EntityId),
    BurstBullets(EntityId, Direction, usize),
    RealtimeAxisVelocityMove(EntityId, RealtimeAxisVelocity),
    Destroy(EntityId),
    MoveClouds(EntityId),
}

impl ActionArgs {
    pub fn to_action(self, action: &mut EcsAction, ecs: &EcsCtx, spatial_hash: &SpatialHashTable, entity_ids: &EntityIdReserver) -> Result<()> {
        match self {
            ActionArgs::Null => (),
            ActionArgs::Wait => {
                actions::wait(action)?;
            }
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
            ActionArgs::BurstBullets(entity_id, direction, count) => {
                actions::burst_bullets(action, entity_id, direction, count)?;
            }
            ActionArgs::ExplodeBullets(entity_id) => {
                actions::explode_bullets(action, ecs.entity(entity_id), entity_ids)?;
            }
            ActionArgs::RealtimeAxisVelocityMove(entity_id, velocity) => {
                actions::realtime_axis_velocity_move(action, ecs.entity(entity_id), velocity)?;
            }
            ActionArgs::Destroy(entity_id) => {
                actions::destroy(action, ecs.entity(entity_id))?;
            }
            ActionArgs::MoveClouds(entity_id) => {
                actions::move_clouds(action, ecs.entity(entity_id), spatial_hash)?;
            }
        }
        Ok(())
    }
}
