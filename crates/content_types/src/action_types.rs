use math::{Direction, Coord};
use ecs_core::EntityId;
use realtime_velocity::RealtimeVelocity;
use projectile_collision::ProjectileCollision;
use level_switch::LevelSwitch;
use bullet_type::BulletType;
use steer_direction::SteerDirection;
use change_speed::ChangeSpeed;

#[derive(Clone, Copy)]
pub struct Reaction {
    pub action: ActionArgs,
    pub delay: u64,
}

impl Reaction {
    pub fn new(action: ActionArgs, delay: u64) -> Self {
        Reaction {
            action: action,
            delay: delay,
        }
    }
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
    TakeLetter(EntityId, EntityId),
    Explode(EntityId),
    ExplodeSpawn(Coord),
    RepairTyre(EntityId, usize),
    RepairEngine(EntityId, usize),
    Consume(EntityId, EntityId),
}
