#![allow(dead_code)]

#[macro_use]
extern crate itertools;
extern crate rand;
extern crate num;
extern crate rustty;

mod frontends;
mod ecs;
mod math;
mod knowledge;
mod spatial_hash;
mod util;
mod direction;
mod grid;

fn main() {

    let mut sh = spatial_hash::SpatialHashTable::new();
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
                    ecs::prototypes::pc(g.entity_mut(ids.reserve()), coord);
                    ecs::prototypes::outside_floor(g.entity_mut(ids.reserve()), coord);
                }
                _ => panic!(),
            }
            x += 1;
        }
        y += 1;
    }

    sh.update(&ctx, &g);
    ctx.commit(&mut g);

    for i in 0..height {
        for j in 0..width {
            let coord = math::Coord::new(j as isize, i as isize);
            if let Some(cell) = sh.get(coord) {
                if cell.solid() {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
        }
        print!("\n");
    }
}
