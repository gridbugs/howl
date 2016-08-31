use game::Component::*;
use game::ComponentType as Type;
use game::{
    TurnSchedule,
    UpdateSummary,
    MetaAction,
    Rule,
    RuleResult,
    RuleContext,
    EntityTable,
    EntityId,
    Entity,
};

use game::io::{
    terminal_player_actor,
    WindowKnowledgeRenderer,
};
use game::components::Level;
use game::observer::{
    DrawableObserver,
    Observer,
};

use schedule::Schedule;

use terminal::window_manager::{
    WindowRef,
    InputSource
};

use std::cell;
use std::collections::VecDeque;
use std::thread;
use std::time::Duration;

pub struct GameContext<'a> {
    pub entities: EntityTable,
    pub pc: Option<EntityId>,

    // io
    input_source: InputSource<'a>,
    game_window: WindowRef<'a>,
    renderer: WindowKnowledgeRenderer<'a>,

    // rule application
    update_queue: Schedule<UpdateSummary>,
    reaction_queue: VecDeque<UpdateSummary>,
    rules: Vec<Box<Rule>>,

    // observation
    observer: DrawableObserver,

    // time
    turn_count: u64,
}

impl<'a> GameContext<'a> {
    pub fn new(input_source: InputSource<'a>, game_window: WindowRef<'a>) -> Self {
        GameContext {
            entities: EntityTable::new(),
            pc: None,
            input_source: input_source,
            game_window: game_window,
            renderer: WindowKnowledgeRenderer::new(game_window),
            update_queue: Schedule::new(),
            reaction_queue: VecDeque::new(),
            rules: Vec::new(),
            observer: DrawableObserver::new(),
            turn_count: 0,
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

    fn pc_schedule(&self) -> cell::RefMut<TurnSchedule> {
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

    pub fn observe_pc(&mut self) -> bool {
        self.observer.observe(self.pc.unwrap(), &self.entities, self.turn_count)
    }

    pub fn render_pc_knowledge(&self) {
        self.renderer.render(&self.entities, self.pc.unwrap(), self.turn_count);
    }

    pub fn render(&mut self) -> bool {
        if self.observe_pc() {
            self.render_pc_knowledge();
            true
        } else {
            false
        }
    }
}

enum TurnError {
    Quit,
}

enum UpdateError {
    NothingApplied,
}

impl<'a> GameContext<'a> {
    fn rule_context<'b: 'a>(&'b self, update: &'b UpdateSummary) -> RuleContext<'b> {
        RuleContext::new(update, &self.entities)
    }

    fn apply_update(&mut self, update: UpdateSummary)
        -> Result<(), UpdateError>
    {
        let mut no_commits = true;

        self.update_queue.insert(update, 0);

        'outer: while let Some((update, time_delta)) = self.update_queue.next() {

            if time_delta != 0 {
                if self.render() {
                    thread::sleep(Duration::from_millis(time_delta));
                }
            }

            {
                let rule_context = RuleContext::new(&update, &self.entities);

                self.reaction_queue.clear();
                for rule in &self.rules {
                    let result = rule.check(rule_context);

                    match result {
                        RuleResult::Instead(mut updates) => {
                            for u in updates.drain(..) {
                                self.update_queue.insert(u, 0);
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
            }

            no_commits = false;

            let action_time = update.action_time();

            self.turn_count += 1;

            update.commit(&mut self.entities, self.turn_count);

            while let Some(update) = self.reaction_queue.pop_front() {
                self.update_queue.insert(update, action_time);
            }
        }

        if no_commits {
            Err(UpdateError::NothingApplied)
        } else {
            self.render();
            Ok(())
        }
    }

    fn game_turn(&mut self) -> Result<(), TurnError> {
        self.turn_count += 1;
        let entity_id = self.pc_schedule_next();

        self.render();

        if !self.entity_is_pc(entity_id) {
            self.observer.observe(entity_id, &self.entities, self.turn_count);
        }

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
