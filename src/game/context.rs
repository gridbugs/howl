use game::{
    TurnSchedule,
    UpdateSummary,
    MetaAction,
    Rule,
    RuleResult,
    RuleContext,
    EntityContext,
    EntityId,
    Level,
    LevelId,
    ComponentType,
    actions,
    EntityWrapper,
    EntityStore,
};
use game::components::Form;

use game::io::{
    terminal_player_actor,
    WindowKnowledgeRenderer,
};
use game::observer::DrawableObserver;

use schedule::Schedule;

use terminal::window_manager::{
    WindowRef,
    InputSource
};

use table::TableRef;

use std::cell;
use std::collections::VecDeque;
use std::thread;
use std::time::Duration;

pub struct GameContext<'a> {
    pub entities: EntityContext,
    pub pc: Option<EntityId>,
    pc_level_id: LevelId,

    // io
    input_source: InputSource<'a>,
    game_window: WindowRef<'a>,
    renderer: WindowKnowledgeRenderer<'a>,

    // rule application
    update_queue: Schedule<UpdateSummary>,
    reaction_queue: VecDeque<(u64, UpdateSummary)>,
    rules: Vec<Box<Rule>>,

    // observation
    observer: DrawableObserver,

    // time
    turn_count: u64,
}

impl<'a> GameContext<'a> {
    pub fn new(input_source: InputSource<'a>, game_window: WindowRef<'a>) -> Self {
        GameContext {
            entities: EntityContext::new(),
            pc: None,
            pc_level_id: 0,
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

    fn pc_level_id(&self) -> LevelId {
        self.pc_level_id
    }

    fn pc_level(&self) -> &Level {
        self.entities.level(self.pc_level_id()).unwrap()
    }

    fn pc_level_mut(&mut self) -> &mut Level {
        let id = self.pc_level_id();
        self.entities.level_mut(id).unwrap()
    }

    fn pc_schedule(&self) -> cell::RefMut<TurnSchedule> {
        self.pc_level().schedule.borrow_mut()
    }

    pub fn pc_schedule_next(&self) -> EntityId {
        self.pc_schedule().next().unwrap()
    }

    pub fn act(&mut self, entity_id: EntityId) -> MetaAction {
        loop {
            if let Some(meta_action) = terminal_player_actor::act(&self.input_source, entity_id, self.pc_level_id(), &self.entities) {
                return meta_action;
            }
        }
    }

    pub fn entity_is_pc(&self, entity: EntityId) -> bool {
        self.entities.get_from_level(entity, self.pc_level_id()).unwrap().has(ComponentType::PlayerActor)
    }

    pub fn observe_pc(&mut self) -> bool {
        self.observer.observe(self.pc.unwrap(), self.entities.level(self.pc_level_id).unwrap(), self.turn_count)
    }

    pub fn render_pc_knowledge(&self) {
        self.renderer.render(&self.pc_level(), self.pc.unwrap(), self.turn_count);
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

#[derive(Debug)]
enum TurnError {
    Quit,
}

#[derive(Debug)]
enum UpdateError {
    NothingApplied,
}

impl<'a> GameContext<'a> {
    fn apply_update(&mut self, update: UpdateSummary, render: bool) -> usize {
        let mut commit_count = 0;

        self.update_queue.insert(update, 0);

        'outer: while let Some((update, time_delta)) = self.update_queue.next() {

            if render && time_delta != 0 {
                if self.render() {
                    thread::sleep(Duration::from_millis(time_delta));
                }
            }

            {
                let rule_context = RuleContext::new(&update, self.entities.level(self.pc_level_id).unwrap(), &self.entities);

                self.reaction_queue.clear();
                for rule in &self.rules {
                    let result = rule.check(rule_context);

                    match result {
                        RuleResult::Instead(mut updates) => {
                            for (time, update) in updates.drain(..) {
                                self.update_queue.insert(update, time);
                            }
                            continue 'outer;
                        },
                        RuleResult::After(mut updates) => {
                            for (time, update) in updates.drain(..) {
                                self.reaction_queue.push_back((time, update));
                            }
                        },
                    }
                }
            }

            let action_time = update.action_time();

            self.turn_count += 1;

            let level_id = self.pc_level_id();
            update.commit(&mut self.entities, level_id, self.turn_count);
            commit_count += 1;

            while let Some((time, update)) = self.reaction_queue.pop_front() {
                self.update_queue.insert(update, action_time + time);
            }
        }

        if render {
            self.render();
        }

        commit_count
    }

    fn cloud_progress(&mut self) -> UpdateSummary {
        self.pc_level_mut().apply_perlin_change();
        self.pc_level().perlin_update()
    }

    fn transformation_progress(&mut self, id: EntityId, level_id: LevelId) -> Option<UpdateSummary> {
        let entity = self.entities.get_from_level(id, level_id).unwrap();
        if let Some(form) = entity.form() {
            if let Some(position) = entity.position() {
                let sh = self.entities.level(level_id).unwrap().spacial_hash();
                let sh_cell = sh.get((position.x, position.y)).unwrap();
                if sh_cell.has(ComponentType::Moon) {
                    if form == Form::Human {
                        return Some(actions::beast_transform_progress(entity, -1));
                    } else {
                        return Some(actions::human_transform_progress(entity, 1));
                    }
                } else {
                    if form == Form::Human {
                        return Some(actions::beast_transform_progress(entity, 1));
                    } else {
                        return Some(actions::human_transform_progress(entity, -1));
                    }
                }
            }
        }

        None
    }

    fn game_turn(&mut self) -> Result<(), TurnError> {
        self.turn_count += 1;
        let entity_id = self.pc_schedule_next();

        let level_id = self.pc_level_id();
        self.cloud_progress().commit(&mut self.entities, level_id, self.turn_count);

        if let Some(update) = self.transformation_progress(entity_id, level_id) {
            self.apply_update(update, false);
        }

        self.render();

        if !self.entity_is_pc(entity_id) {
            self.observer.observe(entity_id, self.entities.level(self.pc_level_id).unwrap(), self.turn_count);
        }

        loop {
            match self.act(entity_id) {
                MetaAction::Quit => return Err(TurnError::Quit),
                MetaAction::Update(update) => {
                    let commit_count = self.apply_update(update, true);
                    if commit_count == 0 && self.entity_is_pc(entity_id) {
                        continue;
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
