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

fn get_direction<I: InputSource>(map: &ControlMap, mut input_source: I) -> Option<Direction> {
    input_source.next_input().and_then(|event| {
        map.get(event).and_then(|control| {
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

fn display_message_log<K: KnowledgeRenderer, I: InputSource>(input: &mut BehaviourInput<K>, mut input_source: I) {

    let message_log = input.entity.borrow_message_log().unwrap();

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

        if let Some(event) = input_source.next_input() {
            if let Some(control) = input.control_map.get(event) {
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
    }

    input.renderer.publish_all_windows(&input.entity, input.language);
}

fn aim<R: KnowledgeRenderer, I: InputSource>(input: &mut BehaviourInput<R>, mut input_source: I) -> Option<(EntityId, Direction)> {

    let mut message_log = input.entity.borrow_mut_message_log().expect("Expected component message_log");

    message_log.add_temporary(MessageType::ChooseDirection);
    input.renderer.update_log_buffer(message_log.deref(), input.language);
    input.renderer.draw_log();
    input.renderer.publish_all_windows(&input.entity, input.language);

    let mut should_clear_log = true;

    let ret = input_source.next_input().and_then(|event| {
        input.control_map.get(event).and_then(|control| {
            match control {
                Control::Direction(direction) => {
                    let weapon_slots = input.entity.borrow_weapon_slots().expect("Expected component weapon_slots");
                    if let Some(weapon) = weapon_slots.get(direction) {
                        Some((*weapon, direction))
                    } else {
                        message_log.add_temporary(MessageType::EmptyWeaponSlotMessage);
                        should_clear_log = false;
                        None
                    }
                }
                _ => None,
            }
        })
    });

    if should_clear_log {
        message_log.add_temporary(MessageType::Empty);
    }
    input.renderer.update_log_buffer(message_log.deref(), input.language);
    input.renderer.draw_log();
    input.renderer.publish_all_windows(&input.entity, input.language);

    ret
}

fn inventory<K: KnowledgeRenderer, I: InputSource>(input: &mut BehaviourInput<K>, mut input_source: I) -> Option<EntityId> {

    let mut menu = SelectMenu::new();
    for entity_id in input.entity.borrow_inventory().expect("Missing component inventory").iter() {
        let name = input.entity.ecs().get_copy_name(entity_id).expect("Missing component name");
        let menu_message = MenuMessageType::Name(name);
        menu.push(SelectMenuItem::new(menu_message, entity_id));
    }

    let capacity = input.entity.copy_inventory_capacity().expect("Missing component inventory_capacity");
    let size = input.entity.borrow_inventory().expect("Missing component inventory").len();

    let ret = SelectMenuOperation::new(
        input.renderer,
        &mut input_source,
        Some(MessageType::Inventory {
            capacity: capacity,
            size: size,
        }),
        input.language,
        menu,
        None,
        Some(&input.entity)).run_can_escape().map(|(id, _)| id);

    input.renderer.publish_all_windows(&input.entity, input.language);

    ret
}

fn try_consume_item<K: KnowledgeRenderer>(input: &mut BehaviourInput<K>, item_id: EntityId) -> Option<ActionArgs> {
    let speed = input.entity.copy_current_speed().expect("Missing component current_speed");
    if speed == 0 {
        let mut inv = input.entity.borrow_mut_inventory().expect("Missing component inventory");
        inv.remove(item_id);
        return Some(ActionArgs::Consume(input.entity.id(), item_id));
    }
    let mut message_log = input.entity.borrow_mut_message_log().expect("Expected component message_log");
    message_log.add_temporary(MessageType::MustBeStopped);
    input.renderer.update_log_buffer(message_log.deref(), input.language);
    input.renderer.draw_log();
    input.renderer.publish_all_windows(&input.entity, input.language);
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
    let weapon_slots = input.entity.borrow_weapon_slots().expect("Expected component weapon_slots");

    let mut message = Message::new();

    for d in math::cardinal_direction_iter() {
        let m = direction_to_relative_message(d);
        input.language.translate(m, &mut message);
        message.push(MessagePart::plain(": "));

        if let Some(weapon_id) = weapon_slots.get(d) {
            let name = input.entity.ecs().get_copy_name(*weapon_id).expect("Expected component name");
            input.language.translate(MessageType::Name(name), &mut message);
        } else {
            input.language.translate(MessageType::EmptyWeaponSlot, &mut message);
        }
        message.push(MessagePart::Newline);
    }

    display_message_scrolling(input.renderer, &mut input_source, &message, true);
    input.renderer.publish_all_windows(&input.entity, input.language);
}

fn get_meta_action<K: KnowledgeRenderer, I: InputSource>(input: &mut BehaviourInput<K>, mut input_source: I) -> Option<MetaAction> {

    input_source.next_input().and_then(|event| {
        if event == InputEvent::Quit {
            return Some(MetaAction::External(External::Quit));
        }
        input.control_map.get(event).and_then(|control| {
            match control {
                Control::Direction(Direction::East) => Some(MetaAction::ActionArgs(ActionArgs::ChangeSpeed(input.entity.id(), ChangeSpeed::Accelerate))),
                Control::Direction(Direction::West) => Some(MetaAction::ActionArgs(ActionArgs::ChangeSpeed(input.entity.id(), ChangeSpeed::Decelerate))),
                Control::Direction(Direction::North) => Some(MetaAction::ActionArgs(ActionArgs::Steer(input.entity.id(), SteerDirection::Up))),
                Control::Direction(Direction::South) => Some(MetaAction::ActionArgs(ActionArgs::Steer(input.entity.id(), SteerDirection::Down))),
                Control::Fire => {
                    aim(input, input_source).map(|(gun_id, direction)| {
                        MetaAction::ActionArgs(ActionArgs::FireGun {
                            gun_id: gun_id,
                            shooter_id: input.entity.id(),
                            direction: direction,
                        })
                    })
                }
                Control::Inventory => {
                    inventory(input, input_source).and_then(|item| try_consume_item(input, item)).map(MetaAction::ActionArgs)
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
    })
}
