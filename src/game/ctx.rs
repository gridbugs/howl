use std::cell::RefCell;
use std::ops::DerefMut;
use rand::Rng;

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
    Inventory,
}

pub enum ItemMenuSelection {
    Back,
    Remove,
}

enum BuyError {
    CantAfford,
    InventoryFull,
    NoEffect,
}

enum WeaponMenuError {
    InventoryFull,
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
    shop_id: EntityId,
}

pub struct GameState {
    levels: LevelTable,
    global_ids: Option<GlobalIds>,
    entity_ids: EntityIdReserver,
    turn_id: u64,
    action_id: ActionId,
    between_levels: bool,
    staging: EcsCtx,
    staged: Option<EntityId>,
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
            staging: EcsCtx::new(),
            staged: None,
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
    staging: SerializableEcsCtx,
    staged: Option<EntityId>,
}

impl From<GameState> for SerializableGameState {
    fn from(game_state: GameState) -> Self {
        let GameState { levels, global_ids, entity_ids, turn_id, action_id, between_levels, staging, staged } = game_state;
        SerializableGameState {
            levels: SerializableLevelTable::from(levels),
            global_ids: global_ids,
            entity_ids: SerializableEntityIdReserver::from(entity_ids),
            turn_id: turn_id,
            action_id: action_id,
            between_levels: between_levels,
            staging: SerializableEcsCtx::from(staging),
            staged: staged,
        }
    }
}

impl From<SerializableGameState> for GameState {
    fn from(game_state: SerializableGameState) -> Self {
        let SerializableGameState { levels, global_ids, entity_ids, turn_id, action_id, between_levels, staging, staged } = game_state;
        GameState {
            levels: LevelTable::from(levels),
            global_ids: global_ids,
            entity_ids: EntityIdReserver::from(entity_ids),
            turn_id: turn_id,
            action_id: action_id,
            between_levels: between_levels,
            staging: EcsCtx::from(staging),
            staged: staged,
        }
    }
}

#[derive(Clone, Copy)]
enum ShopItemType {
    Pistol,
    Shotgun,
    MachineGun,
    Railgun,
    EngineRepair,
    TyresRepair,
    Armour(usize),
    SpareTyre,
    EngineRepairKit,
}

