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

    sh.update(&ctx, &g, 0);
    ctx.commit(&mut g);

    for i in 0..height {
        for j in 0..width {
            let coord = math::Coord::new(j as isize, i as isize);
            let cell = sh.get(coord);
            if cell.solid() {
                print!("#");
            } else {
                print!(" ");
            }
        }
        print!("\n");
    }

    let action = action_test(ctx.entity(pc_id.unwrap()));

    println!("{:?}", action);
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
