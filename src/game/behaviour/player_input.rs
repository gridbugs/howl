use game::{BehaviourLeaf, ActionArgs, MetaAction, Control};
use behaviour::LeafResolution;
use ecs::EntityRef;
use direction::Direction;
use frontends::ansi;
use rustty::Event;

pub fn ansi_player_input(input_source: ansi::InputSource) -> BehaviourLeaf {
    BehaviourLeaf::new(move |input| {
        loop {
            if let Some(event) = input_source.get_event() {
                if let Some(meta_action) = event_to_meta_action(input.entity, event) {
                    return LeafResolution::Yield(meta_action);
                }
            }
        }
    })
}

fn event_to_meta_action(entity: EntityRef, event: Event) -> Option<MetaAction> {
    match event {
        Event::Char('q') | Event::Char('Q') => Some(MetaAction::Control(Control::Quit)),
        Event::Up => Some(MetaAction::ActionArgs(ActionArgs::Walk(entity.id(), Direction::North))),
        Event::Down => Some(MetaAction::ActionArgs(ActionArgs::Walk(entity.id(), Direction::South))),
        Event::Left => Some(MetaAction::ActionArgs(ActionArgs::Walk(entity.id(), Direction::West))),
        Event::Right => Some(MetaAction::ActionArgs(ActionArgs::Walk(entity.id(), Direction::East))),
        _ => None,
    }
}