const RANDOM_SHOP_ITEM_TYPES: [ShopItemType; 12] = [
    ShopItemType::Pistol,
    ShopItemType::Shotgun,
    ShopItemType::MachineGun,
    ShopItemType::Railgun,
    ShopItemType::EngineRepair,
    ShopItemType::TyresRepair,
    ShopItemType::Armour(1),
    ShopItemType::Armour(2),
    ShopItemType::Armour(3),
    ShopItemType::Armour(4),
    ShopItemType::SpareTyre,
    ShopItemType::EngineRepairKit,
];
const RANDOM_SHOP_ITEM_WEIGHTS: [usize; 12] = [10, 9, 8, 7, 8, 8, 8, 6, 4, 2, 6, 6];
const SHOP_MAX_ITEMS: usize = 12;
const SHOP_MIN_ITEMS: usize = 6;

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
                    current_menu_state,
                    None);

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
                self.renderer.borrow_mut().reset_buffers();
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
        let GlobalIds { pc_id, .. } = game_state.global_ids.expect("Uninitialised game state");
        let mut current_menu_state = None;

        loop {
            let mut menu = SelectMenu::new();

            menu.push(SelectMenuItem::new(MenuMessageType::Shop, BetweenLevelsSelection::Shop));
            menu.push(SelectMenuItem::new(MenuMessageType::Garage, BetweenLevelsSelection::Garage));
            menu.push(SelectMenuItem::new(MenuMessageType::Inventory, BetweenLevelsSelection::Inventory));
            menu.push(SelectMenuItem::new(MenuMessageType::NextDelivery, BetweenLevelsSelection::NextDelivery));

            let maybe_selection = SelectMenuOperation::new(
                self.renderer.borrow_mut().deref_mut(),
                &mut self.input_source,
                Some(MessageType::SurvivorCamp),
                &self.language,
                menu,
                current_menu_state,
                Some(game_state.staging.entity(pc_id))).run_can_escape();

            if let Some((selection, menu_state)) = maybe_selection {
                current_menu_state = Some(menu_state.clone());
                match selection {
                    BetweenLevelsSelection::NextDelivery => {
                        if self.next_level_menu(game_state) {
                            return BetwenLevelsResolution::Start;
                        }
                    }
                    BetweenLevelsSelection::Shop => {
                        self.shop_menu(game_state);
                    }
                    BetweenLevelsSelection::Garage => {
                        self.garage_menu(game_state);
                    }
                    BetweenLevelsSelection::Inventory => {
                        self.inventory_menu(game_state);
                    }
                }
            } else {
                return BetwenLevelsResolution::Pause;
            }
        }
    }

    fn shop_menu(&mut self, game_state: &mut GameState) {
        let GlobalIds { shop_id, pc_id, .. } = game_state.global_ids.expect("Uninitialised game state");
        let mut buy_result = Ok(());
        loop {
            let mut menu = SelectMenu::new();

            for entity_id in game_state.staging.borrow_inventory(shop_id).expect("Missing component inventory").iter() {
                let item = game_state.staging.entity(entity_id);
                let name = item.copy_name().expect("Missing component name");
                let price = item.copy_price().expect("Missing component price");

                let menu_message = MenuMessageType::ShopItem(name, price);
                menu.push(SelectMenuItem::new(menu_message, entity_id));
            }

            let bank = game_state.staging.get_copy_bank(pc_id).expect("Missing component bank");

            let title = match buy_result {
                Ok(()) => MessageType::ShopTitle(bank),
                Err(BuyError::CantAfford) => MessageType::ShopTitleInsufficientFunds(bank),
                Err(BuyError::InventoryFull) => MessageType::ShopTitleInventoryFull(bank),
                Err(BuyError::NoEffect) => MessageType::ShopTitleNoEffect(bank),
            };

            let maybe_selection = SelectMenuOperation::new(
                self.renderer.borrow_mut().deref_mut(),
                &mut self.input_source,
                Some(title),
                &self.language,
                menu,
                None,
                Some(game_state.staging.entity(pc_id))).run_can_escape();

            if let Some((item_id, _)) = maybe_selection {

                buy_result = self.buy_item(game_state, item_id);

            } else {
                break;
            }
        }
    }

    fn inventory_menu(&mut self, game_state: &mut GameState) {
        let GlobalIds { pc_id, .. } = game_state.global_ids.expect("Uninitialised game state");
        let mut current_menu_state = None;

        loop {
            let capacity = game_state.staging.get_copy_inventory_capacity(pc_id).expect("Missing component inventory_capacity");
            let size = game_state.staging.borrow_inventory(pc_id).expect("Missing component inventory").len();

            let mut menu = SelectMenu::new();
            for entity_id in game_state.staging.borrow_inventory(pc_id).expect("Missing component inventory").iter() {
                let item = game_state.staging.entity(entity_id);
                let name = item.copy_name().expect("Missing component name");

                let menu_message = MenuMessageType::Name(name);
                menu.push(SelectMenuItem::new(menu_message, entity_id));
            }

            let maybe_selection = SelectMenuOperation::new(
                self.renderer.borrow_mut().deref_mut(),
                &mut self.input_source,
                Some(MessageType::Inventory {
                    capacity: capacity,
                    size: size,
                }),
                &self.language,
                menu,
                current_menu_state,
                Some(game_state.staging.entity(pc_id))).run_can_escape();

            if let Some((item_id, menu_state)) = maybe_selection {
                match self.item_menu(game_state, item_id) {
                    Some(ItemMenuSelection::Remove) => {
                        current_menu_state = None;
                    }
                    _ => {
                        current_menu_state = Some(menu_state);
                    }
                }
            } else {
                break;
            }
        }
    }

    fn item_menu(&mut self, game_state: &mut GameState, item_id: EntityId) -> Option<ItemMenuSelection> {
        let GlobalIds { pc_id, .. } = game_state.global_ids.expect("Uninitialised game state");
        let maybe_description = game_state.staging.get_copy_description(item_id);
        let name = game_state.staging.get_copy_name(item_id).expect("Missing component name");

        let title = if let Some(description) = maybe_description {
            MessageType::NameAndDescription(name, description)
        } else {
            MessageType::Name(name)
        };

        let mut menu = SelectMenu::new();

        menu.push(SelectMenuItem::new(MenuMessageType::Back, ItemMenuSelection::Back));
        menu.push(SelectMenuItem::new(MenuMessageType::Remove, ItemMenuSelection::Remove));

        let maybe_selection = SelectMenuOperation::new(
            self.renderer.borrow_mut().deref_mut(),
            &mut self.input_source,
            Some(title),
            &self.language,
            menu,
            None,
            Some(game_state.staging.entity(pc_id))).run_can_escape();

        if let Some((selection, _)) = maybe_selection {
            match selection {
                ItemMenuSelection::Back => {}
                ItemMenuSelection::Remove => {
                    self.remove_item(game_state, item_id);
                }
            }

            return Some(selection);
        } else {
            return None;
        }
    }

    fn remove_item(&mut self, game_state: &mut GameState, item_id: EntityId) {
        let GlobalIds { pc_id, .. } = game_state.global_ids.expect("Uninitialised game state");

        let mut inventory = game_state.staging.borrow_mut_inventory(pc_id).expect("Expected component inventory");

        inventory.remove(item_id);
    }

    fn garage_menu(&mut self, game_state: &mut GameState) {
        let GlobalIds { pc_id, .. } = game_state.global_ids.expect("Uninitialised game state");
        const DIRECTIONS: [Direction; 4] = [Direction::East, Direction::West, Direction::North, Direction::South];
        let mut result = Ok(());
        loop {
            let menu = {
                let mut menu = SelectMenu::new();

                let weapon_slots = game_state.staging.borrow_weapon_slots(pc_id).expect("Expected component weapon_slots");

                for d in DIRECTIONS.iter() {
                    let maybe_name = weapon_slots.get(*d).and_then(|id| game_state.staging.get_copy_name(*id));
                    let message = MenuMessageType::WeaponSlot(RelativeDirection::from(*d), maybe_name);
                    menu.push(SelectMenuItem::new(message, *d));
                }

                menu
            };

            let title = match result {
                Err(WeaponMenuError::InventoryFull) => MessageType::GarageInventoryFull,
                _ => MessageType::Garage,
            };

            let maybe_selection = SelectMenuOperation::new(
                self.renderer.borrow_mut().deref_mut(),
                &mut self.input_source,
                Some(title),
                &self.language,
                menu,
                None,
                Some(game_state.staging.entity(pc_id))).run_can_escape();

            if let Some((selection, _)) = maybe_selection {
                result = self.weapon_slot_menu(game_state, selection);
            } else {
                break;
            }
        }
    }

    fn weapon_slot_menu(&mut self, game_state: &mut GameState, slot: Direction) -> Result<(), WeaponMenuError> {
        let GlobalIds { pc_id, .. } = game_state.global_ids.expect("Uninitialised game state");

        let menu = {

            let mut menu = SelectMenu::new();

            let inventory = game_state.staging.borrow_inventory(pc_id).expect("Expected component inventory");

            menu.push(SelectMenuItem::new(MenuMessageType::Empty, None));

            for entity_id in inventory.iter() {
                if game_state.staging.contains_gun_type(entity_id) {
                    let name = game_state.staging.get_copy_name(entity_id).expect("Expected component name");
                    menu.push(SelectMenuItem::new(MenuMessageType::Name(name), Some(entity_id)));
                }
            }

            menu
        };

        let title = {
            let direction = RelativeDirection::from(slot);
            let weapon_slots = game_state.staging.borrow_weapon_slots(pc_id).expect("Expected component weapon_slots");
            let current = weapon_slots.get(slot);
            let maybe_name = current.map(|id| game_state.staging.get_copy_name(*id).expect("Expected component name"));
            MessageType::WeaponSlotTitle(direction, maybe_name)
        };

        let maybe_selection = SelectMenuOperation::new(
            self.renderer.borrow_mut().deref_mut(),
            &mut self.input_source,
            Some(title),
            &self.language,
            menu,
            None,
            Some(game_state.staging.entity(pc_id))).run_can_escape();

        if let Some((selection, _)) = maybe_selection {
            if let Some(weapon_id) = selection {
                self.equip_weapon(game_state, slot, weapon_id)
            } else {
                self.clear_weapon(game_state, slot)
            }
        } else {
            Ok(())
        }
    }

    fn equip_weapon(&mut self, game_state: &mut GameState, slot: Direction, to_equip: EntityId) -> Result<(), WeaponMenuError> {
        let GlobalIds { pc_id, .. } = game_state.global_ids.expect("Uninitialised game state");

        let maybe_to_unequip = {
            let mut weapon_slots = game_state.staging.borrow_mut_weapon_slots(pc_id).expect("Expected component weapon_slots");
            let maybe_to_unequip = weapon_slots.remove(slot);
            weapon_slots.insert(slot, to_equip);
            maybe_to_unequip
        };

        let mut inventory = game_state.staging.borrow_mut_inventory(pc_id).expect("Missing component inventory");

        inventory.remove(to_equip);
        if let Some(to_unequip) = maybe_to_unequip {
            inventory.insert(to_unequip);
        }

        Ok(())
    }

    fn clear_weapon(&mut self, game_state: &mut GameState, slot: Direction) -> Result<(), WeaponMenuError> {
        let GlobalIds { pc_id, .. } = game_state.global_ids.expect("Uninitialised game state");

        let maybe_weapon_id = {
            let mut weapon_slots = game_state.staging.borrow_mut_weapon_slots(pc_id).expect("Expected component weapon_slots");

            if weapon_slots.get(slot).is_some() {
                let max_inventory = game_state.staging.get_copy_inventory_capacity(pc_id).expect("Missing component inventory_capacity");
                let current_inventory = game_state.staging.borrow_inventory(pc_id).expect("Missing component inventory").len();

                if current_inventory >= max_inventory {
                    return Err(WeaponMenuError::InventoryFull);
                }
            }

            weapon_slots.remove(slot)
        };

        if let Some(weapon_id) = maybe_weapon_id {
            let mut inventory = game_state.staging.borrow_mut_inventory(pc_id).expect("Missing component inventory");
            inventory.insert(weapon_id);
        }

        Ok(())
    }

    fn buy_item(&mut self, game_state: &mut GameState, item_id: EntityId) -> Result<(), BuyError> {
        let GlobalIds { shop_id, pc_id, .. } = game_state.global_ids.expect("Uninitialised game state");

        let bank = game_state.staging.get_copy_bank(pc_id).expect("Missing component bank");
        let price = game_state.staging.get_copy_price(item_id).expect("Missing component price");

        if price > bank {
            return Err(BuyError::CantAfford);
        }

        self.pre_add_item_to_player(game_state, item_id)?;

        // commit the payment
        let remaining_bank = bank - price;
        game_state.staging.insert_bank(pc_id, remaining_bank);

        // remove the item from the shop
        game_state.staging.borrow_mut_inventory(shop_id).expect("Missing component inventory").remove(item_id);

        self.add_item_to_player(game_state, item_id);

        Ok(())
    }

    fn pre_add_item_to_player(&self, game_state: &GameState, item_id: EntityId) -> Result<(), BuyError> {
        let GlobalIds { pc_id, .. } = game_state.global_ids.expect("Uninitialised game state");

        let max_inventory = game_state.staging.get_copy_inventory_capacity(pc_id).expect("Missing component inventory_capacity");
        let current_inventory = game_state.staging.borrow_inventory(pc_id).expect("Missing component inventory").len();

        if current_inventory >= max_inventory {
            return Err(BuyError::InventoryFull);
        }

        if let Some(repair_type) = game_state.staging.get_copy_repair_type(item_id) {
            match repair_type {
                RepairType::Engine => {
                    let engine = game_state.staging.get_copy_engine_health(pc_id).expect("Missing component engine_health");
                    if engine.is_full() {
                        return Err(BuyError::NoEffect);
                    }
                }
                RepairType::Tyres => {
                    let tyres = game_state.staging.get_copy_tyre_health(pc_id).expect("Missing component tyre_health");
                    if tyres.is_full() {
                        return Err(BuyError::NoEffect);
                    }
                }
            }
        } else if let Some(new_armour) = game_state.staging.get_copy_armour_upgrade(item_id) {
            let current_armour = game_state.staging.get_copy_armour(pc_id).expect("Missing component armour");
            if new_armour <= current_armour {
                return Err(BuyError::NoEffect);
            }
        }

        Ok(())
    }

    fn add_item_to_player(&mut self, game_state: &mut GameState, item_id: EntityId) {
        let GlobalIds { pc_id, .. } = game_state.global_ids.expect("Uninitialised game state");

        if let Some(repair_type) = game_state.staging.get_copy_repair_type(item_id) {
            match repair_type {
                RepairType::Engine => {
                    game_state.staging.get_mut_engine_health(pc_id).expect("Missing component engine_health").inc(1);
                }
                RepairType::Tyres => {
                    game_state.staging.get_mut_tyre_health(pc_id).expect("Missing component tyre_health").inc(1);
                }
            }
        } else if let Some(new_armour) = game_state.staging.get_copy_armour_upgrade(item_id) {
            let current_armour = game_state.staging.get_copy_armour(pc_id).expect("Missing component armour");
            if new_armour > current_armour {
                game_state.staging.insert_armour(pc_id, new_armour);
            }
        } else {
            // add the item to the player's inventory
            game_state.staging.borrow_mut_inventory(pc_id).expect("Missing component inventory").insert(item_id);
        }
    }

    fn next_level_menu(&mut self, game_state: &mut GameState) -> bool {

        self.unstage(TerrainType::Road, game_state);

        true
    }

    fn prepare_between_levels(&mut self, game_state: &mut GameState) {
        let GlobalIds { pc_id, shop_id, .. } = game_state.global_ids.expect("Uninitialised game state");

        let mut rng_borrow = self.rng.inner_mut();
        let mut rng = rng_borrow.deref_mut();

        let total_shop_roll = RANDOM_SHOP_ITEM_WEIGHTS.iter().fold(0, |acc, &x| acc + x);
        let num_shop_items = rng.gen::<usize>() % (SHOP_MAX_ITEMS - SHOP_MIN_ITEMS) + SHOP_MIN_ITEMS;
        let mut inventory = EntitySet::new();

        for _ in 0..num_shop_items {
            let mut roll = rng.gen::<usize>() % total_shop_roll;
            for (weight, item_type) in izip!(RANDOM_SHOP_ITEM_WEIGHTS.iter(), RANDOM_SHOP_ITEM_TYPES.iter()) {
                if roll < *weight {
                    let id = game_state.entity_ids.new_id();
                    inventory.insert(id);

                    match *item_type {
                        ShopItemType::Pistol => {
                            prototypes::pistol(game_state.staging.entity_mut(id));
                        }
                        ShopItemType::Shotgun => {
                            prototypes::shotgun(game_state.staging.entity_mut(id));
                        }
                        ShopItemType::MachineGun => {
                            prototypes::machine_gun(game_state.staging.entity_mut(id));
                        }
                        ShopItemType::Railgun => {
                            prototypes::railgun(game_state.staging.entity_mut(id));
                        }
                        ShopItemType::EngineRepair => {
                            prototypes::engine_repair(game_state.staging.entity_mut(id));
                        }
                        ShopItemType::TyresRepair => {
                            prototypes::tyres_repair(game_state.staging.entity_mut(id));
                        }
                        ShopItemType::Armour(amount) => {
                            prototypes::armour_upgrade(game_state.staging.entity_mut(id), amount);
                        }
                        ShopItemType::SpareTyre => {
                            prototypes::spare_tyre(game_state.staging.entity_mut(id));
                        }
                        ShopItemType::EngineRepairKit => {
                            prototypes::engine_repair_kit(game_state.staging.entity_mut(id));
                        }
                    }

                    break;
                }

                roll -= *weight;
            }
        }

        prototypes::shop(game_state.staging.entity_mut(shop_id), inventory);

        let bank = game_state.staging.get_copy_bank(pc_id).expect("Missing component bank");
        let letter_count = game_state.staging.get_copy_letter_count(pc_id).expect("Missing component letter_count");
        game_state.staging.insert_bank(pc_id, bank + 20 + letter_count * 40);
        game_state.staging.insert_letter_count(pc_id, 0);
        let mut hit_points = game_state.staging.get_copy_hit_points(pc_id).expect("Missing component hit_points");
        hit_points.fill();
        game_state.staging.insert_hit_points(pc_id, hit_points);
        game_state.staging.insert_message_log(pc_id, MessageLog::new());
    }

    fn install_control_map(game_state: &mut GameState, control_map: ControlMap) {
        let GlobalIds { pc_id, level_id, .. } = game_state.global_ids.expect("Uninitialised game state");

        let level = game_state.levels.level_mut(level_id);
        level.ecs.insert_control_map(pc_id, control_map);
    }

    fn game_loop(&mut self, game_state: &mut GameState) -> GameResult<ExitReason> {

        if game_state.between_levels {
            return Ok(ExitReason::BetweenLevels);
        }

        loop {

            let GlobalIds { pc_id, level_id, .. } = game_state.global_ids.expect("Uninitialised game state");

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
                    let old_ticket = level.ecs.get_copy_schedule_ticket(entity_id).expect("Expected schedule_ticket component");
                    let new_ticket = level.turn_schedule.insert_with_ticket(entity_id, 0, old_ticket);
                    level.ecs.insert_schedule_ticket(entity_id, new_ticket);
                    return Ok(reason);
                }
                TurnResolution::Schedule(entity_id, delay) => {
                    let level = game_state.levels.level_mut(level_id);
                    let ticket = level.turn_schedule.insert(entity_id, delay);
                    level.ecs.insert_schedule_ticket(entity_id, ticket);
                }
                TurnResolution::NoSchedule => {}
                TurnResolution::LevelSwitch { entity_id, exit_id, level_switch } => {
                    self.switch_level(entity_id, exit_id, level_switch, game_state);
                    if level_switch == LevelSwitch::LeaveLevel {
                        self.prepare_between_levels(game_state);
                    }
                    return Ok(ExitReason::BetweenLevels);
                }
                TurnResolution::GameOver(reason) => {
                    return Ok(ExitReason::GameOver(reason));
                }
            }
        }
    }

    fn death_message(&self, game_state: &GameState) {
        let GlobalIds { pc_id, level_id, .. } = game_state.global_ids.expect("Unitialised game state");
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
        let GlobalIds { pc_id, level_id, .. } = game_state.global_ids.expect("Unitialised game state");

        let ref ecs = game_state.levels.level(level_id).ecs;
        ecs.borrow_mut_message_log(pc_id).expect("Expected message log component").add(message);
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
                current_menu_state,
                None).run_can_escape() {

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
                    Some(menu_state),
                    None).publish();

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

    fn init_demo(&mut self, game_state: &mut GameState) {

        let pc_id = game_state.entity_ids.new_id();

        let mut action = EcsAction::new();
        prototypes::pc(action.entity_mut(pc_id), Coord::new(0, 0));

        let pistol_id = game_state.entity_ids.new_id();
        prototypes::pistol(action.entity_mut(pistol_id));

        action.borrow_mut_weapon_slots(pc_id).expect("Missing component weapon_slots")
            .insert(Direction::East, pistol_id);

        // throw away connections in the first level a they would have nothing to connect to anyway
        let (level, _) = Level::new_with_entity(TerrainType::Road,
                                                pc_id,
                                                &mut action,
                                                &game_state.entity_ids,
                                                &self.rng,
                                                game_state.action_id,
                                                None,
                                                0);

        game_state.action_id += 1;

        let level_id = game_state.levels.add_level(level);

        game_state.global_ids = Some(GlobalIds {
            pc_id: pc_id,
            level_id: level_id,
            shop_id: game_state.entity_ids.new_id(),
        });
    }

    fn unstage(&mut self, terrain_type: TerrainType, game_state: &mut GameState) {
        let entity_id = game_state.staged.take().expect("No staged entity");
        let global_ids = game_state.global_ids.as_mut().expect("Unitialised game state");

        let mut entity_remove = EcsAction::new();
        let mut entity_insert = EcsAction::new();

        if let Some(weapon_slots) = game_state.staging.borrow_weapon_slots(entity_id) {
            for (_, id) in weapon_slots.iter() {
                entity_remove.entity_delete_by_id(*id, &game_state.staging);
            }
        }

        if let Some(inventory) = game_state.staging.borrow_inventory(entity_id) {
            for id in inventory.iter() {
                entity_remove.entity_delete_by_id(id, &game_state.staging);
            }
        }

        entity_remove.entity_delete_by_id(entity_id, &game_state.staging);
        game_state.staging.commit_into(&mut entity_remove, &mut entity_insert);
        game_state.action_id += 1;

        let (level, _) = Level::new_with_entity(terrain_type,
                                                entity_id,
                                                &mut entity_insert,
                                                &game_state.entity_ids,
                                                &self.rng,
                                                game_state.action_id,
                                                None,
                                                global_ids.level_id + 1);
        game_state.action_id += 1;

        // can't go back to previous levels
        game_state.levels.level_mut(global_ids.level_id).clear();

        let new_level_id = game_state.levels.add_level(level);
        global_ids.level_id = new_level_id;

        game_state.staging.clear();
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
                                           Some(parent_ctx),
                                           global_ids.level_id + 1)
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
            LevelSwitch::LeaveLevel => {
                game_state.staging.commit(&mut entity_insert);
                game_state.staged = Some(entity_id);
                global_ids.level_id
            }
        };

        // update the current level
        global_ids.level_id = new_level_id;
    }

    fn pc_observe_from_action(&self, action: &mut EcsAction, entity_id: EntityId,
                              level_id: LevelId, level: &Level, action_id: ActionId) {

        let position = action.get_copy_position(entity_id).expect("Missing component position");
        let vision_distance = action.get_copy_vision_distance(entity_id).expect("Missing component vision_distance");
        let mut knowledge = action.borrow_mut_drawable_knowledge(entity_id).expect("Missing component drawable_knowledge");
        let level_knowledge = knowledge.level_mut_or_insert_size(level_id,
                                                                 level.spatial_hash.width(),
                                                                 level.spatial_hash.height());

        let action_env = ActionEnv::new(&level.ecs, action_id);

        self.pc_observer.observe(position, &level.spatial_hash, vision_distance, level_knowledge, action_env);
    }
}
