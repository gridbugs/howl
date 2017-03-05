use std::cell::RefCell;
use std::ops::DerefMut;

use game::*;
use game::data::*;
use ecs::*;
use util::{LeakyReserver, Schedule};
use coord::Coord;
use direction::Direction;

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
    Controls,
}

pub enum GameOverReason {
    PlayerDied,
}

pub enum ExitReason {
    GameOver(GameOverReason),
    Pause,
    Quit,
    BetweenLevels,
}

pub enum BetwenLevelsResolution {
    Pause,
    Start,
}

pub enum BetweenLevelsSelection {
    NextDelivery,
    Shop,
    Garage,
}

pub struct GameCtx<Renderer: KnowledgeRenderer, Input: InputSource> {
    renderer: RefCell<Renderer>,
    input_source: Input,
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
    turn_id: u64,
    action_id: ActionId,
    between_levels: bool,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            levels: LevelTable::new(),
            global_ids: None,
            entity_ids: EntityIdReserver::new(),
            turn_id: 0,
            action_id: 0,
            between_levels: false,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerializableGameState {
    levels: SerializableLevelTable,
    global_ids: Option<GlobalIds>,
    entity_ids: SerializableEntityIdReserver,
    turn_id: u64,
    action_id: ActionId,
    between_levels: bool,
}

impl From<GameState> for SerializableGameState {
    fn from(game_state: GameState) -> Self {
        let GameState { levels, global_ids, entity_ids, turn_id, action_id, between_levels } = game_state;
        SerializableGameState {
            levels: SerializableLevelTable::from(levels),
            global_ids: global_ids,
            entity_ids: SerializableEntityIdReserver::from(entity_ids),
            turn_id: turn_id,
            action_id: action_id,
            between_levels: between_levels,
        }
    }
}

impl From<SerializableGameState> for GameState {
    fn from(game_state: SerializableGameState) -> Self {
        let SerializableGameState { levels, global_ids, entity_ids, turn_id, action_id, between_levels } = game_state;
        GameState {
            levels: LevelTable::from(levels),
            global_ids: global_ids,
            entity_ids: EntityIdReserver::from(entity_ids),
            turn_id: turn_id,
            action_id: action_id,
            between_levels: between_levels,
        }
    }
}

impl<Renderer: KnowledgeRenderer, Input: 'static + InputSource + Clone> GameCtx<Renderer, Input> {
    pub fn new(renderer: Renderer, input_source: Input, seed: usize, width: usize, height: usize) -> Self {
        GameCtx {
            renderer: RefCell::new(renderer),
            input_source: input_source.clone(),
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
        let mut current_menu_state = None;

        loop {

            self.renderer.borrow_mut().reset_buffers();

            let mut control_map = control_file::from_file(args.user_path.join(user_files::CONTROL)).unwrap_or_default();

            let mut menu = SelectMenu::new();

            if current_game_state.is_some() {
                menu.push(SelectMenuItem::new(MenuMessageType::Continue, MainMenuSelection::Continue));
            }

            menu.push(SelectMenuItem::new(MenuMessageType::NewGame, MainMenuSelection::NewGame));
            menu.push(SelectMenuItem::new(MenuMessageType::Controls, MainMenuSelection::Controls));

            if current_game_state.is_some() {
                menu.push(SelectMenuItem::new(MenuMessageType::SaveAndQuit, MainMenuSelection::SaveAndQuit));
            } else {
                menu.push(SelectMenuItem::new(MenuMessageType::Quit, MainMenuSelection::Quit));
            }

            let (item, menu_state) = {
                let mut renderer_borrow = self.renderer.borrow_mut();
                let renderer = renderer_borrow.deref_mut();
                let menu_op = SelectMenuOperation::new(
                    renderer,
                    &mut self.input_source,
                    Some(MessageType::Title),
                    &self.language,
                    menu,
                    current_menu_state);

                if current_game_state.is_some() {
                    menu_op.run_can_escape().unwrap_or((MainMenuSelection::Continue, SelectMenuState::new()))
                } else {
                    menu_op.run()
                }
            };

            current_menu_state = None;

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
                    self.intro_message();
                    self.welcome_message(&game_state);

                    game_state
                }
                MainMenuSelection::Continue => current_game_state.take().expect("Missing game state"),
                MainMenuSelection::Controls => {
                    self.configure_controls(&mut control_map);
                    control_file::to_file(args.user_path.join(user_files::CONTROL), &control_map);
                    current_menu_state = Some(menu_state);
                    continue;
                }
            };

            Self::install_control_map(&mut game_state, control_map);

            loop {
                match self.game_loop(&mut game_state)? {
                    ExitReason::Pause => {
                        current_game_state = Some(game_state);
                        break;
                    }
                    ExitReason::Quit => {
                        save_file::save(args.user_path.as_path(), game_state);
                        return Ok(());
                    }
                    ExitReason::GameOver(reason) => {
                        match reason {
                            GameOverReason::PlayerDied => {
                                self.death_message(&game_state);
                                self.input_source.next_input();
                            }
                        }
                        current_game_state = None;
                        save_file::delete(args.user_path.as_path());
                        break;
                    }
                    ExitReason::BetweenLevels => {
                        game_state.between_levels = true;
                        match self.between_levels_menu(&mut game_state) {
                            BetwenLevelsResolution::Pause => {
                                current_game_state = Some(game_state);
                                break;
                            }
                            BetwenLevelsResolution::Start => {
                                game_state.between_levels = false;
                            }
                        }
                    }
                }
            }
        }
    }

    fn between_levels_menu(&mut self, game_state: &mut GameState) -> BetwenLevelsResolution {
        let mut current_menu_state = None;
        let mut renderer_borrow = self.renderer.borrow_mut();
        let mut renderer = renderer_borrow.deref_mut();

        loop {
            let mut menu = SelectMenu::new();

            menu.push(SelectMenuItem::new(MenuMessageType::Shop, BetweenLevelsSelection::Shop));
            menu.push(SelectMenuItem::new(MenuMessageType::Garage, BetweenLevelsSelection::Garage));
            menu.push(SelectMenuItem::new(MenuMessageType::NextDelivery, BetweenLevelsSelection::NextDelivery));

            let maybe_selection = SelectMenuOperation::new(
                renderer,
                &mut self.input_source,
                Some(MessageType::Title),
                &self.language,
                menu,
                current_menu_state).run_can_escape();

            if let Some((selection, menu_state)) = maybe_selection {
                current_menu_state = Some(menu_state.clone());
                match selection {
                    BetweenLevelsSelection::NextDelivery => return BetwenLevelsResolution::Start,
                    BetweenLevelsSelection::Shop => {
                    }
                    BetweenLevelsSelection::Garage => {
                    }
                }
            } else {
                return BetwenLevelsResolution::Pause;
            }
        }
    }

    fn install_control_map(game_state: &mut GameState, control_map: ControlMap) {
        let GlobalIds { pc_id, level_id } = game_state.global_ids.expect("Uninitialised game state");

        let level = game_state.levels.level_mut(level_id);
        level.ecs.insert_control_map(pc_id, control_map);
    }

    fn game_loop(&mut self, game_state: &mut GameState) -> GameResult<ExitReason> {

        if game_state.between_levels {
            return Ok(ExitReason::BetweenLevels);
        }

        loop {

            let GlobalIds { pc_id, level_id } = game_state.global_ids.expect("Uninitialised game state");

            game_state.turn_id += 1;

            let resolution = {
                let level = game_state.levels.level_mut(level_id);
                if let Some(turn_event) = level.turn_schedule.next() {

                    TurnEnv {
                        turn_id: game_state.turn_id,
                        action_id: &mut game_state.action_id,
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
                TurnResolution::Exit(reason, entity_id) => {
                    let level = game_state.levels.level_mut(level_id);
                    let old_ticket = level.ecs.schedule_ticket(entity_id).expect("Expected schedule_ticket component");
                    let new_ticket = level.turn_schedule.insert_with_ticket(entity_id, 0, old_ticket);
                    level.ecs.insert_schedule_ticket(entity_id, new_ticket);
                    return Ok(reason);
                }
                TurnResolution::Schedule(entity_id, delay) => {
                    let level = game_state.levels.level_mut(level_id);
                    let ticket = level.turn_schedule.insert(entity_id, delay);
                    level.ecs.insert_schedule_ticket(entity_id, ticket);
                }
                TurnResolution::LevelSwitch { entity_id, exit_id, level_switch } => {
                    self.switch_level(entity_id, exit_id, level_switch, game_state);
                    return Ok(ExitReason::BetweenLevels);
                }
                TurnResolution::GameOver(reason) => {
                    return Ok(ExitReason::GameOver(reason));
                }
            }
        }
    }

    fn welcome_message(&self, game_state: &GameState) {
        self.add_message(game_state, MessageType::Welcome);
    }

    fn death_message(&self, game_state: &GameState) {
        let GlobalIds { pc_id, level_id } = game_state.global_ids.expect("Unitialised game state");
        self.add_message(game_state, MessageType::YouDied);
        let entity = game_state.levels.level(level_id).ecs.entity(pc_id);
        self.renderer.borrow_mut().update_and_publish_all_windows_for_entity_with_overlay(
            game_state.action_id,
            level_id,
            entity,
            &self.language,
            &RenderOverlay::Death);
    }

    fn add_message(&self, game_state: &GameState, message: MessageType) {
        let GlobalIds { pc_id, level_id } = game_state.global_ids.expect("Unitialised game state");

        let ref ecs = game_state.levels.level(level_id).ecs;
        ecs.message_log_borrow_mut(pc_id).expect("Expected message log component").add(message);
    }

    fn configure_controls(&mut self, control_map: &mut ControlMap) {

        let mut renderer_borrow = self.renderer.borrow_mut();
        let mut renderer = renderer_borrow.deref_mut();
        let mut current_menu_state = None;

        loop {
            let mut menu = SelectMenu::new();
            let descriptions = control_map.descriptions();

            for (control, maybe_input) in descriptions.iter() {
                let message = if let Some(input) = maybe_input {
                    MenuMessageType::Control(input, control)
                } else {
                    MenuMessageType::UnboundControl(control)
                };

                menu.push(SelectMenuItem::new(message, control));
            }

            if let Some((control_to_change, menu_state)) = SelectMenuOperation::new(
                renderer,
                &mut self.input_source,
                None,
                &self.language,
                menu,
                current_menu_state).run_can_escape() {

                current_menu_state = Some(menu_state.clone());
                let mut menu = SelectMenu::new();

                for (control, maybe_input) in descriptions.iter() {
                    let message = if control == control_to_change {
                        MenuMessageType::ControlBinding(control)
                    } else {
                        if let Some(input) = maybe_input {
                            MenuMessageType::Control(input, control)
                        } else {
                            MenuMessageType::UnboundControl(control)
                        }
                    };

                    menu.push(SelectMenuItem::new(message, control));
                }
                SelectMenuOperation::new(
                    renderer,
                    &mut self.input_source,
                    None,
                    &self.language,
                    menu,
                    Some(menu_state)).publish();

                if let Some(input) = self.input_source.next_input() {
                    ControlSpec::from(&*control_map).get(control_to_change).map(|input| {
                        control_map.remove(input);
                    });

                    control_map.insert(input, control_to_change);
                }
            } else {
                break;
            }
        }
    }

    fn intro_message(&mut self) {

        let mut message = Message::new();

        self.language.translate(MessageType::Intro, &mut message);
        message.push(MessagePart::Newline);
        message.push(MessagePart::Newline);
        self.language.translate(MessageType::PressAnyKey, &mut message);

        display_message_scrolling(self.renderer.borrow_mut().deref_mut(), &mut self.input_source, &message, true);
    }

    fn init_demo(&mut self, game_state: &mut GameState) {

        let pc_id = game_state.entity_ids.new_id();

        let mut action = EcsAction::new();
        prototypes::pc(action.entity_mut(pc_id), Coord::new(0, 0));

        let pistol_id = game_state.entity_ids.new_id();
        prototypes::pistol(action.entity_mut(pistol_id));

        action.weapon_slots_mut(pc_id).expect("Missing component weapon_slots")
            .insert(Direction::East, pistol_id);

        // throw away connections in the first level a they would have nothing to connect to anyway
        let (level, _) = Level::new_with_entity(TerrainType::DemoA,
                                                pc_id,
                                                &mut action,
                                                &game_state.entity_ids,
                                                &self.rng,
                                                game_state.action_id,
                                                None);

        game_state.action_id += 1;

        let level_id = game_state.levels.add_level(level);

        game_state.global_ids = Some(GlobalIds {
            pc_id: pc_id,
            level_id: level_id,
        });
    }

    fn switch_level(&mut self, entity_id: EntityId, exit_id: EntityId, level_switch: LevelSwitch, game_state: &mut GameState) {
        let global_ids = game_state.global_ids.as_mut().expect("Unitialised game state");

        let mut entity_insert = game_state.levels
            .level_mut(global_ids.level_id)
            .remove_entity(entity_id, game_state.action_id);

        game_state.action_id += 1;

        if entity_id == global_ids.pc_id {
            let level = game_state.levels.level(global_ids.level_id);
            self.pc_observe_from_action(&mut entity_insert, entity_id, global_ids.level_id, level, game_state.action_id);
        }

        let new_level_id = match level_switch {
            LevelSwitch::NewLevel(terrain_type) => {

                let ( level, connections ) = {

                    let current_level = game_state.levels.level(global_ids.level_id);

                    // create link to level exit
                    let parent_ctx = ParentLevelCtx {
                        level: current_level,
                        level_id: global_ids.level_id,
                        exit_id: exit_id,
                    };

                    // create the new level
                    Level::new_with_entity(terrain_type,
                                           entity_id,
                                           &mut entity_insert,
                                           &game_state.entity_ids,
                                           &self.rng,
                                           game_state.action_id,
                                           Some(parent_ctx))
                };

                game_state.action_id += 1;

                // add the new level to the level table
                let new_level_id = game_state.levels.add_level(level);

                // connect the current level to the new level
                game_state.levels.level_mut(global_ids.level_id).connect(new_level_id, &connections);

                new_level_id
            }
            LevelSwitch::ExistingLevel(exit) => {

                game_state.levels.level_mut(exit.level_id)
                    .insert_entity_at_exit_and_commit(&mut entity_insert, entity_id,
                                                      exit.exit_id, game_state.action_id);

                game_state.action_id += 1;

                exit.level_id
            }
        };

        // update the current level
        global_ids.level_id = new_level_id;
    }

    fn pc_observe_from_action(&self, action: &mut EcsAction, entity_id: EntityId,
                              level_id: LevelId, level: &Level, action_id: ActionId) {

        let position = action.position(entity_id).expect("Missing component position");
        let vision_distance = action.vision_distance(entity_id).expect("Missing component vision_distance");
        let knowledge = action.drawable_knowledge_mut(entity_id).expect("Missing component drawable_knowledge");
        let level_knowledge = knowledge.level_mut_or_insert_size(level_id,
                                                                 level.spatial_hash.width(),
                                                                 level.spatial_hash.height());

        let action_env = ActionEnv::new(&level.ecs, action_id);

        self.pc_observer.observe(position, &level.spatial_hash, vision_distance, level_knowledge, action_env);
    }
}
