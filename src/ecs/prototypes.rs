use math::Coord;
use frontends::ansi;
use direction::Direction;

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
    entity.insert_outside();

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
    entity.insert_vision_distance(8);
    entity.insert_door_opener();
    entity.insert_control_map(ControlMap::new_default());
    entity.insert_pc();
    entity.insert_walk_delay(10);

    entity
}

pub fn dog<E: EntityPopulate>(mut entity: E, position: Coord) -> E {
    entity.insert_position(position);
    entity.insert_ansi_tile(ansi::foreground('d', ansi::colours::YELLOW, ansi::styles::BOLD));
    entity.insert_tile_depth(2);
    entity.insert_collider();
    entity.insert_behaviour_state(BehaviourState::new());
    entity.insert_behaviour_type(BehaviourType::SimpleNpc);
    entity.insert_vision_distance(8);
    entity.insert_simple_npc_knowledge(SimpleNpcKnowledge::new());
    entity.insert_path_traverse(PathTraverse::new());
    entity.insert_door_opener();
    entity.insert_walk_delay(15);

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

pub fn bullet<E: EntityPopulate>(mut entity: E, position: Coord, direction: Direction) -> E {

    const SPEED_CELLS_PER_SEC: f64 = 40.0;

    entity.insert_position(position);
    entity.insert_realtime_axis_velocity(RealtimeAxisVelocity::from_cells_per_sec(
            SPEED_CELLS_PER_SEC, direction));
    entity.insert_destroy_on_collision();
    entity.insert_ansi_tile(ansi::foreground('*', ansi::colours::RED, ansi::styles::NONE));
    entity.insert_tile_depth(1);

    entity
}

pub fn clouds<E: EntityPopulate>(mut entity: E, width: usize, height: usize) -> E {

    entity.insert_cloud_state(CloudState::new(width, height));
    entity.insert_behaviour_state(BehaviourState::new());
    entity.insert_behaviour_type(BehaviourType::Clouds);

    entity
}
