use ecs::message::Message;
use ecs::message::FieldType;
use ecs::message::Field;
use ecs::update::Update;
use ecs::entity::Component::*;
use ecs::entity::ComponentType as Type;
use ecs::systems::schedule::Schedule;
use ecs::systems::terminal_player_actor;
use ecs::systems::window_renderer;
use ecs::components::level::Level;
use ecs::systems::apply_update;

use game::rule::{Rule, RuleResult};

use std::cell;
use std::collections::VecDeque;

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

    // io
    input_source: InputSource<'a>,
    game_window: WindowRef<'a>,

    // rule application
    entities_copy: EntityTable,
    action_queue: VecDeque<Message>,
    reaction_queue: VecDeque<Message>,
    rules: Vec<Box<Rule>>,
}

impl<'a> GameContext<'a> {
    pub fn new(input_source: InputSource<'a>, game_window: WindowRef<'a>) -> Self {
        GameContext {
            entities: EntityTable::new(),
            pc: None,
            input_source: input_source,
            game_window: game_window,
            entities_copy: EntityTable::new(),
            action_queue: VecDeque::new(),
            reaction_queue: VecDeque::new(),
            rules: Vec::new(),
        }
    }

    pub fn rule<R: 'static + Rule>(&mut self, r: R) -> &mut Self {
        self.rules.push(Box::new(r));

        self
    }

    pub fn finalise(&mut self) {
        self.entities_copy = self.entities.clone();
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
            &Field::ActorTurn { actor: entity_id } => {
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

fn apply_action_on_entities(action: &Message, entities: &mut EntityTable) -> Update {
    if let Some(&Field::Update(ref update)) = action.get(FieldType::Update) {
        apply_update::apply_update(update, entities)
    } else {
        panic!("No Update field found in message")
    }
}

pub enum ActionResult {
    Done,
    Retry,
}

enum TurnResult {
    Continue,
    QuitGame,
}

impl<'a> GameContext<'a> {
    pub fn apply_action(&mut self, action: Message) -> ActionResult {

        self.action_queue.push_back(action);

        while let Some(action) = self.action_queue.pop_front() {
            let revert = apply_action_on_entities(&action, &mut self.entities);
            let mut cancelled = false;

            self.reaction_queue.clear();

            for rule in &self.rules {
                let result = rule.check(&action, &self.entities_copy, &self.entities);

                match result {
                    RuleResult::Instead(_) => {
                        cancelled = true;
                        break;
                    },
                    RuleResult::After(_) => {

                    },
                }
            }

            if cancelled {
                apply_update::apply_update(&revert, &mut self.entities);
            } else {
                apply_action_on_entities(&action, &mut self.entities_copy);
            }
        }

        ActionResult::Done
    }

    fn game_turn(&mut self) -> TurnResult {
        let turn = self.pc_schedule_next();

        if self.turn_entity_is_pc(&turn) {
            self.render_pc_level();
        }

        loop {
            let action = self.get_action(&turn);

            if action.has(FieldType::QuitGame) {
                return TurnResult::QuitGame;
            }

            if let ActionResult::Done = self.apply_action(action) {
                break;
            }
        }

        self.render_pc_level();

        TurnResult::Continue
    }

    pub fn game_loop(&mut self) {
        loop {
            if let TurnResult::QuitGame = self.game_turn() {
                break;
            }
        }
    }
}
