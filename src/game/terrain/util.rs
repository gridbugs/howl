use ecs::*;
use game::*;
use game::data::*;
use coord::Coord;
use direction::Direction;

pub fn terrain_from_strings<S: TurnScheduleQueue>(strings: &[&str],
                                                  level_switch: Option<LevelSwitch>,
                                                  ids: &EntityIdReserver,
                                                  schedule: &mut S,
                                                  g: &mut EcsAction,
                                                  rng: &GameRng) -> (usize, usize) {
    let width = strings[0].len();
    let height = strings.len();

    let mut y = 0;
    for line in strings {
        let mut x = 0;
        for ch in line.chars() {
            let coord = Coord::new(x, y);
            match ch {
                '.' => {
                    prototypes::road(g.entity_mut(ids.new_id()), coord, rng);
                }
                '#' => {
                    prototypes::barrel(g.entity_mut(ids.new_id()), coord);
                    prototypes::road(g.entity_mut(ids.new_id()), coord, rng);
                }
                '&' => {
                    prototypes::letter(g.entity_mut(ids.new_id()), coord);
                    prototypes::road(g.entity_mut(ids.new_id()), coord, rng);
                }
                ',' => {
                    prototypes::dirt(g.entity_mut(ids.new_id()), coord, rng);
                }
                'z' => {
                    prototypes::dirt(g.entity_mut(ids.new_id()), coord, rng);
                    let id = ids.new_id();
                    prototypes::zombie(g.entity_mut(id), coord);
                    let turn_offset = g.turn_offset(id).expect("Expected component turn_offset");
                    let ticket = schedule.schedule_turn(id, turn_offset);
                    g.insert_schedule_ticket(id, ticket);
                }
                'c' => {
                    prototypes::dirt(g.entity_mut(ids.new_id()), coord, rng);
                    let id = ids.new_id();
                    prototypes::car(g.entity_mut(id), coord);
                    let turn_offset = g.turn_offset(id).expect("Expected component turn_offset");
                    let ticket = schedule.schedule_turn(id, turn_offset);
                    g.insert_schedule_ticket(id, ticket);


                    let gun_id = ids.new_id();
                    prototypes::shotgun(g.entity_mut(gun_id));
                    g.weapon_slots_mut(id).unwrap().insert(Direction::North, gun_id);
                    g.weapon_slots_mut(id).unwrap().insert(Direction::South, gun_id);
                }
                'b' => {
                    prototypes::dirt(g.entity_mut(ids.new_id()), coord, rng);
                    let id = ids.new_id();
                    prototypes::bike(g.entity_mut(id), coord);
                    let turn_offset = g.turn_offset(id).expect("Expected component turn_offset");
                    let ticket = schedule.schedule_turn(id, turn_offset);
                    g.insert_schedule_ticket(id, ticket);


                    let gun_id = ids.new_id();
                    prototypes::pistol(g.entity_mut(gun_id));
                    g.weapon_slots_mut(id).unwrap().insert(Direction::North, gun_id);
                    g.weapon_slots_mut(id).unwrap().insert(Direction::South, gun_id);
                    g.weapon_slots_mut(id).unwrap().insert(Direction::East, gun_id);
                    g.weapon_slots_mut(id).unwrap().insert(Direction::West, gun_id);
                }
                'Z' => {
                    prototypes::road(g.entity_mut(ids.new_id()), coord, rng);
                    let id = ids.new_id();
                    prototypes::zombie(g.entity_mut(id), coord);
                    let turn_offset = g.turn_offset(id).expect("Expected component turn_offset");
                    let ticket = schedule.schedule_turn(id, turn_offset);
                    g.insert_schedule_ticket(id, ticket);
                }
                '%' => {
                    prototypes::wreck(g.entity_mut(ids.new_id()), coord, rng);
                    prototypes::road(g.entity_mut(ids.new_id()), coord, rng);
                }
                '$' => {
                    prototypes::wreck(g.entity_mut(ids.new_id()), coord, rng);
                    prototypes::dirt(g.entity_mut(ids.new_id()), coord, rng);
                }
                '~' => {
                    prototypes::acid(g.entity_mut(ids.new_id()), coord, rng);
                }
                _ => panic!(),
            }

            if x == line.len() as isize - 1 {
                prototypes::goal(g.entity_mut(ids.new_id()), coord, level_switch.unwrap());
            }

            x += 1;
        }
        y += 1;
    }

    add_management_entities(ids, schedule, g);

    (width, height)
}

pub fn add_management_entities<S: TurnScheduleQueue>(ids: &EntityIdReserver,
                                                     schedule: &mut S,
                                                     g: &mut EcsAction) {
    let acid_animator_id = ids.new_id();
    prototypes::acid_animator(g.entity_mut(acid_animator_id));
    let turn_offset = g.turn_offset(acid_animator_id).expect("Expected component turn_offset");
    let ticket = schedule.schedule_turn(acid_animator_id, turn_offset);
    g.insert_schedule_ticket(acid_animator_id, ticket);

    let physics_id = ids.new_id();
    prototypes::physics(g.entity_mut(physics_id));
    let turn_offset = g.turn_offset(physics_id).expect("Expected component turn_offset");
    let ticket = schedule.schedule_turn(physics_id, turn_offset);
    g.insert_schedule_ticket(physics_id, ticket);
}

