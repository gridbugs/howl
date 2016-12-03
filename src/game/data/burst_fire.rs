use ecs::EntityId;
use direction::Direction;

#[derive(Clone, Copy, Debug)]
pub struct BurstFire {
    pub entity_id: EntityId,
    pub direction: Direction,
    pub count: usize,
}

impl BurstFire {
    pub fn new(entity_id: EntityId, direction: Direction, count: usize) -> Self {
        BurstFire {
            entity_id: entity_id,
            direction: direction,
            count: count,
        }
    }
}
