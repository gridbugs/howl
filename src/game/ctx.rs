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

#[derive(Serialize, Deserialize)]
pub struct SerializableEntityIdReserver(LeakyReserver<EntityId>);

impl From<EntityIdReserver> for SerializableEntityIdReserver {
    fn from(r: EntityIdReserver) -> Self {
        SerializableEntityIdReserver(r.0.into_inner())
    }
}

impl From<SerializableEntityIdReserver> for EntityIdReserver {
    fn from(r: SerializableEntityIdReserver) -> Self {
        EntityIdReserver(RefCell::new(r.0))
    }
}

enum MainMenuSelection {
    NewGame,
    Quit,
    Continue,
    SaveAndQuit,
}

pub struct GameCtx<Renderer: KnowledgeRenderer, Input: InputSource> {
    renderer: RefCell<Renderer>,
    input_source: Input,
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

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct GlobalIds {
    pc_id: EntityId,
    level_id: LevelId,
}

pub struct GameState {
    levels: LevelTable,
    global_ids: Option<GlobalIds>,
    entity_ids: EntityIdReserver,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            levels: LevelTable::new(),
            global_ids: None,
            entity_ids: EntityIdReserver::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerializableGameState {
    levels: SerializableLevelTable,
    global_ids: Option<GlobalIds>,
    entity_ids: SerializableEntityIdReserver,
}

impl From<GameState> for SerializableGameState {
    fn from(game_state: GameState) -> Self {
        let GameState { levels, global_ids, entity_ids } = game_state;
        SerializableGameState {
            levels: SerializableLevelTable::from(levels),
            global_ids: global_ids,
            entity_ids: SerializableEntityIdReserver::from(entity_ids),
        }
    }
}

impl From<SerializableGameState> for GameState {
    fn from(game_state: SerializableGameState) -> Self {
        let SerializableGameState { levels, global_ids, entity_ids } = game_state;
        GameState {
            levels: LevelTable::from(levels),
            global_ids: global_ids,
            entity_ids: EntityIdReserver::from(entity_ids),
        }
    }
}

impl<Renderer: KnowledgeRenderer, Input: 'static + InputSource + Clone> GameCtx<Renderer, Input> {
    pub fn new(renderer: Renderer, input_source: Input, seed: usize, width: usize, height: usize) -> Self {
        GameCtx {
            renderer: RefCell::new(renderer),
            input_source: input_source.clone(),
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

    pub fn run(&mut self, args: Arguments) -> GameResult<()> {

        let mut current_game_state = save_file::load(args.user_path.as_path());

        loop {

            let control_map = control_spec::from_file(args.user_path.join(user_files::CONTROL)).unwrap_or_default();

            let mut menu = Menu::new();

            if current_game_state.is_some() {
                menu.push(MenuItem::new(MenuMessageType::Continue, MainMenuSelection::Continue));
            }

            menu.push(MenuItem::new(MenuMessageType::NewGame, MainMenuSelection::NewGame));

            if current_game_state.is_some() {
                menu.push(MenuItem::new(MenuMessageType::SaveAndQuit, MainMenuSelection::SaveAndQuit));
            } else {
                menu.push(MenuItem::new(MenuMessageType::Quit, MainMenuSelection::Quit));
            }

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
                MainMenuSelection::SaveAndQuit => {
                    let game_state = current_game_state.take().expect("Missing game state");
                    save_file::save(args.user_path.as_path(), game_state);
                    return Ok(());
                }
                MainMenuSelection::NewGame => {
                    let mut game_state = GameState::new();

                    self.init_demo(&mut game_state);
                    self.intro_message(&control_map);
                    self.welcome_message(&game_state);

                    game_state
                }
                MainMenuSelection::Continue => current_game_state.take().expect("Missing game state"),
            };

            Self::install_control_map(&mut game_state, control_map);

            self.game_loop(&mut game_state)?;

            current_game_state = Some(game_state);
        }
    }

    fn install_control_map(game_state: &mut GameState, control_map: ControlMap) {
        let GlobalIds { pc_id, level_id } = game_state.global_ids.expect("Uninitialised game state");

        let level = game_state.levels.level_mut(level_id);
        level.ecs.insert_control_map(pc_id, control_map);
    }

    fn game_loop(&mut self, game_state: &mut GameState) -> GameResult<()> {
        loop {

            let GlobalIds { pc_id, level_id } = game_state.global_ids.expect("Uninitialised game state");

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
                        entity_ids: &game_state.entity_ids,
                        rng: &self.rng,
                        language: &self.language,
                    }.turn()?

                } else {
                    return Err(GameError::ScheduleEmpty);
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
        let GlobalIds { pc_id, level_id } = game_state.global_ids.expect("Unitialised game state");

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

        let pc_id = game_state.entity_ids.new_id();

        let mut action = EcsAction::new();
        prototypes::pc(action.entity_mut(pc_id), Coord::new(0, 0));

        let level = Level::new_with_pc(TerrainType::DemoA,
                                       pc_id,
                                       &mut action,
                                       &game_state.entity_ids,
                                       &self.rng,
                                       self.action_id);

        self.action_id += 1;

        let level_id = game_state.levels.add_level(level);

        game_state.global_ids = Some(GlobalIds {
            pc_id: pc_id,
            level_id: level_id,
        });
    }

    fn switch_level(&mut self, level_switch: LevelSwitch, game_state: &mut GameState) {
        let global_ids = game_state.global_ids.as_mut().expect("Unitialised game state");

        let mut pc_insert = EcsAction::new();
        let mut pc_remove = EcsAction::new();

        {
            let current_level = game_state.levels.level_mut(global_ids.level_id);
            pc_remove.remove_entity_by_id(global_ids.pc_id, &current_level.ecs);
            current_level.commit_into(&mut pc_remove, &mut pc_insert, self.action_id);
        }

        self.action_id += 1;

        let new_level = Level::new_with_pc(level_switch.terrain_type,
                                           global_ids.pc_id,
                                           &mut pc_insert,
                                           &game_state.entity_ids,
                                           &self.rng,
                                           self.action_id);
        self.action_id += 1;

        global_ids.level_id = game_state.levels.add_level(new_level);
    }
}
