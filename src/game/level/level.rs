use game::*;
use ecs::*;
use spatial_hash::*;

pub struct Level {
    pub ecs: EcsCtx,
    pub spatial_hash: SpatialHashTable,
    pub turn_schedule: TurnSchedule,
}
