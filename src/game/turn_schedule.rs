use ecs::*;
use util::{Schedule, ScheduleTicket};
pub type TurnSchedule = Schedule<EntityId>;

pub trait TurnScheduleQueue {
    fn schedule_turn(&mut self, entity: EntityId, time: u64) -> ScheduleTicket;
}

impl TurnScheduleQueue for TurnSchedule {
    fn schedule_turn(&mut self, entity: EntityId, time: u64) -> ScheduleTicket {
        self.insert(entity, time)
    }
}
