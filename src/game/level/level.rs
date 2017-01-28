use ecs::*;
use spatial_hash::*;
use util::Schedule;

pub struct Level {
    pub ecs: EcsCtx,
    pub spatial_hash: SpatialHashTable,
    pub turn_schedule: Schedule<EntityId>,
}

impl Level {
    pub fn new(width: usize, height: usize) -> Self {
        Level {
            ecs: EcsCtx::new(),
            spatial_hash: SpatialHashTable::new(width, height),
            turn_schedule: Schedule::new(),
        }
    }
}
