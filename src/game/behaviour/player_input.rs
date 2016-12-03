use game::*;
use frontends::*;
use behaviour::LeafResolution;
use ecs::EntityRef;
use direction::Direction;

pub fn ansi_player_input(input_source: ansi::AnsiInputSource) -> BehaviourLeaf {
    BehaviourLeaf::new(move |input| {
        loop {
            if let Some(meta_action) = get_meta_action(input.entity, &input_source) {
                return LeafResolution::Yield(meta_action);
            }
        }
    })
}

fn get_direction<I: InputSource>(map: &ControlMap, input: &I) -> Option<Direction> {
    input.next_input().and_then(|event| {
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

fn get_meta_action<I: InputSource>(entity: EntityRef, input: &I) -> Option<MetaAction> {
    input.next_input().and_then(|event| {
        entity.control_map().and_then(|map| {
            map.control(event).and_then(|control| {
                match control {
                    Control::Direction(d) => Some(MetaAction::ActionArgs(ActionArgs::Walk(entity.id(), d))),
                    Control::Close => {
                        get_direction(map, input).map(|d| MetaAction::ActionArgs(ActionArgs::Close(entity.id(), d)))
                    }
                    Control::Fire => {
                        get_direction(map, input).map(|d| MetaAction::ActionArgs(ActionArgs::FireBullet(entity.id(), d)))
                    }
                    Control::Quit => Some(MetaAction::External(External::Quit)),
                }
            })
        })
    })
}

