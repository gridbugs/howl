use game::SpatialHashTable;
use ecs::EcsCtx;

pub struct Level {
    pub ecs: EcsCtx,
    pub spatial_hash: SpatialHashTable,
}

impl Level {
    pub fn new() -> Self {
        Level {
            ecs: EcsCtx::new(),
            spatial_hash: SpatialHashTable::new(),
        }
    }
}

impl Default for Level {
    fn default() -> Self {
        Self::new()
    }
}
