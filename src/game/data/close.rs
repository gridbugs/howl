use ecs::*;
use direction::Direction;

#[derive(Serialize, Deserialize)]
pub struct Close {
    pub entity_id: EntityId,
    pub direction: Direction,
}

impl Close {
    pub fn new(entity_id: EntityId, direction: Direction) -> Self {
        Close {
            entity_id: entity_id,
            direction: direction,
        }
    }
}
