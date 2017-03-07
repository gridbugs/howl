use rand::Rng;
use ecs::*;
use game::*;
use game::data::*;
use spatial_hash::*;
use direction::Direction;

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
    RealtimeVelocityMove(EntityId, RealtimeVelocity),
    RealtimeVelocityStart(EntityId, RealtimeVelocity, usize),
    RealtimeVelocityStop(EntityId),
    Destroy(EntityId),
    LevelSwitch {
        entity_id: EntityId,
        exit_id: EntityId,
        level_switch: LevelSwitch
    },
    TryLevelSwitch(EntityId),
    ProjectileCollision(ProjectileCollision),
    Damage(EntityId, usize),
    ComplexDamage(EntityId, usize),
    Die(EntityId),
    AcidAnimate,
    Physics,
    Steer(EntityId, SteerDirection),
    RemoveSteer(EntityId),
    ChangeSpeed(EntityId, ChangeSpeed),
    BecomeBloodstain(EntityId),
    FireGun {
        gun_id: EntityId,
        shooter_id: EntityId,
        direction: Direction,
    },
    FireBurst {
        gun_id: EntityId,
        shooter_id: EntityId,
        direction: Direction,
        remaining: usize,
        speed: f64,
        period: u64,
        spread: usize,
        range: usize,
        bullet_type: BulletType,
    },
    Bump(EntityId, EntityId),
    AcidDamage(EntityId),
}

impl ActionArgs {
    pub fn to_action<R: Rng>(self, action: &mut EcsAction, ecs: &EcsCtx, _spatial_hash: &SpatialHashTable, entity_ids: &EntityIdReserver, r: &mut R) {
        match self {
            ActionArgs::Null => (),
            ActionArgs::Walk(entity_id, direction) => {
                actions::walk(action, ecs.entity(entity_id), direction);
            }
            ActionArgs::RealtimeVelocityMove(entity_id, velocity) => {
                actions::realtime_velocity_move(action, ecs.entity(entity_id), velocity);
            }
            ActionArgs::RealtimeVelocityStart(entity_id, velocity, moves) => {
                actions::realtime_velocity_start(action, ecs.entity(entity_id), velocity, moves);
            }
            ActionArgs::RealtimeVelocityStop(entity_id) => {
                actions::realtime_velocity_stop(action, entity_id);
            }
            ActionArgs::Destroy(entity_id) => {
                actions::destroy(action, ecs.entity(entity_id));
            }
            ActionArgs::LevelSwitch { entity_id, exit_id, level_switch }  => {
                actions::level_switch(action, entity_id, exit_id, level_switch);
            }
            ActionArgs::ProjectileCollision(projectile_collision) => {
                actions::projectile_collision(action, projectile_collision, ecs);
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
            ActionArgs::Physics => {
                actions::physics(action);
            }
            ActionArgs::Steer(entity_id, direction) => {
                actions::steer(action, ecs.entity(entity_id), direction, r);
            }
            ActionArgs::RemoveSteer(entity_id) => {
                actions::remove_steer(action, entity_id);
            }
            ActionArgs::ChangeSpeed(entity_id, change) => {
                actions::change_speed(action, ecs.entity(entity_id), change);
            }
            ActionArgs::BecomeBloodstain(entity_id) => {
                actions::become_bloodstain(action, ecs.entity(entity_id), entity_ids);
            }
            ActionArgs::FireGun { gun_id, shooter_id, direction } => {
                actions::fire_gun(action, ecs.entity(gun_id), ecs.entity(shooter_id), direction, entity_ids, r);
            }
            ActionArgs::FireBurst { gun_id, shooter_id, direction, remaining, speed, period, spread, range, bullet_type } => {
                actions::fire_burst(action, ecs.entity(gun_id), ecs.entity(shooter_id), direction, remaining, speed, period, spread, range, bullet_type, entity_ids, r);
            }
            ActionArgs::ComplexDamage(entity_id, damage) => {
                actions::complex_damage(action, ecs.entity(entity_id), damage, r);
            }
            ActionArgs::Bump(victim_id, attacker_id) => {
                actions::bump(action, ecs.entity(victim_id), ecs.entity(attacker_id));
            }
            ActionArgs::AcidDamage(entity_id) => {
                actions::acid_damage(action, ecs.entity(entity_id), r);
            }
        }
    }
}
