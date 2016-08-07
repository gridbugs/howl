use game::entity::Component::*;
use game::entity::ComponentType as Type;
use game::schedule::Schedule;
use game::io::terminal_player_actor;
use game::io::window_renderer;
use game::components::level::Level;
use game::update::Action;

use game::control::Control;
use game::rule::{Rule, RuleResult};

use std::cell;
use std::collections::VecDeque;

use game::entity::{
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
    action_queue: VecDeque<Action>,
    reaction_queue: VecDeque<Action>,
    rules: Vec<Box<Rule>>,
}

impl<'a> GameContext<'a> {
    pub fn new(input_source: InputSource<'a>, game_window: WindowRef<'a>) -> Self {
        GameContext {
            entities: EntityTable::new(),
            pc: None,
            input_source: input_source,
            game_window: game_window,
            action_queue: VecDeque::new(),
            reaction_queue: VecDeque::new(),
            rules: Vec::new(),
        }
    }

    pub fn rule<R: 'static + Rule>(&mut self, r: R) -> &mut Self {
        self.rules.push(Box::new(r));

        self
    }

    pub fn entities(&self) -> &EntityTable {
        &self.entities
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

    pub fn pc_schedule_next(&self) -> EntityId {
        self.pc_schedule().next().unwrap()
    }

    pub fn get_control(&mut self, entity_id: EntityId) -> Control {
        loop {
            if let Some(control) = terminal_player_actor::get_control(&self.input_source, entity_id) {
                return control;
            }
        }
    }

    pub fn entity_is_pc(&self, entity: EntityId) -> bool {
        self.entities.get(entity).has(Type::PlayerActor)
    }

    fn render_level(&self, level: EntityId) {
        window_renderer::render(self.game_window, &self.entities, level);
    }

    pub fn render_pc_level(&self) {
        self.render_level(self.pc_level_id());
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
    pub fn apply_action(&mut self, action: Action) -> ActionResult {

        self.action_queue.push_back(action);

        while let Some(action) = self.action_queue.pop_front() {
            let summary = action.apply(&mut self.entities);

            let mut cancelled = false;

            self.reaction_queue.clear();

            for rule in &self.rules {
                let result = rule.check(&summary, &self.entities);

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
                summary.to_revert_action().apply(&mut self.entities);
            }
        }

        ActionResult::Done
    }

    fn game_turn(&mut self) -> TurnResult {
        let entity_id = self.pc_schedule_next();

        if self.entity_is_pc(entity_id) {
            self.render_pc_level();
        }

        loop {
            match self.get_control(entity_id) {
                Control::Quit => return TurnResult::QuitGame,
                Control::Action(action) => {
                    if let ActionResult::Done = self.apply_action(action) {
                        break;
                    }
                },
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
