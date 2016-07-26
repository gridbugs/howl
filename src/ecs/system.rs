use ecs::message::Message;
use ecs::entity::EntityTable;
use ecs::system_queue::SystemQueue;
use ecs::systems::write_renderer::WriteRenderer;
use ecs::systems::window_renderer::WindowRenderer;
use ecs::systems::terminal_player_actor::TerminalPlayerActor;
use ecs::systems::apply_update::apply_update_message;

use std::io;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum SystemName {
    Renderer,
    PlayerActor,
    ApplyUpdate,
}

#[derive(Debug)]
pub enum System<'a> {
    StdoutRenderer(WriteRenderer<io::Stdout>),
    TestRenderer(WriteRenderer<Vec<u8>>),
    WindowRenderer(WindowRenderer<'a>),
    TerminalPlayerActor(TerminalPlayerActor<'a>),
    ApplyUpdate,
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
            System::TerminalPlayerActor(ref mut actor) => {
                actor.process_message(message, entities, systems);
            }
            System::ApplyUpdate => {
                apply_update_message(message, entities, systems);
            }
        }
    }
}
