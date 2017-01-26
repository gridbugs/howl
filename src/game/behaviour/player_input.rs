use std::ops::Deref;
use std::cmp;

use game::*;
use behaviour::LeafResolution;
use direction::Direction;
use coord::{Coord, StraightLine};

pub fn player_input<K: KnowledgeRenderer, I: 'static + InputSource + Clone>(input_source: I) -> BehaviourLeaf<K> {
    BehaviourLeaf::new(move |input| {
        loop {
            if let Some(meta_action) = get_meta_action(input, input_source.clone()) {
                return LeafResolution::Yield(meta_action);
            }
        }
    })
}

fn get_direction<I: InputSource>(map: &ControlMap, mut input_source: I) -> Option<Direction> {
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

fn aim<R: KnowledgeRenderer, I: InputSource>(input: BehaviourInput<R>, map: &ControlMap, mut input_source: I) -> Option<Coord> {
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

        let overlay = RenderOverlay::aim_line(StraightLine::new(start, end));
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
            } else {
                break;
            }
        }
    }

    renderer.draw();
    None
}

fn display_message_log<K: KnowledgeRenderer, I: InputSource>(input: BehaviourInput<K>, mut input_source: I, map: &ControlMap) {

    let mut renderer = input.renderer.borrow_mut();
    let message_log = input.entity.message_log_borrow().unwrap();

    let mut offset = 0;
    let num_lines = renderer.display_log_num_lines();
    let num_messages = message_log.len();
    let max_offset = if num_messages > num_lines {
        num_messages - num_lines
    } else {
        0
    };

    loop {
        renderer.display_log(message_log.deref(), offset, input.language);

        if let Some(event) = input_source.next_input() {
            if let Some(control) = map.control(event) {
                match control {
                    Control::Quit |
                        Control::DisplayMessageLog => break,
                    Control::Direction(Direction::North) => {
                        offset = cmp::min(max_offset, offset + 1);
                    }
                    Control::Direction(Direction::South) => {
                        if offset > 0 {
                            offset -= 1;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    renderer.draw();
}

fn examine<K: KnowledgeRenderer, I: InputSource>(input: BehaviourInput<K>, mut input_source: I, map: &ControlMap) {


    let mut knowledge = input.entity.drawable_knowledge_borrow_mut().unwrap();
    let level_knowledge = knowledge.level_mut_or_insert_size(input.level_id,
                                                             input.spatial_hash.width(),
                                                             input.spatial_hash.height());

    let mut renderer = input.renderer.borrow_mut();
    let mut message_log = input.entity.message_log_borrow_mut().unwrap();
    let position = input.entity.position().unwrap();

    let mut cursor = position;
    let mut alternative_message = false;

    loop {

        let cell = level_knowledge.get_with_default(cursor);

        if alternative_message {
            alternative_message = false;
        } else {
            let message = if cell.last_updated() == input.action_env.id {
                MessageType::YouSee(cell.you_see())
            } else if cell.last_updated() == 0 {
                MessageType::Unseen
            } else {
                MessageType::YouRemember(cell.you_see())
            };

            message_log.add_temporary(message);
            renderer.update_log(message_log.deref(), input.language);
        }

        let overlay = RenderOverlay::examine_cursor(cursor);
        renderer.draw_with_overlay(&overlay);

        if let Some(event) = input_source.next_input() {
            if let Some(control) = map.control(event) {
                match control {
                    Control::Quit |
                        Control::Examine => break,
                    Control::Direction(direction) => {
                        cursor += direction.vector();
                    }
                    Control::Return => {
                        let message = if let Some(description) = cell.description() {
                            MessageType::Description(description)
                        } else if let Some(you_see) = cell.you_see() {
                            MessageType::YouSeeDescription(you_see)
                        } else {
                            message_log.add_temporary(MessageType::NoDescription);
                            renderer.update_log(message_log.deref(), input.language);
                            alternative_message = true;
                            continue;
                        };

                        renderer.display_message_fullscreen(message, input.language);
                        input_source.next_input();
                    }
                    _ => {}
                }
            }
        }
    }

    message_log.add_temporary(MessageType::Empty);
    renderer.update_log(message_log.deref(), input.language);
    renderer.draw();
}

fn get_meta_action<K: KnowledgeRenderer, I: InputSource>(input: BehaviourInput<K>, mut input_source: I) -> Option<MetaAction> {
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
                    Control::DisplayMessageLog => {
                        display_message_log(input, input_source, map);
                        None
                    }
                    Control::Examine => {
                        examine(input, input_source, map);
                        None
                    }
                    _ => None,
                }
            })
        })
    })
}

