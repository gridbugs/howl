use game::EntityId;

// TODO
// Currently this just repeatedly schedules the player character 

#[derive(Debug, Clone)]
pub struct TurnSchedule {
    entity: Option<EntityId>,
}

impl TurnSchedule {
    pub fn new() -> Self {
        TurnSchedule { entity: None }
    }

    pub fn set_pc(&mut self, pc: EntityId) {
        self.entity = Some(pc);
    }

    pub fn next(&mut self) -> Option<EntityId> {
        self.entity
    }
}
