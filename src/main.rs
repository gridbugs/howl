#![allow(dead_code)]

#[macro_use]
extern crate itertools;
extern crate rand;
extern crate num;
extern crate rustty;

mod frontends;
mod ecs;
mod math;
mod game;
mod util;
mod direction;
mod grid;
mod behaviour;

fn main() {

    let mut sh = game::SpatialHashTable::new();
    let mut ctx = ecs::EcsCtx::new();
    let mut ids = util::LeakyReserver::new();

    let mut g = ecs::EcsAction::new();

    let strings = ["&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&",
                   "&,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,&",
                   "&,,############################,,,,,,&",
                   "&,,#.........#................#,,&,,,&",
                   "&,,#.........#................#,,,&,,&",
                   "&,,#..........................#,,&,,,&",
                   "&&,#.........#................#,,,,,,&",
                   "&,&#.........##########.#######,,,,,,&",
                   "&,,#.........#,,,,,,,,,,,,,,,,,,,,,,,&",
                   "&&,#.........#,,,,,,,,,&,,,,,,,&,&,&,&",
                   "&,,#.........#,,,,,&,,,,,,,,&,,,,,,,,&",
                   "&,,#..........,,,,,,&,,,,,,,,,,,,,,,,&",
                   "&&,#.........#,,,,,&,,,,,,,,,&,,,,,,,&",
                   "&,,#.........#,,,,,,,,,,&,,&,,,&,&,,,&",
                   "&,&#.........#,,,,@,,,,&,,,,,,,,,,,,,&",
                   "&,,###########,,,,,,,&,,,,,,,&,&,,,,,&",
                   "&,,&,,,,,,,,,,,,,,,,,&,,,,&,,,,,,,,,,&",
                   "&,&,,,,,,,,,,,,&,,,,,,,,,,,,,,,,,,,,,&",
                   "&,,,&,,,,,,,,,,,,,,,,&,,,,,#########,&",
                   "&,&,,,&,,,,,&,,&,,,,&,,,,,,#.......#,&",
                   "&,,,,,&,,,,,,,,,&,,,,&,,,,,#.......#,&",
                   "&,,,,,,,,,&,,,,,,,,,,,,,&,,........#,&",
                   "&,&,&,,,,&&,,,&,&,,,,,,,&,,#.......#,&",
                   "&,,,,,,,,,,,,,,,,,,,&,,,,,,#.......#,&",
                   "&,,,&,,,,,,,&,,,,,,,,,,,,,,#########,&",
                   "&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&"];

    let height = strings.len();
    let width = strings[0].len();

    let mut pc_id = None;

    let mut y = 0;
    for line in &strings {
        let mut x = 0;
        for ch in line.chars() {
            let coord = math::Coord::new(x, y);
            match ch {
                '#' => {
                    ecs::prototypes::wall(g.entity_mut(ids.reserve()), coord);
                    ecs::prototypes::floor(g.entity_mut(ids.reserve()), coord);
                }
                '&' => {
                    ecs::prototypes::tree(g.entity_mut(ids.reserve()), coord);
                    ecs::prototypes::outside_floor(g.entity_mut(ids.reserve()), coord);
                }
                '.' => {
                    ecs::prototypes::floor(g.entity_mut(ids.reserve()), coord);
                }
                ',' => {
                    ecs::prototypes::outside_floor(g.entity_mut(ids.reserve()), coord);
                }
                '@' => {
                    let id = ids.reserve();
                    pc_id = Some(id);
                    ecs::prototypes::pc(g.entity_mut(id), coord);
                    ecs::prototypes::outside_floor(g.entity_mut(ids.reserve()), coord);
                }
                _ => panic!(),
            }
            x += 1;
        }
        y += 1;
    }

    sh.update(game::Turn::new(&ctx, 0), &g);
    ctx.commit(&mut g);

    let turn = game::Turn::new(&ctx, 1);
    pc_observe(ctx.entity(pc_id.unwrap()), turn, &sh);

    pc_draw_knowledge(ctx.entity(pc_id.unwrap()), math::Coord::new(0, 0), width, height);

//    let action = action_test(ctx.entity(pc_id.unwrap()));


//    println!("{:?}", action);
}

fn pc_observe(entity: ecs::EntityRef, turn: game::Turn, sh: &game::SpatialHashTable) {
    let mut knowledge = entity.ansi_drawable_knowledge_borrow_mut().unwrap();
    let level_knowledge = knowledge.level_mut(0);
    let position = entity.position().unwrap();
    let vision_distance = entity.vision_distance().unwrap();
    let shadowcast = game::Shadowcast::new();

    shadowcast.observe(position, sh, vision_distance, level_knowledge, turn);
}

fn pc_draw_knowledge(entity: ecs::EntityRef, top_left: math::Coord, width: usize, height: usize) {
    let knowledge = entity.ansi_drawable_knowledge_borrow().unwrap();
    let level_knowledge = knowledge.level(0);

    for i in 0..(height as isize + top_left.y) {
        for j in 0..(width as isize + top_left.x) {
            let coord = math::Coord::new(j, i);
            let cell = level_knowledge.get_with_default(coord);
            if let Some(foreground) = cell.foreground() {
                let simple = match foreground {
                    frontends::ansi::ComplexTile::Wall { front, ..} => {
                        front
                    }
                    frontends::ansi::ComplexTile::Simple(tile) => {
                        tile
                    }
                };
                if let Some(ch) = simple.character() {
                    print!("{}", ch);
                } else {
                    print!(" ");
                }
            } else {
                print!(" ");
            }
        }
        print!("\n");
    }
}

fn action_test(entity: ecs::EntityRef) -> game::ActionArgs {
    let mut window_allocator = frontends::ansi::WindowAllocator::new().unwrap();
    let input_source = window_allocator.make_input_source();
    let behaviour = game::BehaviourContext::new(input_source);

    run_behaviour(&behaviour, entity)
}

fn run_behaviour(behaviour: &game::BehaviourContext, entity: ecs::EntityRef) -> game::ActionArgs {
    let mut behaviour_state = entity.behaviour_state_borrow_mut().unwrap();

    if !behaviour_state.is_initialised() {
        let behaviour_type = entity.behaviour_type().unwrap();
        behaviour_state.initialise(behaviour.graph(), behaviour.nodes().index(behaviour_type)).unwrap();
    }

    let input = game::BehaviourInput {
        entity: entity,
    };

    behaviour_state.run(behaviour.graph(), input).unwrap()
}
