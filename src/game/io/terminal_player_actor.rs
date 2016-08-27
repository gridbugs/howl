use game::{
    EntityId,
    Entity,
    EntityTable,
    MetaAction,
    UpdateSummary,
};
use game::ComponentType as CType;

use game::actions;
use game::components::DoorState;

use grid::Grid;
use rustty::Event;
use terminal::window_manager::InputSource;
use geometry::direction::Direction;

const ETX: char = '\u{3}';

pub fn act<'a>(input_source: &InputSource<'a>,
                       entity_id: EntityId,
                       entities: &EntityTable)
    -> Option<MetaAction>
{
    let entity = entities.get(entity_id);

    if let Some(event) = input_source.get_event() {
        if let Some(direction) = event_to_direction(event) {
            Some(MetaAction::Update(actions::walk(entity, direction)))
        } else {
            if let Some(update) = event_to_action(event, entity, entities) {
                Some(MetaAction::Update(update))
            } else {
                event_to_meta_action(event, entity, entities)
            }
        }
    } else {
        None
    }
}

fn event_to_direction(event: Event) -> Option<Direction> {
    match event {
        // Arrow keys
        Event::Up => Some(Direction::North),
        Event::Down => Some(Direction::South),
        Event::Right => Some(Direction::East),
        Event::Left => Some(Direction::West),

        // Vi keys
        Event::Char('k') => Some(Direction::North),
        Event::Char('j') => Some(Direction::South),
        Event::Char('l') => Some(Direction::East),
        Event::Char('h') => Some(Direction::West),
        Event::Char('y') => Some(Direction::NorthWest),
        Event::Char('u') => Some(Direction::NorthEast),
        Event::Char('b') => Some(Direction::SouthWest),
        Event::Char('n') => Some(Direction::SouthEast),
        _ => None,
    }
}

fn close_door(entity: &Entity, entities: &EntityTable) -> Option<UpdateSummary> {
    let level = entities.get(entity.on_level().unwrap());
    let sh = level.level_spacial_hash().unwrap();

    for cell in sh.grid.some_nei_iter(entity.position().unwrap()) {
        if cell.has(CType::Door) {
            for e in entities.id_set_iter(&cell.entities) {
                if let Some(DoorState::Open) = e.door_state() {
                    return Some(actions::close_door(e.id()));
                }
            }
        }
    }

    None
}

fn event_to_action(event: Event, entity: &Entity, entities: &EntityTable) -> Option<UpdateSummary> {
    match event {
        Event::Char('f') => Some(actions::fire_bullet(entity, Direction::North, entities)),
        _ => None,
    }
}

fn event_to_meta_action(event: Event, entity: &Entity, entities: &EntityTable) -> Option<MetaAction> {
    match event {
        Event::Char(ETX) => Some(MetaAction::Quit),
        Event::Char('q') => Some(MetaAction::Quit),
        Event::Char('c') => close_door(entity, entities).map(|u| MetaAction::Update(u)),
        _ => None,
    }
}
