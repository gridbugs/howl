use ecs::*;
use game::*;
use game::data::*;
use spatial_hash::*;
use direction::Direction;
use coord::Coord;

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
    Walk(EntityId, Direction),
    OpenDoor(EntityId),
    CloseDoor(EntityId),
    Close(EntityId, Direction),
    FireBullet(EntityId, Coord),
    RealtimeVelocityMove(EntityId, RealtimeVelocity),
    Destroy(EntityId),
    MoveClouds(EntityId),
    TransformTerrorPillarTerrorFly(EntityId),
    TransformTree(EntityId),
    LevelSwitch(LevelSwitch),
    ProjectileCollision(ProjectileCollision),
    Damage(EntityId, usize),
}

impl ActionArgs {
    pub fn to_action(self, action: &mut EcsAction, ecs: &EcsCtx, spatial_hash: &SpatialHashTable, entity_ids: &EntityIdReserver) -> Result<()> {
        match self {
            ActionArgs::Null => (),
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
            ActionArgs::FireBullet(entity_id, delta) => {
                actions::fire_bullet(action, ecs.entity(entity_id), delta, entity_ids)?;
            }
            ActionArgs::RealtimeVelocityMove(entity_id, velocity) => {
                actions::realtime_velocity_move(action, ecs.entity(entity_id), velocity)?;
            }
            ActionArgs::Destroy(entity_id) => {
                actions::destroy(action, ecs.entity(entity_id))?;
            }
            ActionArgs::MoveClouds(entity_id) => {
                actions::move_clouds(action, entity_id, ecs, spatial_hash)?;
            }
            ActionArgs::TransformTerrorPillarTerrorFly(entity_id) => {
                actions::transform_terror_pillar_terror_fly(action, ecs.entity(entity_id))?;
            }
            ActionArgs::TransformTree(entity_id) => {
                actions::transform_tree(action, ecs.entity(entity_id))?;
            }
            ActionArgs::LevelSwitch(level_switch) => {
                actions::level_switch(action, level_switch)?;
            }
            ActionArgs::ProjectileCollision(projectile_collision) => {
                actions::projectile_collision(action, projectile_collision)?;
            }
            ActionArgs::Damage(entity_id, amount) => {
                actions::damage(action, ecs.entity(entity_id), amount)?;
            }
        }
        Ok(())
    }
}
