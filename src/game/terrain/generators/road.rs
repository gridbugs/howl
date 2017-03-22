use std::cmp;
use std::ops::DerefMut;
use rand::Rng;
use ecs::*;
use game::*;
use game::data::*;
use game::terrain::util;
use coord::Coord;
use grid::*;
use perlin::*;
use math::*;
use direction::*;

const MAP_WIDTH_MAX: usize = 120;
const MAP_WIDTH_MIN: usize = 60;
const MAP_HEIGHT: usize = 30;
const ROAD_HEIGHT: usize = 6;
const ROAD_TOP: usize = (MAP_HEIGHT + ROAD_HEIGHT) / 2;
const ROAD_BOTTOM: usize = ROAD_TOP - ROAD_HEIGHT;
const START_COORD: Coord = Coord { x: 0, y: MAP_HEIGHT as isize / 2 };

const PERLIN_ZOOM: usize = 5;
const PERLIN_ZOOM_F: f64 = PERLIN_ZOOM as f64;
const PERLIN_MIN: f64 = 0.2;
const PERLIN_MAX: f64 = 0.5;

#[derive(Clone, Copy, PartialEq, Eq)]
enum EntityType {
    Dirt,
    Road,
    Acid,
    Goal,
    Wreck,
    Barrel,
    Letter,
    Zombie,
    Car,
    Bike,
}

const RANDOM_ENTITY_TYPES: [EntityType; 6] = [
    EntityType::Wreck,
    EntityType::Barrel,
    EntityType::Letter,
    EntityType::Zombie,
    EntityType::Car,
    EntityType::Bike,
];
const RANDOM_ENTITY_TOTAL: usize = 1000;

fn choose_random_entity<R: Rng>(types: &[EntityType], weights: &[usize], total: usize, rng: &mut R) -> Option<EntityType> {
    let mut roll = rng.gen::<usize>() % total;
    for (weight, entity_type) in izip!(weights.iter(), types.iter()) {
        if roll < *weight {
            return Some(*entity_type);
        }
        roll -= *weight;
    }

    None
}

pub fn road<S: TurnScheduleQueue>(ids: &EntityIdReserver,
                                  rng: &GameRng,
                                  schedule: &mut S,
                                  g: &mut EcsAction,
                                  difficulty: usize) -> TerrainMetadata {


    let map_width = rng.gen_usize() % (MAP_WIDTH_MAX - MAP_WIDTH_MIN) + MAP_WIDTH_MIN;
    let mut grid: StaticGrid<Vec<EntityType>> = StaticGrid::new_default(map_width, MAP_HEIGHT);

    let perlin = PerlinGrid::new(map_width / PERLIN_ZOOM, MAP_HEIGHT / PERLIN_ZOOM,
                                 PerlinWrapType::Repeat, rng.inner_mut().deref_mut());

    let random_entity_dirt_weights = [
        5, /* Wreck */
        10, /* Barrrel */
        3, /* Letter */
        20 + cmp::min(difficulty * 10, 60), /* Zombie */
        cmp::min(difficulty / 4, 4), /* Car */
        1 + cmp::min(difficulty / 2, 8), /* Bike */
    ];

    let random_entity_road_weights = [
        5, /* Wreck */
        10, /* Barrrel */
        3, /* Letter */
        10 + cmp::min(difficulty * 5, 30), /* Zombie */
        1 + cmp::min(difficulty / 3, 6), /* Car */
        cmp::min(difficulty / 3, 6), /* Bike */
    ];

    for (coord, cell_mut) in izip!(grid.coord_iter(), grid.iter_mut()) {

        let mut acid = false;
        let perlin_coord = Vector2::new(coord.x as f64 / PERLIN_ZOOM_F, coord.y as f64 / PERLIN_ZOOM_F);
        if let Some(noise) = perlin.noise(perlin_coord.x, perlin_coord.y) {
            if noise > PERLIN_MIN && noise <= PERLIN_MAX {
                acid = true;
            }
        }

        if acid {
            cell_mut.push(EntityType::Acid);
        } else if coord.y > ROAD_BOTTOM as isize && coord.y <= ROAD_TOP as isize {
            cell_mut.push(EntityType::Road);
            if let Some(entity_type) = choose_random_entity(&RANDOM_ENTITY_TYPES,
                                                            &random_entity_road_weights,
                                                            RANDOM_ENTITY_TOTAL,
                                                            rng.inner_mut().deref_mut()) {
                cell_mut.push(entity_type);
            }
        } else {
            cell_mut.push(EntityType::Dirt);
            if let Some(entity_type) = choose_random_entity(&RANDOM_ENTITY_TYPES,
                                                            &random_entity_dirt_weights,
                                                            RANDOM_ENTITY_TOTAL,
                                                            rng.inner_mut().deref_mut()) {
                cell_mut.push(entity_type);
            }
        }

        if coord.x == map_width as isize - 1 {
            cell_mut.push(EntityType::Goal);
        }
    }

    for (coord, cell) in izip!(grid.coord_iter(), grid.iter()) {
        for entity_type in cell.iter() {
            match *entity_type {
                EntityType::Dirt => {
                    prototypes::dirt(g.entity_mut(ids.new_id()), coord, rng);
                }
                EntityType::Road => {
                    prototypes::road(g.entity_mut(ids.new_id()), coord, rng);
                }
                EntityType::Acid => {
                    prototypes::acid(g.entity_mut(ids.new_id()), coord, rng);
                }
                EntityType::Goal => {
                    prototypes::goal(g.entity_mut(ids.new_id()), coord, LevelSwitch::LeaveLevel);
                }
                EntityType::Wreck => {
                    prototypes::wreck(g.entity_mut(ids.new_id()), coord, rng);
                }
                EntityType::Barrel => {
                    prototypes::barrel(g.entity_mut(ids.new_id()), coord);
                }
                EntityType::Letter => {
                    prototypes::letter(g.entity_mut(ids.new_id()), coord);
                }
                EntityType::Zombie => {
                    prototypes::dirt(g.entity_mut(ids.new_id()), coord, rng);
                    let id = ids.new_id();
                    prototypes::zombie(g.entity_mut(id), coord);
                    let turn_offset = g.get_copy_turn_offset(id).expect("Expected component turn_offset");
                    let ticket = schedule.schedule_turn(id, turn_offset);
                    g.insert_schedule_ticket(id, ticket);
                }
                EntityType::Car => {
                    prototypes::dirt(g.entity_mut(ids.new_id()), coord, rng);
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
                EntityType::Bike => {
                    prototypes::dirt(g.entity_mut(ids.new_id()), coord, rng);
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
            }
        }
    }

    util::add_management_entities(ids, schedule, g);

    TerrainMetadata {
        width: map_width,
        height: MAP_HEIGHT,
        start_coord: START_COORD,
        connection_report: LevelConnectionReport::new(),
    }
}
