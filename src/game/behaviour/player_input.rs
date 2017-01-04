use game::*;
use ecs::EntityRef;
use behaviour::LeafResolution;
use direction::Direction;
use coord::{Coord, StraightLine, Rect, InfiniteAccumulatingLineState};

pub fn player_input(input_source: InputSourceRef) -> BehaviourLeaf {
    BehaviourLeaf::new(move |input| {
        loop {
            if let Some(meta_action) = get_meta_action(input, input_source) {
                return LeafResolution::Yield(meta_action);
            }
        }
    })
}

fn get_direction(map: &ControlMap, input_source: InputSourceRef) -> Option<Direction> {
    input_source.next_input().and_then(|event| {
        map.control(event).and_then(|control| {
            control_to_direction(control)
        })
    })
}

fn control_to_direction(control: Control) -> Option<Direction> {
    match control {
        Control::Direction(d) => Some(d),
        _ => None,
    }
}

fn aim(input: BehaviourInput, map: &ControlMap, input_source: InputSourceRef) -> Option<Coord> {
    let start = input.entity.position().unwrap();
    let mut end = start;

    let mut renderer = input.renderer.borrow_mut();

    loop {

        let range = distance_to_range(start.square_distance(end));

        let overlay = RenderOverlay {
            aim_line: Some(AimLine {
                line: StraightLine::new(start, end),
                range: range,
            }),
        };

        renderer.draw_with_overlay(&overlay);

        if let Some(event) = input_source.next_input() {
            if let Some(control) = map.control(event) {
                if let Some(direction) = control_to_direction(control) {
                    let next_end = end + direction.vector();
                    if renderer.contains_world_coord(next_end) {
                        end = next_end;
                    }
                } else if control == Control::Fire {
                    renderer.draw();
                    return Some(end);
                } else {
                    break;
                }
            }
        }
    }

    renderer.draw();
    None
}

fn distance_to_range(distance: usize) -> RangeType {
    match distance {
        0...2 => RangeType::ShortRange,
        2...6 => RangeType::NormalRange,
        6...12 => RangeType::LongRange,
        _ => RangeType::OutOfRange,
    }
}

fn get_chance_to_hit(_entity: EntityRef, _range: RangeType) -> f64 {
    0.75
}

fn get_hit_coord(input: BehaviourInput, coord: Coord) -> Coord {
    let range = distance_to_range(input.entity.position().unwrap().square_distance(coord));
    let hit_chance = get_chance_to_hit(input.entity, range);
    let radius = input.rng.count_failures(hit_chance, 2);

    if radius == 0 {
        coord
    } else {
        let rect = Rect::new_centred_square(coord, radius).unwrap();
        rect.border_get(input.rng.gen_usize_below(rect.border_count())).unwrap()
    }
}

fn get_fire_delta(input: BehaviourInput, coord: Coord) -> Coord {
    let hit_coord = get_hit_coord(input, coord);
    hit_coord - input.entity.position().unwrap()
}

fn get_meta_action(input: BehaviourInput, input_source: InputSourceRef) -> Option<MetaAction> {
    input_source.next_input().and_then(|event| {
        input.entity.control_map().and_then(|map| {
            map.control(event).and_then(|control| {
                match control {
                    Control::Direction(d) => Some(MetaAction::ActionArgs(ActionArgs::Walk(input.entity.id(), d))),
                    Control::Close => {
                        get_direction(map, input_source).map(|d| MetaAction::ActionArgs(ActionArgs::Close(input.entity.id(), d)))
                    }
                    Control::Fire => {
                        aim(input, map, input_source).map(|coord| {
                            let delta = get_fire_delta(input, coord);

                            MetaAction::ActionArgs(ActionArgs::FireBullet(input.entity.id(), delta))
                        })
                    }
                    Control::Wait => {
                        Some(MetaAction::ActionArgs(ActionArgs::Null))
                    }
                    Control::Quit => Some(MetaAction::External(External::Quit)),
                }
            })
        })
    })
}

