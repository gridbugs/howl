use rand::Rng;
use engine_defs::*;
use ecs_content::*;
use game::*;
use content_types::*;
use math::Coord;
use math::Direction;

pub fn terrain_from_strings<S: TurnScheduleQueue, R: Rng>(strings: &[&str],
                                                          level_switch: Option<LevelSwitch>,
                                                          ids: &EntityIdReserver,
                                                          schedule: &mut S,
                                                          g: &mut EcsAction,
                                                          r: &mut R) -> (usize, usize) {
    let width = strings[0].len();
    let height = strings.len();

    let mut y = 0;
    for line in strings {
        let mut x = 0;
        for ch in line.chars() {
            let coord = Coord::new(x, y);
            match ch {
                '.' => {
                    prototypes::stone_floor(g.entity_mut(ids.new_id()), coord);
                }
                '#' => {
                    prototypes::stone_floor(g.entity_mut(ids.new_id()), coord);
                    prototypes::wall(g.entity_mut(ids.new_id()), coord);
                }
                '@' => {
                    prototypes::pc(g.entity_mut(ids.new_id()), coord);
                    prototypes::stone_floor(g.entity_mut(ids.new_id()), coord);
                }
                _ => panic!(),
            }

            x += 1;
        }
        y += 1;
    }

    (width, height)
}
