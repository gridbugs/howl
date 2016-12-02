use math::Coord;
use frontends::ansi;

use ecs::EntityPopulate;
use game::*;
use game::data::*;

pub fn wall<E: EntityPopulate>(mut entity: E, position: Coord) -> E {
    entity.insert_position(position);
    entity.insert_opacity(1.0);
    entity.insert_solid();

    entity.insert_ansi_tile(ansi::ComplexTile::Wall {
        front: ansi::SimpleTile::Full {
            ch: '▄',
            fg: ansi::colours::YELLOW,
            bg: ansi::colours::GREY,
            style: ansi::styles::NONE,
        },
        back: ansi::SimpleTile::Foreground('█', ansi::colours::GREY, ansi::styles::NONE),
    });
    entity.insert_tile_depth(1);

    entity
}

pub fn tree<E: EntityPopulate>(mut entity: E, position: Coord) -> E {
    entity.insert_position(position);
    entity.insert_opacity(0.4);
    entity.insert_solid();

    entity.insert_ansi_tile(ansi::foreground('&', ansi::colours::GREEN, ansi::styles::NONE));
    entity.insert_tile_depth(1);

    entity
}

pub fn floor<E: EntityPopulate>(mut entity: E, position: Coord) -> E {
    entity.insert_position(position);
    entity.insert_ansi_tile(ansi::full('.', ansi::colours::WHITE, ansi::colours::DARK_GREY, ansi::styles::NONE));
    entity.insert_tile_depth(0);

    entity
}

pub fn outside_floor<E: EntityPopulate>(mut entity: E, position: Coord) -> E {
    entity.insert_position(position);
    entity.insert_ansi_tile(ansi::full('.', ansi::colours::WHITE, ansi::colours::DARK_GREY, ansi::styles::NONE));
    entity.insert_tile_depth(0);

    entity
}

pub fn pc<E: EntityPopulate>(mut entity: E, position: Coord) -> E {
    entity.insert_position(position);
    entity.insert_ansi_tile(ansi::foreground('@', ansi::colours::WHITE, ansi::styles::BOLD));
    entity.insert_tile_depth(2);
    entity.insert_collider();
    entity.insert_behaviour_state(BehaviourState::new());
    entity.insert_behaviour_type(BehaviourType::AnsiPlayerInput);
    entity.insert_ansi_drawable_knowledge(AnsiDrawableKnowledge::new());
    entity.insert_vision_distance(20);
    entity.insert_door_opener();

    entity
}

pub fn door<E: EntityPopulate>(mut entity: E, position: Coord, state: DoorState) -> E {
    entity.insert_position(position);

    if state.is_open() {
        entity.insert_ansi_tile(ansi::full('-', ansi::colours::WHITE, ansi::colours::DARK_GREY, ansi::styles::NONE));
        entity.insert_opacity(0.0);
    } else {
        entity.insert_ansi_tile(ansi::full('+', ansi::colours::WHITE, ansi::colours::DARK_GREY, ansi::styles::NONE));
        entity.insert_solid();
        entity.insert_opacity(1.0);
    }
    entity.insert_tile_depth(1);
    entity.insert_door_state(state);

    entity
}
