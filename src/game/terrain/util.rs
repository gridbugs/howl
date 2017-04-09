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
                    prototypes::road(g.entity_mut(ids.new_id()), coord, r);
                }
                '#' => {
                    prototypes::barrel(g.entity_mut(ids.new_id()), coord);
                    prototypes::road(g.entity_mut(ids.new_id()), coord, r);
                }
                '&' => {
                    prototypes::letter(g.entity_mut(ids.new_id()), coord);
                    prototypes::road(g.entity_mut(ids.new_id()), coord, r);
                }
                ',' => {
                    prototypes::dirt(g.entity_mut(ids.new_id()), coord, r);
                }
                'z' => {
                    prototypes::dirt(g.entity_mut(ids.new_id()), coord, r);
                    let id = ids.new_id();
                    prototypes::zombie(g.entity_mut(id), coord);
                    let turn_offset = g.get_copy_turn_offset(id).expect("Expected component turn_offset");
                    let ticket = schedule.schedule_turn(id, turn_offset);
                    g.insert_schedule_ticket(id, ticket);
                }
                'c' => {
                    prototypes::dirt(g.entity_mut(ids.new_id()), coord, r);
                    let id = ids.new_id();
                    prototypes::car(g.entity_mut(id), coord);
                    let turn_offset = g.get_copy_turn_offset(id).expect("Expected component turn_offset");
                    let ticket = schedule.schedule_turn(id, turn_offset);
                    g.insert_schedule_ticket(id, ticket);


                    let gun_id = ids.new_id();
                    prototypes::shotgun(g.entity_mut(gun_id));
                    g.borrow_mut_weapon_slots(id).unwrap().insert(Direction::North, gun_id);
                    g.borrow_mut_weapon_slots(id).unwrap().insert(Direction::South, gun_id);
                }
                'b' => {
                    prototypes::dirt(g.entity_mut(ids.new_id()), coord, r);
                    let id = ids.new_id();
                    prototypes::bike(g.entity_mut(id), coord);
                    let turn_offset = g.get_copy_turn_offset(id).expect("Expected component turn_offset");
                    let ticket = schedule.schedule_turn(id, turn_offset);
                    g.insert_schedule_ticket(id, ticket);


                    let gun_id = ids.new_id();
                    prototypes::pistol(g.entity_mut(gun_id));
                    g.borrow_mut_weapon_slots(id).unwrap().insert(Direction::North, gun_id);
                    g.borrow_mut_weapon_slots(id).unwrap().insert(Direction::South, gun_id);
                    g.borrow_mut_weapon_slots(id).unwrap().insert(Direction::East, gun_id);
                    g.borrow_mut_weapon_slots(id).unwrap().insert(Direction::West, gun_id);
                }
                'Z' => {
                    prototypes::road(g.entity_mut(ids.new_id()), coord, r);
                    let id = ids.new_id();
                    prototypes::zombie(g.entity_mut(id), coord);
                    let turn_offset = g.get_copy_turn_offset(id).expect("Expected component turn_offset");
                    let ticket = schedule.schedule_turn(id, turn_offset);
                    g.insert_schedule_ticket(id, ticket);
                }
                '%' => {
                    prototypes::wreck(g.entity_mut(ids.new_id()), coord, r);
                    prototypes::road(g.entity_mut(ids.new_id()), coord, r);
                }
                '$' => {
                    prototypes::wreck(g.entity_mut(ids.new_id()), coord, r);
                    prototypes::dirt(g.entity_mut(ids.new_id()), coord, r);
                }
                '~' => {
                    prototypes::acid(g.entity_mut(ids.new_id()), coord, r);
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
    let physics_id = ids.new_id();
    prototypes::physics(g.entity_mut(physics_id));
    let turn_offset = g.get_copy_turn_offset(physics_id).expect("Expected component turn_offset");
    let ticket = schedule.schedule_turn(physics_id, turn_offset);
    g.insert_schedule_ticket(physics_id, ticket);
}

