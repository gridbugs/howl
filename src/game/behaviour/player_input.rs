use game::*;
use behaviour::LeafResolution;
use direction::Direction;
use coord::{Coord, StraightLine};

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
    let mut knowledge = input.entity.drawable_knowledge_borrow_mut().unwrap();
    let level_knowledge = knowledge.level_mut_or_insert_size(input.level_id,
                                                             input.spatial_hash.width(),
                                                             input.spatial_hash.height());

    let targets = level_knowledge.sort_targets(start);
    let mut target_idx = 0;

    let mut end = if !targets.is_empty() {
        targets[target_idx]
    } else {
        start
    };

    let mut renderer = input.renderer.borrow_mut();

    loop {

        let overlay = RenderOverlay {
            aim_line: Some(StraightLine::new(start, end)),
        };

        renderer.draw_with_overlay(&overlay);

        if let Some(event) = input_source.next_input() {
            if let Some(control) = map.control(event) {
                if let Some(direction) = control_to_direction(control) {
                    let next_end = end + direction.vector();
                    if renderer.contains_world_coord(next_end) {
                        end = next_end;
                    }
                } else if control == Control::NextTarget {
                    if !targets.is_empty() {
                        target_idx = (target_idx + 1) % targets.len();
                        end = targets[target_idx];
                    }
                } else if control == Control::PrevTarget {
                    if !targets.is_empty() {
                        target_idx = (target_idx + targets.len() - 1) % targets.len();
                        end = targets[target_idx];
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
                            let delta = coord - input.entity.position().unwrap();

                            MetaAction::ActionArgs(ActionArgs::FireBullet(input.entity.id(), delta))
                        })
                    }
                    Control::Wait => {
                        Some(MetaAction::ActionArgs(ActionArgs::Null))
                    }
                    Control::Quit => Some(MetaAction::External(External::Quit)),
                    Control::NextTarget => None,
                    Control::PrevTarget => None,
                }
            })
        })
    })
}

