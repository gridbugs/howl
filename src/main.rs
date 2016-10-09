#![allow(dead_code)]

#[macro_use]
extern crate itertools;
extern crate num;
extern crate rand;
extern crate rustty;

#[macro_use]
mod debug;
#[macro_use]
mod table;
#[macro_use]
mod game;
mod perlin;
mod tile;
mod geometry;
mod schedule;
mod grid;
mod colour;
mod terminal;
mod best;
mod clear;
mod object_pool;
mod vision;
mod reserver;
mod math;
mod search;
mod behaviour;

use game::entities::*;
use game::{EntityContext, ReserveEntityId, EntityId, GameContext, Level, EntityWrapper};
use game::rules;
use game::components::DoorState;

use terminal::{Window, InputSource, WindowAllocator, BufferType, WindowBuffer};

use std::io;

fn populate(entities: &mut EntityContext) -> EntityId {
    let strings = ["&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&",
                   "&,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,&",
                   "&,,############################,,,,,,&",
                   "&,,#.........#................#,,&,,,&",
                   "&,,#.........#................#,,,&,,&",
                   "&,,#.........+................#,,&,,,&",
                   "&&,#.........#................#,,,,,,&",
                   "&,&#.........##########+#######,,,,,,&",
                   "&,,#.........#,,,,,,,,,,,,,,,,,,,,,,,&",
                   "&&,#.........#,d,,,,,,,&,,,,,,,&,&,&,&",
                   "&,,#.........#,,,,,&,,,,,,,,&,,,,,,,,&",
                   "&,,#.........+,,,,,,&,,,,,,,,,,,,,,,,&",
                   "&&,#.........#,,,,,&,,,,,,,,,&,,,,,,,&",
                   "&,,#.........#,,,,,,,,,,&,,&,,,&,&,,,&",
                   "&,&#.........#,,,,@,,,,&,,,,,,,,,,,,,&",
                   "&,,###########,,,,,,,&,,,,,,,&,&,,,,,&",
                   "&,,&,,,,,,,,,,,,,,,,,&,,,,&,,,,,,,,,,&",
                   "&,&,,,,,,,,,,,,&,,,,,,,,,,,,,,,,,,,,,&",
                   "&,,,&,,,,,,,,,,,,,,,,&,,,,,#########,&",
                   "&,&,,,&,,,,,&,,&,,,,&,,,,,,#.......#,&",
                   "&,,,,,&,,,,,,,,,&,,,,&,,,,,#.......#,&",
                   "&,,,,,,,,,&,,,,,,,,,,,,,&,,+.......#,&",
                   "&,&,&,,,,&&,,,&,&,,,,,,,&,,#.......#,&",
                   "&,,,,,,,,,,,,,,,,,,,&,,,,,,#.......#,&",
                   "&,,,&,,,,,,,&,,,,,,,,,,,,,,#########,&",
                   "&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&"];


    let height = strings.len();
    let width = strings[0].len();

    let level_id = entities.reserve_level_id();
    let mut level = Level::new(width, height, level_id);

    let mut level_entities = Vec::new();
    {
        let mut y = 0;
        for line in &strings {
            let mut x = 0;
            for ch in line.chars() {

                let moonlight = !level.is_cloud(x, y);

                match ch {
                    '#' => {
                        level_entities.push(make_wall(x, y));
                        level_entities.push(make_floor(x, y));
                    }
                    '&' => {
                        level_entities.push(make_tree(x, y));
                        level_entities.push(make_floor_outside(x, y, moonlight));
                    }
                    '.' => {
                        level_entities.push(make_floor(x, y));
                    }
                    ',' => {
                        level_entities.push(make_floor_outside(x, y, moonlight));
                    }
                    '+' => {
                        level_entities.push(make_door(x, y, DoorState::Closed));
                        level_entities.push(make_floor(x, y));
                    }
                    '@' => {
                        level_entities.push(make_pc(x, y));
                        level_entities.push(make_floor_outside(x, y, moonlight));
                    }
                    'd' => {
                        level_entities.push(make_dog(x, y));
                        level_entities.push(make_floor_outside(x, y, moonlight));
                    }
                    _ => panic!(),
                };

                x += 1;
            }
            y += 1;
        }
    }

    let mut pc = None;

    for entity in level_entities.drain(..) {
        let id = entities.reserve_entity_id();

        if entity.is_pc() {
            assert!(pc.is_none(), "Multiple player characters");
            pc = Some(id);
        }

        level.add_external(id, entity, 0);
    }

    entities.add_level(level);

    pc.expect("no player character found")
}

const DEBUG_WINDOW_WIDTH: usize = 80;
const DEBUG_WINDOW_HEIGHT: usize = 10;

fn main() {
    window_session();
}

fn window_session() {
    let wa = WindowAllocator::new().unwrap();

    // Initialise debug window
    let mut debug_buffer = make_debug_window(&wa, DEBUG_WINDOW_WIDTH, DEBUG_WINDOW_HEIGHT);

    debug::debug::init(&mut debug_buffer as &mut io::Write);

    game(wa.make_input_source(),
         wa.make_window(0, 0, 80, 30, BufferType::Double));
}

fn game<'a>(input_source: InputSource<'a>, game_window: Window<'a>) {
    let mut game_context = GameContext::new(input_source, game_window);

    game_context.pc = Some(populate(&mut game_context.entities));

    game_context.rule(rules::door::DetectOpen)
        .rule(rules::collision::DetectCollision)
        .rule(rules::axis_velocity::StartVelocityMovement)
        .rule(rules::axis_velocity::MaintainVelocityMovement)
        .rule(rules::burst_fire::BurstFireRule)
        .rule(rules::transformation::BeastTransformation)
        .rule(rules::transformation::HumanTransformation);

    game_context.game_loop();
}

fn make_debug_window<'a>(wa: &'a WindowAllocator, width: usize, height: usize) -> WindowBuffer<'a> {
    let mut debug_buffer = wa.make_window_buffer((wa.width() - width) as isize,
                                                 (wa.height() - height) as isize,
                                                 width,
                                                 height,
                                                 2,
                                                 1);


    debug_buffer.draw_borders();

    debug_buffer
}
