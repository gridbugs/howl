use rand::Rng;
use ecs::*;
use game::*;
use game::data::*;
use spatial_hash::*;
use direction::Direction;
use coord::Coord;

pub type ActionId = u64;

#[derive(Debug, Clone, Copy)]
pub enum External {
    Pause,
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
    Close(EntityId, Direction),
    FireBullet(EntityId, Coord),
    RealtimeVelocityMove(EntityId, RealtimeVelocity),
    Destroy(EntityId),
    TransformTerrorPillarTerrorFly(EntityId),
    TransformTree(EntityId),
    LevelSwitch {
        entity_id: EntityId,
        exit_id: EntityId,
        level_switch: LevelSwitch
    },
    TryLevelSwitch(EntityId),
    ProjectileCollision(ProjectileCollision),
    Damage(EntityId, usize),
    Die(EntityId),
    AcidAnimate,
}

impl ActionArgs {
    pub fn to_action<R: Rng>(self, action: &mut EcsAction, ecs: &EcsCtx, _spatial_hash: &SpatialHashTable, entity_ids: &EntityIdReserver, r: &mut R) {
        match self {
            ActionArgs::Null => (),
            ActionArgs::Walk(entity_id, direction) => {
                actions::walk(action, ecs.entity(entity_id), direction);
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
            ActionArgs::TransformTerrorPillarTerrorFly(entity_id) => {
                actions::transform_terror_pillar_terror_fly(action, ecs.entity(entity_id));
            }
            ActionArgs::TransformTree(entity_id) => {
                actions::transform_tree(action, ecs.entity(entity_id));
            }
            ActionArgs::LevelSwitch { entity_id, exit_id, level_switch }  => {
                actions::level_switch(action, entity_id, exit_id, level_switch);
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
            ActionArgs::AcidAnimate => {
                actions::acid_animate(action, ecs, r);
            }
        }
    }
}
