use std::fmt::Debug;

use ecs::message::Message;
use ecs::entity_table::EntityTable;
use ecs::system_queue::SystemQueue;

pub trait System : Debug {
    fn process_message(&mut self, message: &mut Message,
                       entities: &mut EntityTable,
                       systems: &SystemQueue);
}
