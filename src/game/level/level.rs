use game::SpatialHashTable;
use ecs::{EcsCtx, EntityId};
use util::Schedule;

pub struct Level {
    pub ecs: EcsCtx,
    pub spatial_hash: SpatialHashTable,
    pub turn_schedule: Schedule<EntityId>,
}

impl Level {
    pub fn new() -> Self {
        Level {
            ecs: EcsCtx::new(),
            spatial_hash: SpatialHashTable::new(),
            turn_schedule: Schedule::new(),
        }
    }
}

impl Default for Level {
    fn default() -> Self {
        Self::new()
    }
}
