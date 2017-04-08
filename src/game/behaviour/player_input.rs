use std::ops::Deref;
use std::cmp;

use rand::Rng;

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
    if frame.count() % 20 == 0 {
        for id in input.ecs.id_iter_acid_animation() {
            // don't always change every tile
            if input.rng.next_f64() > 0.5 {
                continue;
            }

            let animation = input.ecs.get_probabilistic_animation(id).expect("Entity missing probabilistic_animation");
            let tile = *animation.choose(input.rng);
            input.action.insert_tile(id, tile);
        }
    }

    *input.action_id += 1;

    input.spatial_hash.update(input.ecs, input.action, *input.action_id);
    input.ecs.commit(input.action);

    let pc = input.ecs.entity(input.entity_id);

    let mut knowledge = pc.borrow_mut_drawable_knowledge()
        .expect("PC missing drawable_knowledge");

    let level_knowledge = knowledge.level_mut_or_insert_size(input.level_id,
                                                             input.spatial_hash.width(),
                                                             input.spatial_hash.height());
    let position = pc.copy_position().expect("PC missing position");
    let vision_distance = pc.copy_vision_distance().expect("PC missing vision_distance");

    let action_env = ActionEnv::new(input.ecs, *input.action_id);
    let changed = input.pc_observer.observe(position, input.spatial_hash, vision_distance, level_knowledge, action_env);

    if changed {
        input.renderer.update_and_publish_game_window(*input.action_id, level_knowledge, position);
    }

    input.action.clear();
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

    {
        let entity = input.ecs.entity(input.entity_id);
        let mut message_log = entity.borrow_mut_message_log().expect("Expected component message_log");

        message_log.add_temporary(MessageType::ChooseDirection);
        input.renderer.update_log_buffer(message_log.deref(), input.language);
        input.renderer.draw_log();
        input.renderer.publish_all_windows(&entity, input.language);
    }

    let mut should_clear_log = true;

    let ret = input.control_map.get(next_input(input, input_source)).and_then(|control| {
        let entity = input.ecs.entity(input.entity_id);
        let mut message_log = entity.borrow_mut_message_log().expect("Expected component message_log");

        match control {
            Control::Direction(direction) => {
                let weapon_slots = entity.borrow_weapon_slots().expect("Expected component weapon_slots");
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
    });

    {
        let entity = input.ecs.entity(input.entity_id);
        let mut message_log = entity.borrow_mut_message_log().expect("Expected component message_log");
        if should_clear_log {
            message_log.add_temporary(MessageType::Empty);
        }
        input.renderer.update_log_buffer(message_log.deref(), input.language);
        input.renderer.draw_log();
        input.renderer.publish_all_windows(&entity, input.language);
    }

    ret
}

fn inventory<K: KnowledgeRenderer, I: InputSource>(input: &mut BehaviourInput<K>, mut input_source: I) -> Option<EntityId> {

    let entity = input.ecs.entity(input.entity_id);

    let mut menu = SelectMenu::new();
    for entity_id in entity.borrow_inventory().expect("Missing component inventory").iter() {
        let name = input.ecs.get_copy_name(entity_id).expect("Missing component name");
        let menu_message = MenuMessageType::Name(name);
        menu.push(SelectMenuItem::new(menu_message, entity_id));
    }

    let capacity = entity.copy_inventory_capacity().expect("Missing component inventory_capacity");
    let size = entity.borrow_inventory().expect("Missing component inventory").len();

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
        Some(&entity)).run_can_escape().map(|(id, _)| id);

    input.renderer.publish_all_windows(&entity, input.language);

    ret
}

fn try_consume_item<K: KnowledgeRenderer>(input: &mut BehaviourInput<K>, item_id: EntityId) -> Option<ActionArgs> {
    let entity = input.ecs.entity(input.entity_id);
    let speed = entity.copy_current_speed().expect("Missing component current_speed");
    if speed == 0 {
        let mut inv = entity.borrow_mut_inventory().expect("Missing component inventory");
        inv.remove(item_id);
        return Some(ActionArgs::Consume(input.entity_id, item_id));
    }
    let mut message_log = entity.borrow_mut_message_log().expect("Expected component message_log");
    message_log.add_temporary(MessageType::MustBeStopped);
    input.renderer.update_log_buffer(message_log.deref(), input.language);
    input.renderer.draw_log();
    input.renderer.publish_all_windows(&entity, input.language);
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
    let entity = input.ecs.entity(input.entity_id);
    let weapon_slots = entity.borrow_weapon_slots().expect("Expected component weapon_slots");

    let mut message = Message::new();

    for d in math::cardinal_direction_iter() {
        let m = direction_to_relative_message(d);
        input.language.translate(m, &mut message);
        message.push(MessagePart::plain(": "));

        if let Some(weapon_id) = weapon_slots.get(d) {
            let name = input.ecs.get_copy_name(*weapon_id).expect("Expected component name");
            input.language.translate(MessageType::Name(name), &mut message);
        } else {
            input.language.translate(MessageType::EmptyWeaponSlot, &mut message);
        }
        message.push(MessagePart::Newline);
    }

    display_message_scrolling(input.renderer, &mut input_source, &message, true);
    input.renderer.publish_all_windows(&entity, input.language);
}

fn get_meta_action<K: KnowledgeRenderer, I: InputSource + Clone>(input: &mut BehaviourInput<K>, input_source: I) -> Option<MetaAction> {

    let event = next_input(input, input_source.clone());
    if event == InputEvent::Quit {
        return Some(MetaAction::External(External::Quit));
    }
    input.control_map.get(event).and_then(|control| {
        match control {
            Control::Direction(Direction::East) => Some(MetaAction::ActionArgs(ActionArgs::ChangeSpeed(input.entity_id, ChangeSpeed::Accelerate))),
            Control::Direction(Direction::West) => Some(MetaAction::ActionArgs(ActionArgs::ChangeSpeed(input.entity_id, ChangeSpeed::Decelerate))),
            Control::Direction(Direction::North) => Some(MetaAction::ActionArgs(ActionArgs::Steer(input.entity_id, SteerDirection::Up))),
            Control::Direction(Direction::South) => Some(MetaAction::ActionArgs(ActionArgs::Steer(input.entity_id, SteerDirection::Down))),
            Control::Fire => {
                aim(input, input_source).map(|(gun_id, direction)| {
                    MetaAction::ActionArgs(ActionArgs::FireGun {
                        gun_id: gun_id,
                        shooter_id: input.entity_id,
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
}
