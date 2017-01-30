use ecs::*;
use game::*;
use game::data::*;
use coord::Coord;

pub fn terrain_from_strings<S: TurnScheduleQueue>(strings: &[&str],
                                                  ids: &EntityIdReserver,
                                                  schedule: &mut S,
                                                  g: &mut EcsAction) -> TerrainMetadata {
    let width = strings[0].len();
    let height = strings.len();

    let mut pc = None;

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
                '+' => {
                    prototypes::door(g.entity_mut(ids.new_id()), coord, DoorState::Closed);
                    prototypes::floor(g.entity_mut(ids.new_id()), coord);
                }
                '@' => {
                    let id = ids.new_id();
                    pc = Some(id);
                    prototypes::pc(g.entity_mut(id), coord);
                    prototypes::outside_floor(g.entity_mut(ids.new_id()), coord);

                    let ticket = schedule.schedule_turn(id, PC_TURN_OFFSET);
                    g.insert_schedule_ticket(id, ticket);
                }
                't' => {
                    prototypes::outside_floor(g.entity_mut(ids.new_id()), coord);
                    let id = prototypes::terror_pillar(g, ids, coord);

                    let ticket = schedule.schedule_turn(id, NPC_TURN_OFFSET);
                    g.insert_schedule_ticket(id, ticket);
                }
                _ => panic!(),
            }
            x += 1;
        }
        y += 1;
    }

    TerrainMetadata {
        width: width,
        height: height,
        pc: pc,
    }
}
