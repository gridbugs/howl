use std::ops::Deref;
use std::cmp;

use control::*;
use game::*;
use message::*;
use content_types::*;
use ecs_core::*;
use ecs_content::*;

use behaviour::LeafResolution;
use math::{self, Direction};

pub fn player_input<K: KnowledgeRenderer, I: 'static + InputSource + Clone>(input_source: I) -> BehaviourLeaf<K> {
    BehaviourLeaf::new(move |input| {
        loop {
            if let Some(meta_action) = get_meta_action(input, input_source.clone()) {
                return LeafResolution::Yield(meta_action);
            }
        }
    })
}

fn animate_frame<K: KnowledgeRenderer>(input: &mut BehaviourInput<K>, frame: Frame) {
    animation::animate_frame(
        input.ecs,
        input.action,
        input.spatial_hash,
        input.entity_id,
        input.level_id,
        input.action_id,
        input.entity_ids,
        input.rng,
        input.pc_observer,
        input.renderer,
        input.language,
        frame
    );
}

fn next_input<K: KnowledgeRenderer, I: InputSource>(input: &mut BehaviourInput<K>, mut input_source: I) -> InputEvent {
    loop {
        match input_source.next_external() {
            ExternalEvent::Input(input_event) => return input_event,
            ExternalEvent::Frame(frame) => animate_frame(input, frame),
        }
    }
}

fn control_to_direction(control: Control) -> Option<Direction> {
    match control {
        Control::Direction(d) => Some(d),
        _ => None,
    }
}

fn display_message_log<K: KnowledgeRenderer, I: InputSource>(input: &mut BehaviourInput<K>, mut input_source: I) {

    let entity = input.ecs.entity(input.entity_id);

    let message_log = entity.borrow_message_log().unwrap();

    let mut offset = 0;
    let num_lines = input.renderer.fullscreen_log_num_rows();
    let num_messages = message_log.len();
    let max_offset = if num_messages > num_lines {
        num_messages - num_lines
    } else {
        0
    };

    loop {
        input.renderer.publish_fullscreen_log(message_log.deref(), offset, input.language);

        if let Some(control) = input.control_map.get(input_source.next_input()) {
            match control {
                Control::Pause |
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

    input.renderer.publish_all_windows(&entity, input.language);
}

fn aim<R: KnowledgeRenderer, I: InputSource>(input: &mut BehaviourInput<R>, input_source: I) -> Option<(EntityId, Direction)> {
    None
}

fn inventory<K: KnowledgeRenderer, I: InputSource>(input: &mut BehaviourInput<K>, mut input_source: I) -> Option<EntityId> {
    None
}

fn direction_to_relative_message(direction: Direction) -> MessageType {
    match direction {
        Direction::East => MessageType::Front,
        Direction::West => MessageType::Rear,
        Direction::North => MessageType::Left,
        Direction::South => MessageType::Right,
        _ => panic!(),
    }
}

fn display_status<K: KnowledgeRenderer, I: InputSource>(input: &mut BehaviourInput<K>, mut input_source: I) {
}

fn get_meta_action<K: KnowledgeRenderer, I: InputSource + Clone>(input: &mut BehaviourInput<K>, input_source: I) -> Option<MetaAction> {

    let event = next_input(input, input_source.clone());
    if event == InputEvent::Quit {
        return Some(MetaAction::External(External::Quit));
    }
    input.control_map.get(event).and_then(|control| {
        match control {
            Control::Fire => {
                aim(input, input_source).map(|(gun_id, direction)| {
                    MetaAction::ActionArgs(ActionArgs::Null)
                })
            }
            Control::Wait => {
                Some(MetaAction::ActionArgs(ActionArgs::Null))
            }
            Control::Pause => Some(MetaAction::External(External::Pause)),
            Control::DisplayMessageLog => {
                display_message_log(input, input_source);
                None
            }
            Control::Status => {
                display_status(input, input_source);
                None
            }
            _ => None,
        }
    })
}
