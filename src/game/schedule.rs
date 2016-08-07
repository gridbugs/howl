use game::entity::EntityId;

// TODO
// Currently this just repeatedly schedules the player character 

#[derive(Debug, Clone)]
pub struct Schedule {
    entity: Option<EntityId>,
}

impl Schedule {
    pub fn new() -> Schedule {
        Schedule { entity: None }
    }

    pub fn set_pc(&mut self, pc: EntityId) {
        self.entity = Some(pc);
    }

    pub fn next(&mut self) -> Option<EntityId> {
        self.entity
    }
}
