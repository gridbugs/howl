#![allow(dead_code)]

#[macro_use] extern crate itertools;
extern crate num;
extern crate rand;
extern crate rustty;

#[macro_use] mod debug;
#[macro_use] mod ecs;
mod perlin;
mod renderer;
mod geometry;
mod grid;
mod colour;
mod terminal;
mod allocator;

use ecs::entity_types::*;
use ecs::message::Message;
use ecs::message::FieldType;
use ecs::message::Field::*;
use ecs::entity::Component::*;
use ecs::entity::ComponentType as Type;
use ecs::entity::{EntityTable, EntityId, Entity};
use ecs::systems::schedule::Schedule;
use ecs::systems::terminal_player_actor;
use ecs::systems::window_renderer;
use ecs::systems::apply_update::apply_update;
use ecs::components::level::Level;

use terminal::window_manager::{WindowManager, WindowRef, InputSource};
use terminal::window_buffer::WindowBuffer;

use std::io;
use std::cell;

const LEVEL_WIDTH: usize = 10;
const LEVEL_HEIGHT: usize = 10;

fn populate(entities: &mut EntityTable) -> Option<EntityId> {
    let level_id = entities.add(make_level(LEVEL_WIDTH, LEVEL_HEIGHT));

    for y in 0..LEVEL_HEIGHT {
        for x in 0..LEVEL_WIDTH {

            let floor = entities.add(make_floor(x as isize, y as isize, level_id));
            if let Some(&mut LevelData(ref mut level)) = entities.get_mut(level_id).get_mut(Type::LevelData) {
                level.add(floor);
            }

            if x == 0 || x == LEVEL_WIDTH - 1 || y == 0 || y == LEVEL_HEIGHT - 1 {
                let wall = entities.add(make_wall(x as isize, y as isize, level_id));
                if let Some(&mut LevelData(ref mut level)) = entities.get_mut(level_id).get_mut(Type::LevelData) {
                    level.add(wall);
                }
            }
        }
    }

    let pc = entities.add(make_pc(3, 2, level_id));
    if let Some(&mut LevelData(ref mut level)) = entities.get_mut(level_id).get_mut(Type::LevelData) {
        level.schedule.borrow_mut().set_pc(pc);
        level.add(pc);
        Some(pc)
    } else {
        None
    }
}

struct GameContext<'a> {
    entities: EntityTable,
    pc: Option<EntityId>,
    input_source: InputSource<'a>,
    game_window: WindowRef<'a>,
}

impl<'a> GameContext<'a> {
    fn new(input_source: InputSource<'a>, game_window: WindowRef<'a>) -> Self {
        GameContext {
            entities: EntityTable::new(),
            pc: None,
            input_source: input_source,
            game_window: game_window,
        }
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

    fn pc_schedule_next(&self) -> Message {
        self.pc_schedule().next().unwrap()
    }

    fn get_action(&mut self, turn: &Message) -> Message {
        loop {
            if let Some(message) =
                terminal_player_actor::get_action(&self.input_source, &self.entities, turn)
            {
                return message;
            }
        }
    }

    fn turn_entity(&self, turn: &Message) -> &Entity {
        match turn.get(FieldType::ActorTurn).unwrap() {
            &ActorTurn { actor: entity_id } => {
                self.entities.get(entity_id)
            },
            _ => unreachable!()
        }
    }

    fn turn_entity_is_pc(&self, turn: &Message) -> bool {
        self.turn_entity(turn).has(Type::PlayerActor)
    }

    fn apply_action(&mut self, action: &Message) {
        apply_update(action, &mut self.entities);
    }

    fn render_level(&self, level: EntityId) {
        window_renderer::render(self.game_window, &self.entities, level);
    }

    fn render_pc_level(&self) {
        self.render_level(self.pc_level_id());
    }
}

const DEBUG_WINDOW_WIDTH: usize = 80;
const DEBUG_WINDOW_HEIGHT: usize = 10;

fn main() {
    window_session();
}

fn window_session() {
    let wm = terminal::window_manager::WindowManager::new().unwrap();

    // Initialise debug window
    let mut debug_buffer = make_debug_window(&wm, DEBUG_WINDOW_WIDTH,
                                                  DEBUG_WINDOW_HEIGHT);

    debug::debug::init(&mut debug_buffer as &mut io::Write);

    game(wm.make_input_source(), wm.make_window(0, 0, 80, 20));
}

fn game<'a>(input_source: InputSource<'a>, game_window: WindowRef<'a>) {
    let mut game_context = GameContext::new(input_source, game_window);
    game_context.pc = populate(&mut game_context.entities);

    loop {
        let turn = game_context.pc_schedule_next();

        if game_context.turn_entity_is_pc(&turn) {
            game_context.render_pc_level();
        }

        let action = game_context.get_action(&turn);

        if action.has(FieldType::QuitGame) {
            break;
        }

        game_context.apply_action(&action);

        game_context.render_pc_level();
    }
}

fn make_debug_window<'a>(wm: &'a WindowManager, width: usize, height: usize)
    -> WindowBuffer<'a>
{
    let debug_buffer = wm.make_window_buffer(
        (wm.get_width() - width) as isize,
        (wm.get_height() - height) as isize,
        width, height, 2, 1);

    debug_buffer.draw_borders();

    debug_buffer
}
