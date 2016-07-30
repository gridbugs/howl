use ecs::message::Message;
use ecs::message::FieldType;
use ecs::message::Field::*;
use ecs::entity::Component::*;
use ecs::entity::ComponentType as Type;
use ecs::systems::schedule::Schedule;
use ecs::systems::terminal_player_actor;
use ecs::systems::window_renderer;
use ecs::components::level::Level;

use std::cell;

use ecs::entity::{
    EntityTable,
    EntityId,
    Entity,
};

use terminal::window_manager::{
    WindowRef,
    InputSource
};

pub struct GameContext<'a> {
    pub entities: EntityTable,
    pub pc: Option<EntityId>,
    input_source: InputSource<'a>,
    game_window: WindowRef<'a>,
}

impl<'a> GameContext<'a> {
    pub fn new(input_source: InputSource<'a>, game_window: WindowRef<'a>) -> Self {
        GameContext {
            entities: EntityTable::new(),
            pc: None,
            input_source: input_source,
            game_window: game_window,
        }
    }

    fn pc_level_id(&self) -> EntityId {
        let pc = self.pc.unwrap();
        match self.entities.get(pc).get(Type::OnLevel).unwrap() {
            &OnLevel(level) => level,
            _ => unreachable!(),
        }
    }

    fn pc_level_entity(&self) -> &Entity {
        self.entities.get(self.pc_level_id())
    }

    fn pc_level(&self) -> &Level {
        match self.pc_level_entity().get(Type::LevelData).unwrap() {
            &LevelData(ref level) => level,
            _ => unreachable!()
        }
    }

    fn pc_schedule(&self) -> cell::RefMut<Schedule> {
        self.pc_level().schedule.borrow_mut()
    }

    pub fn pc_schedule_next(&self) -> Message {
        self.pc_schedule().next().unwrap()
    }

    pub fn get_action(&mut self, turn: &Message) -> Message {
        loop {
            if let Some(message) =
                terminal_player_actor::get_action(&self.input_source, &self.entities, turn)
            {
                return message;
            }
        }
    }

    fn turn_entity(&self, turn: &Message) -> &Entity {
        match turn.get(FieldType::ActorTurn).unwrap() {
            &ActorTurn { actor: entity_id } => {
                self.entities.get(entity_id)
            },
            _ => unreachable!()
        }
    }

    pub fn turn_entity_is_pc(&self, turn: &Message) -> bool {
        self.turn_entity(turn).has(Type::PlayerActor)
    }

    fn render_level(&self, level: EntityId) {
        window_renderer::render(self.game_window, &self.entities, level);
    }

    pub fn render_pc_level(&self) {
        self.render_level(self.pc_level_id());
    }
}
