use ecs::message::Message;
use ecs::entity_table::EntityTable;
use ecs::system_queue::SystemQueue;
use ecs::systems::write_renderer::WriteRenderer;

use std::io;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum SystemName {
    Renderer,
}

#[derive(Debug)]
pub enum System {
    StdoutRenderer(WriteRenderer<io::Stdout>),
    TestRenderer(WriteRenderer<Vec<u8>>),
}

impl System {
    pub fn process_message(&mut self, message: &mut Message,
                           entities: &mut EntityTable,
                           systems: &SystemQueue)
    {
        match *self {
            System::StdoutRenderer(ref mut renderer) => {
                renderer.process_message(message, entities, systems);
            },
            System::TestRenderer(ref mut renderer) => {
                renderer.process_message(message, entities, systems);
            },
        }
    }
}
