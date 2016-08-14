use game::Component::*;
use game::ComponentType as Type;
use game::{
    Schedule,
    UpdateSummary,
    MetaAction,
    Rule,
    RuleResult,
    GameEntity,
};

use game::vision;
use game::vision::{
    DefaultObserver,
    DefaultVisionReport,
};

use game::io::terminal_player_actor;
use game::io::window_renderer;
use game::components::Level;

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

    // observation
    observer: Box<DefaultObserver>,
    vision_report: DefaultVisionReport,
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
            observer: Box::new(vision::square),
            vision_report: DefaultVisionReport::new(),
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

    pub fn act(&mut self, entity_id: EntityId) -> MetaAction {
        loop {
            if let Some(meta_action) = terminal_player_actor::act(&self.input_source, entity_id, &self.entities) {
                return meta_action;
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
    fn apply_update(&mut self, update: UpdateSummary) -> Result<(), UpdateError> {

        let mut no_commits = true;

        self.update_queue.push_back(update);

        'outer: while let Some(update) = self.update_queue.pop_front() {
            self.reaction_queue.clear();

            for rule in &self.rules {
                let result = rule.check(&update, &self.entities);

                match result {
                    RuleResult::Instead(mut updates) => {
                        for u in updates.drain(..) {
                            self.update_queue.push_back(u);
                        }
                        continue 'outer;
                    },
                    RuleResult::After(mut updates) => {
                        for u in updates.drain(..) {
                            self.reaction_queue.push_back(u);
                        }
                    },
                }
            }

            no_commits = false;
            update.commit(&mut self.entities);

            while let Some(update) = self.reaction_queue.pop_front() {
                self.update_queue.push_back(update);
            }
        }

        if no_commits {
            Err(UpdateError::NothingApplied)
        } else {
            Ok(())
        }
    }

    fn observe(&mut self, entity_id: EntityId) {
        let entity = self.entities.get(entity_id);
        let eye = entity.position().unwrap();
        let level = self.entities.get(entity.on_level().unwrap());
        let sh = level.level_spacial_hash().unwrap();
        let vision_info = entity.vision_info().unwrap();
        self.vision_report.clear();
        self.observer.observe(eye, &sh.grid, vision_info, &mut self.vision_report);
    }

    fn game_turn(&mut self) -> Result<(), TurnError> {
        let entity_id = self.pc_schedule_next();

        self.observe(entity_id);
        self.render_pc_level();

        loop {
            match self.act(entity_id) {
                MetaAction::Quit => return Err(TurnError::Quit),
                MetaAction::Update(update) => {
                    if let Err(err) = self.apply_update(update) {
                        match err {
                            UpdateError::NothingApplied => {
                                if self.entity_is_pc(entity_id) {
                                    // the player can choose a new action
                                    continue;
                                } else {
                                    // Other actors skip their turn.
                                    // This is to prevent infinite loops in the
                                    // face of buggy ai.
                                    break;
                                }
                            },
                        }
                    } else {
                        break;
                    }
                },
            }
        }

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
