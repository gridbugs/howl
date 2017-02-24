use rand::Rng;
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
    LevelSwitch(EntityId, LevelSwitch),
    TryLevelSwitch(EntityId),
    ProjectileCollision(ProjectileCollision),
    Damage(EntityId, usize),
    Die(EntityId),
}

impl ActionArgs {
    pub fn to_action<R: Rng>(self, action: &mut EcsAction, ecs: &EcsCtx, spatial_hash: &SpatialHashTable, entity_ids: &EntityIdReserver, r: &mut R) {
        match self {
            ActionArgs::Null => (),
            ActionArgs::Walk(entity_id, direction) => {
                actions::walk(action, ecs.entity(entity_id), direction);
            }
            ActionArgs::OpenDoor(entity_id) => {
                actions::open_door(action, ecs.entity(entity_id));
            }
            ActionArgs::CloseDoor(entity_id) => {
                actions::close_door(action, ecs.entity(entity_id));
            }
            ActionArgs::Close(entity_id, direction) => {
                actions::close(action, entity_id, direction);
            }
            ActionArgs::FireBullet(entity_id, delta) => {
                actions::fire_bullet(action, ecs.entity(entity_id), delta, entity_ids);
            }
            ActionArgs::RealtimeVelocityMove(entity_id, velocity) => {
                actions::realtime_velocity_move(action, ecs.entity(entity_id), velocity);
            }
            ActionArgs::Destroy(entity_id) => {
                actions::destroy(action, ecs.entity(entity_id));
            }
            ActionArgs::MoveClouds(entity_id) => {
                actions::move_clouds(action, entity_id, ecs, spatial_hash, r);
            }
            ActionArgs::TransformTerrorPillarTerrorFly(entity_id) => {
                actions::transform_terror_pillar_terror_fly(action, ecs.entity(entity_id));
            }
            ActionArgs::TransformTree(entity_id) => {
                actions::transform_tree(action, ecs.entity(entity_id));
            }
            ActionArgs::LevelSwitch(entity_id, level_switch) => {
                actions::level_switch(action, entity_id, level_switch);
            }
            ActionArgs::ProjectileCollision(projectile_collision) => {
                actions::projectile_collision(action, projectile_collision);
            }
            ActionArgs::Damage(entity_id, amount) => {
                actions::damage(action, ecs.entity(entity_id), amount);
            }
            ActionArgs::Die(entity_id) => {
                actions::die(action, ecs.entity(entity_id));
            }
            ActionArgs::TryLevelSwitch(entity_id) => {
                actions::try_level_switch(action, entity_id);
            }
        }
    }
}
