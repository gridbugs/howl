use ecs::message::Message;
use ecs::message_queue::MessageQueue;
use ecs::entity::{EntityId, EntityTable};
use ecs::system_queue::SystemQueue;
use ecs::systems::write_renderer::WriteRenderer;
use ecs::systems::window_renderer::WindowRenderer;
use ecs::systems::terminal_player_actor::TerminalPlayerActor;
use ecs::systems::apply_update::apply_update_message;
use ecs::systems::schedule::schedule_player_turn;

use std::io;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum SystemName {
    Renderer,
    PlayerActor,
    ApplyUpdate,
    ScheduleTurn,
}

#[derive(Debug)]
pub enum System<'a> {
    StdoutRenderer(WriteRenderer<io::Stdout>),
    TestRenderer(WriteRenderer<Vec<u8>>),
    WindowRenderer(WindowRenderer<'a>),
    TerminalPlayerActor(TerminalPlayerActor<'a>),
    ApplyUpdate,
    SchedulePlayerTurn(EntityId),
}

impl<'a> System<'a> {
    pub fn process_message(&mut self, message: &mut Message,
                           entities: &mut EntityTable,
                           systems: &SystemQueue,
                           message_queue: &mut MessageQueue)
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
                actor.process_message(message, message_queue, entities, systems);
            }
            System::ApplyUpdate => {
                apply_update_message(message, entities, systems);
            },
            System::SchedulePlayerTurn(entity) => {
                schedule_player_turn(entity, entities, message, message_queue);
            }
        }
    }
}
