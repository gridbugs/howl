use ecs::message::Message;
use ecs::entity_table::EntityTable;
use ecs::system_queue::SystemQueue;
use ecs::systems::write_renderer::WriteRenderer;
use ecs::systems::window_renderer::WindowRenderer;

use std::io;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum SystemName {
    Renderer,
}

#[derive(Debug)]
pub enum System<'a> {
    StdoutRenderer(WriteRenderer<io::Stdout>),
    TestRenderer(WriteRenderer<Vec<u8>>),
    WindowRenderer(WindowRenderer<'a>),
}

impl<'a> System<'a> {
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
            System::WindowRenderer(ref mut renderer) => {
                renderer.process_message(message, entities, systems);
            },
        }
    }
}
