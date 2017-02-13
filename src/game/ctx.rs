use std::cell::RefCell;
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

enum MainMenuSelection {
    NewGame,
    Quit,
    Continue(GameState),
}

pub struct GameCtx<Renderer: KnowledgeRenderer, Input: InputSource> {
    renderer: RefCell<Renderer>,
    input_source: Input,
    entity_ids: EntityIdReserver,
    turn_id: u64,
    action_id: u64,
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

#[derive(Clone, Copy)]
pub struct GlobalIds {
    pc_id: EntityId,
    level_id: LevelId,
}

struct GameState {
    levels: LevelTable,
    ids: Option<GlobalIds>,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            levels: LevelTable::new(),
            ids: None,
        }
    }
}

impl<Renderer: KnowledgeRenderer, Input: 'static + InputSource + Clone> GameCtx<Renderer, Input> {
    pub fn new(renderer: Renderer, input_source: Input, seed: usize, width: usize, height: usize) -> Self {
        let entity_ids = EntityIdReserver::new();
        GameCtx {
            renderer: RefCell::new(renderer),
            input_source: input_source.clone(),
            entity_ids: entity_ids,
            turn_id: 0,
            action_id: 0,
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

    pub fn run(&mut self, _args: Arguments) -> Result<()> {

        let mut current_game_state = None;

        loop {

            let control_map = ControlMap::default(); // TODO load this from a file

            let mut menu = Menu::new();

            if let Some(game_state) = current_game_state.take() {
                menu.push(MenuItem::new(MenuMessageType::Continue, MainMenuSelection::Continue(game_state)));
            }

            menu.push(MenuItem::new(MenuMessageType::NewGame, MainMenuSelection::NewGame));
            menu.push(MenuItem::new(MenuMessageType::Quit, MainMenuSelection::Quit));

            let item = menu_operation::run(
                self.renderer.borrow_mut().deref_mut(),
                &mut self.input_source,
                Some(MessageType::Title),
                &self.language,
                menu);

            let mut game_state = match item {
                MainMenuSelection::Quit => {
                    return Ok(());
                }
                MainMenuSelection::NewGame => {
                    let mut game_state = GameState::new();

                    self.init_demo(&mut game_state);
                    self.intro_message(&control_map);
                    self.welcome_message(&game_state);

                    game_state
                }
                MainMenuSelection::Continue(game_state) => game_state,
            };

            Self::install_control_map(&mut game_state, control_map);

            self.game_loop(&mut game_state)?;

            current_game_state = Some(game_state);
        }
    }

    fn install_control_map(game_state: &mut GameState, control_map: ControlMap) {
        let GlobalIds { pc_id, level_id } = game_state.ids.expect("Uninitialised game state");

        let level = game_state.levels.level_mut(level_id);
        level.ecs.insert_control_map(pc_id, control_map);
    }

    fn game_loop(&mut self, game_state: &mut GameState) -> Result<()> {
        loop {

            let GlobalIds { pc_id, level_id } = game_state.ids.expect("Uninitialised game state");

            self.turn_id += 1;

            let resolution = {
                let level = game_state.levels.level_mut(level_id);
                if let Some(turn_event) = level.turn_schedule.next() {

                    TurnEnv {
                        turn_id: self.turn_id,
                        action_id: &mut self.action_id,
                        level_id: level_id,
                        entity_id: turn_event.event,
                        pc_id: pc_id,
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
                TurnResolution::Quit(entity_id) => {
                    let level = game_state.levels.level_mut(level_id);
                    let ticket = level.turn_schedule.insert(entity_id, 0);
                    level.ecs.insert_schedule_ticket(entity_id, ticket);
                    return Ok(());
                }
                TurnResolution::Schedule(entity_id, delay) => {
                    let level = game_state.levels.level_mut(level_id);
                    let ticket = level.turn_schedule.insert(entity_id, delay);
                    level.ecs.insert_schedule_ticket(entity_id, ticket);
                }
                TurnResolution::LevelSwitch(level_switch) => {
                    self.switch_level(level_switch, game_state);
                }
            }
        }
    }

    fn welcome_message(&self, game_state: &GameState) {
        let GlobalIds { pc_id, level_id } = game_state.ids.expect("Unitialised game state");

        let ref ecs = game_state.levels.level(level_id).ecs;
        ecs.message_log_borrow_mut(pc_id).unwrap().add(MessageType::Welcome);
    }

    fn intro_message(&mut self, control_map: &ControlMap) {

        let mut message = Message::new();

        self.language.translate(MessageType::Intro, &mut message);
        message.push(MessagePart::Newline);
        message.push(MessagePart::Newline);
        self.language.translate(MessageType::PressAnyKey, &mut message);
        message.push(MessagePart::Newline);
        message.push(MessagePart::Newline);
        message.push(MessagePart::Newline);
        self.language.translate_controls(control_map, &mut message);

        display_message_scrolling(self.renderer.borrow_mut().deref_mut(), &mut self.input_source, &message, true);
    }

    fn init_demo(&mut self, game_state: &mut GameState) {

        let pc_id = self.entity_ids.new_id();

        let mut action = EcsAction::new();
        prototypes::pc(action.entity_mut(pc_id), Coord::new(0, 0));

        let level = Level::new_with_pc(TerrainType::DemoA,
                                       pc_id,
                                       &mut action,
                                       &self.entity_ids,
                                       &self.rng,
                                       self.action_id);

        self.action_id += 1;

        let level_id = game_state.levels.add_level(level);

        game_state.ids = Some(GlobalIds {
            pc_id: pc_id,
            level_id: level_id,
        });
    }

    fn switch_level(&mut self, level_switch: LevelSwitch, game_state: &mut GameState) {
        let ids = game_state.ids.as_mut().expect("Unitialised game state");

        let mut pc_insert = EcsAction::new();
        let mut pc_remove = EcsAction::new();

        {
            let current_level = game_state.levels.level_mut(ids.level_id);
            pc_remove.remove_entity_by_id(ids.pc_id, &current_level.ecs);
            current_level.commit_into(&mut pc_remove, &mut pc_insert, self.action_id);
        }

        self.action_id += 1;

        let new_level = Level::new_with_pc(level_switch.terrain_type,
                                           ids.pc_id,
                                           &mut pc_insert,
                                           &self.entity_ids,
                                           &self.rng,
                                           self.action_id);
        self.action_id += 1;

        ids.level_id = game_state.levels.add_level(new_level);
    }
}
