use std::cell::RefCell;
use std::ops::Deref;
use std::ops::DerefMut;

use game::*;
use game::data::*;
use ecs::*;
use util::{LeakyReserver, Schedule};
use coord::Coord;

pub struct EntityIdReserver(RefCell<LeakyReserver<EntityId>>);

impl EntityIdReserver {
    pub fn new() -> Self {
        EntityIdReserver(RefCell::new(LeakyReserver::new()))
    }

    pub fn new_id(&self) -> EntityId {
        self.0.borrow_mut().reserve()
    }
}

pub struct GameCtx<Renderer: KnowledgeRenderer, Input: InputSource> {
    levels: LevelTable,
    renderer: RefCell<Renderer>,
    input_source: Input,
    entity_ids: EntityIdReserver,
    turn_id: u64,
    action_id: u64,
    level_id: LevelId,
    pc_id: EntityId,
    pc_observer: Shadowcast,
    behaviour_ctx: BehaviourCtx<Renderer>,
    rule_reactions: Vec<Reaction>,
    ecs_action: EcsAction,
    action_schedule: Schedule<ActionArgs>,
    width: usize,
    height: usize,
    rng: GameRng,
    language: Box<Language>,
}

impl<Renderer: KnowledgeRenderer, Input: 'static + InputSource + Clone> GameCtx<Renderer, Input> {
    pub fn new(renderer: Renderer, input_source: Input, seed: usize, width: usize, height: usize) -> Self {
        let entity_ids = EntityIdReserver::new();
        let pc_id = entity_ids.new_id();
        GameCtx {
            levels: LevelTable::new(),
            renderer: RefCell::new(renderer),
            input_source: input_source.clone(),
            entity_ids: entity_ids,
            turn_id: 0,
            action_id: 0,
            level_id: 0,
            pc_id: pc_id,
            pc_observer: Shadowcast::new(),
            behaviour_ctx: BehaviourCtx::new(input_source),
            rule_reactions: Vec::new(),
            ecs_action: EcsAction::new(),
            action_schedule: Schedule::new(),
            width: width,
            height: height,
            rng: GameRng::new(seed),
            language: Box::new(languages::English),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        self.init_demo();
        self.intro_message();
        self.welcome_message();
        self.game_loop()
    }

    fn game_loop(&mut self) -> Result<()> {
        loop {

            self.turn_id += 1;

            let resolution = {
                let level = self.levels.level_mut(self.level_id);
                if let Some(turn_event) = level.turn_schedule.next() {

                    TurnEnv {
                        turn_id: self.turn_id,
                        action_id: &mut self.action_id,
                        level_id: self.level_id,
                        entity_id: turn_event.event,
                        pc_id: self.pc_id,
                        renderer: &self.renderer,
                        ecs: &mut level.ecs,
                        spatial_hash: &mut level.spatial_hash,
                        behaviour_ctx: &self.behaviour_ctx,
                        rule_reactions: &mut self.rule_reactions,
                        ecs_action: &mut self.ecs_action,
                        action_schedule: &mut self.action_schedule,
                        turn_schedule: &mut level.turn_schedule,
                        pc_observer: &self.pc_observer,
                        entity_ids: &self.entity_ids,
                        rng: &self.rng,
                        language: &self.language,
                    }.turn()?

                } else {
                    return Err(Error::ScheduleEmpty);
                }
            };

            match resolution {
                TurnResolution::Quit => return Ok(()),
                TurnResolution::Schedule(entity_id, delay) => {
                    let level = self.levels.level_mut(self.level_id);
                    let ticket = level.turn_schedule.insert(entity_id, delay);
                    level.ecs.insert_schedule_ticket(entity_id, ticket);
                }
                TurnResolution::LevelSwitch(level_switch) => {
                    self.switch_level(level_switch);
                }
            }
        }
    }

    fn welcome_message(&self) {
        let ref ecs = self.levels.level(self.level_id).ecs;
        ecs.message_log_borrow_mut(self.pc_id).unwrap().add(MessageType::Welcome);
    }

    fn intro_message(&mut self) {
        let ref ecs = self.levels.level(self.level_id).ecs;
        let pc = ecs.entity(self.pc_id);
        let control_map_ref = pc.control_map_borrow().unwrap();
        let mut message = Message::new();

        self.language.translate(MessageType::Intro, &mut message);
        message.push(MessagePart::Newline);
        message.push(MessagePart::Newline);
        self.language.translate(MessageType::PressAnyKey, &mut message);
        message.push(MessagePart::Newline);
        message.push(MessagePart::Newline);
        message.push(MessagePart::Newline);
        self.language.translate_controls(control_map_ref.deref(), &mut message);

        display_message_scrolling(self.renderer.borrow_mut().deref_mut(), &mut self.input_source, &message, true);
    }

    fn init_demo(&mut self) {

        let mut action = EcsAction::new();
        prototypes::pc(action.entity_mut(self.pc_id), Coord::new(0, 0));

        let level = Level::new_with_pc(TerrainType::DemoA,
                                       self.pc_id,
                                       &mut action,
                                       &self.entity_ids,
                                       &self.rng,
                                       self.action_id);

        self.action_id += 1;

        self.level_id = self.levels.add_level(level);
    }

    fn switch_level(&mut self, level_switch: LevelSwitch) {

        let mut pc_insert = EcsAction::new();
        let mut pc_remove = EcsAction::new();

        {
            let current_level = self.levels.level_mut(self.level_id);
            pc_remove.remove_entity_by_id(self.pc_id, &current_level.ecs);
            current_level.commit_into(&mut pc_remove, &mut pc_insert, self.action_id);
        }

        self.action_id += 1;

        let new_level = Level::new_with_pc(level_switch.terrain_type,
                                           self.pc_id,
                                           &mut pc_insert,
                                           &self.entity_ids,
                                           &self.rng,
                                           self.action_id);
        self.action_id += 1;

        self.level_id = self.levels.add_level(new_level);
    }
}
