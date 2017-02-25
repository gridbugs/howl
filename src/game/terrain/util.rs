use std::ops::DerefMut;
use ecs::*;
use game::*;
use game::data::*;
use coord::Coord;

pub fn terrain_from_strings<S: TurnScheduleQueue>(strings: &[&str],
                                                  level_switch: Option<LevelSwitch>,
                                                  ids: &EntityIdReserver,
                                                  schedule: &mut S,
                                                  g: &mut EcsAction) -> (usize, usize) {
    let width = strings[0].len();
    let height = strings.len();

    let mut y = 0;
    for line in strings {
        let mut x = 0;
        for ch in line.chars() {
            let coord = Coord::new(x, y);
            match ch {
                '#' => {
                    prototypes::wall(g.entity_mut(ids.new_id()), coord);
                    prototypes::floor(g.entity_mut(ids.new_id()), coord);
                }
                '&' => {
                    prototypes::tree(g, ids, coord);

                    prototypes::outside_floor(g.entity_mut(ids.new_id()), coord);
                }
                '.' => {
                    prototypes::floor(g.entity_mut(ids.new_id()), coord);
                }
                ',' => {
                    prototypes::outside_floor(g.entity_mut(ids.new_id()), coord);
                }
                '=' => {
                    if let Some(level_switch) = level_switch {
                        prototypes::book(g.entity_mut(ids.new_id()), coord, level_switch);
                        prototypes::outside_floor(g.entity_mut(ids.new_id()), coord);
                    }
                }
                '+' => {
                    prototypes::door(g.entity_mut(ids.new_id()), coord, DoorState::Closed);
                    prototypes::floor(g.entity_mut(ids.new_id()), coord);
                }
                't' => {
                    prototypes::outside_floor(g.entity_mut(ids.new_id()), coord);
                    let id = prototypes::terror_pillar(g, ids, coord);

                    let turn_offset = g.turn_offset(id).expect("Expected component turn_offset");
                    let ticket = schedule.schedule_turn(id, turn_offset);
                    g.insert_schedule_ticket(id, ticket);
                }
                _ => panic!(),
            }
            x += 1;
        }
        y += 1;
    }

    (width, height)
}

pub fn generate_clouds<S: TurnScheduleQueue>(width: usize,
                                             height: usize,
                                             ids: &EntityIdReserver,
                                             rng: &GameRng,
                                             schedule: &mut S,
                                             g: &mut EcsAction) {
    let cloud_id = ids.new_id();
    prototypes::clouds(g.entity_mut(cloud_id), width, height, rng.inner_mut().deref_mut());
    let turn_offset = g.turn_offset(cloud_id).expect("Expected component turn_offset");
    let ticket = schedule.schedule_turn(cloud_id, turn_offset);
    g.insert_schedule_ticket(cloud_id, ticket);
}
