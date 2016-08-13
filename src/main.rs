#![allow(dead_code)]

#[macro_use] extern crate itertools;
extern crate num;
extern crate rand;
extern crate rustty;

#[macro_use] mod debug;
#[macro_use] mod game;
mod perlin;
mod renderer;
mod geometry;
mod grid;
mod colour;
mod terminal;
mod allocator;

use game::entities::*;
use game::{
    EntityTable,
    EntityId,
    GameContext,
    GameEntity,
};
use game::rules;
use game::components::DoorState;

use terminal::window_manager::{WindowManager, WindowRef, InputSource};
use terminal::window_buffer::WindowBuffer;

use std::io;

fn populate(entities: &mut EntityTable) -> EntityId {
    let strings = [
        "####################################",
        "#............#.....................#",
        "#............#.....................#",
        "#............#.....................#",
        "#............#.....................#",
        "#............+.....................#",
        "#............#.....................#",
        "#............#.....................#",
        "#............#.....................#",
        "#............#.....................#",
        "#............#########+#############",
        "#............#.....................#",
        "#............#.....................#",
        "#............#.....................#",
        "########+#####.....................#",
        "#..................................#",
        "#..................................#",
        "#..................................#",
        "#.................@................#",
        "#..................................#",
        "#..................................#",
        "#..................................#",
        "####################################",
    ];

    let height = strings.len();
    let width = strings[0].len();

    let level_id = entities.add(make_level(width, height));
    entities.get_mut(level_id).level_data_mut().unwrap().set_id(level_id);

    let mut level_entities = Vec::new();

    let mut y = 0;
    for line in &strings {
        let mut x = 0;
        for ch in line.chars() {

            match ch {
                '#' => {
                    level_entities.push(make_wall(x, y, level_id));
                    level_entities.push(make_floor(x, y, level_id));
                },
                '.' => {
                    level_entities.push(make_floor(x, y, level_id));
                },
                '+' => {
                    level_entities.push(make_door(x, y, level_id, DoorState::Closed));
                    level_entities.push(make_floor(x, y, level_id));
                },
                '@' => {
                    level_entities.push(make_pc(x, y, level_id));
                    level_entities.push(make_floor(x, y, level_id));
                },
                _ => panic!(),
            };

            x += 1;
        }
        y += 1;
    }

    let mut pc = None;

    for entity in level_entities.drain(..) {
        let id = entities.add(entity);
        entities.get_mut(level_id).level_data_mut().unwrap().add(id);

        if entities.get(id).is_pc() {
            assert!(pc == None, "Multiple player characters");
            pc = Some(id);
            entities.get(level_id).level_data().unwrap().schedule.borrow_mut().set_pc(id);
        }
    }

    entities.get(level_id).level_data().unwrap().finalise(entities);

    pc.unwrap()
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

    game(wm.make_input_source(), wm.make_window(0, 0, 80, 24));
}

fn game<'a>(input_source: InputSource<'a>, game_window: WindowRef<'a>) {
    let mut game_context = GameContext::new(input_source, game_window);

    game_context.pc = Some(populate(&mut game_context.entities));

    game_context
        .rule(rules::door::detect_open)
        .rule(rules::collision::detect_collision);

    game_context.game_loop();
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
