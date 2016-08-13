use game::entity::Component::*;
use game::entity::ComponentType as Type;
use game::schedule::Schedule;
use game::io::terminal_player_actor;
use game::io::window_renderer;
use game::components::level::Level;
use game::update::UpdateSummary;

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
    update_queue: VecDeque<UpdateSummary>,
    reaction_queue: VecDeque<UpdateSummary>,
    rules: Vec<Box<Rule>>,
}

impl<'a> GameContext<'a> {
    pub fn new(input_source: InputSource<'a>, game_window: WindowRef<'a>) -> Self {
        GameContext {
            entities: EntityTable::new(),
            pc: None,
            input_source: input_source,
            game_window: game_window,
            update_queue: VecDeque::new(),
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
            if let Some(control) = terminal_player_actor::get_control(&self.input_source, entity_id, &self.entities) {
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

enum TurnError {
    Quit,
}

enum UpdateError {
    NothingApplied,
}

impl<'a> GameContext<'a> {
    fn apply_update(&mut self, action: UpdateSummary) -> Result<(), UpdateError> {

        self.update_queue.push_back(action);

        'outer: while let Some(action) = self.update_queue.pop_front() {
            self.reaction_queue.clear();

            for rule in &self.rules {
                let result = rule.check(&action, &self.entities);

                match result {
                    RuleResult::Instead(mut actions) => {
                        for action in actions.drain(..) {
                            self.update_queue.push_back(action);
                        }
                        continue 'outer;
                    },
                    RuleResult::After(mut actions) => {
                        for action in actions.drain(..) {
                            self.reaction_queue.push_back(action);
                        }
                    },
                }
            }

            action.commit(&mut self.entities);

            while let Some(action) = self.reaction_queue.pop_front() {
                self.update_queue.push_back(action);
            }
        }

        Ok(())
    }

    fn game_turn(&mut self) -> Result<(), TurnError> {
        let entity_id = self.pc_schedule_next();

        if self.entity_is_pc(entity_id) {
            self.render_pc_level();
        }

        loop {
            match self.get_control(entity_id) {
                Control::Quit => return Err(TurnError::Quit),
                Control::Update(update) => {
                    if let Err(_) = self.apply_update(update) {
                        continue;
                    } else {
                        break;
                    }
                },
            }
        }

        self.render_pc_level();

        Ok(())
    }

    pub fn game_loop(&mut self) {
        loop {
            if let Err(err) = self.game_turn() {
                match err {
                    TurnError::Quit => break,
                }
            }
        }
    }
}
