use math::{Coord, Vector2};
use direction::Direction;

use ecs::*;
use game::*;
use game::data::*;

pub fn wall<E: EntityPopulate>(mut entity: E, position: Coord) -> E {
    entity.insert_position(position);
    entity.insert_opacity(1.0);
    entity.insert_solid();

    entity.insert_tile(TileType::Wall);

    entity.insert_tile_depth(1);

    entity
}

pub fn tree(action: &mut EcsAction, ids: &EntityIdReserver, position: Coord) -> EntityId {

    let shadow_id = {
        let mut entity = action.entity_mut(ids.new_id());

        entity.insert_tile(TileType::DeadTree);

        entity.insert_transformation_state(TransformationState::Other);
        entity.insert_opacity(0.0);

        entity.id()
    };

    let mut entity = action.entity_mut(ids.new_id());
    entity.insert_position(position);
    entity.insert_opacity(0.6);
    entity.insert_solid();

    entity.insert_tile(TileType::Tree);

    entity.insert_tile_depth(1);
    entity.insert_shadow_entity(shadow_id);
    entity.insert_transformation_state(TransformationState::Real);
    entity.insert_transformation_type(TransformationType::Tree);
    entity.insert_transform_on_moon_change();

    entity.id()
}

pub fn floor<E: EntityPopulate>(mut entity: E, position: Coord) -> E {
    entity.insert_position(position);

    entity.insert_tile(TileType::Floor);

    entity.insert_tile_depth(0);

    entity
}

pub fn outside_floor<E: EntityPopulate>(mut entity: E, position: Coord) -> E {
    entity.insert_position(position);

    entity.insert_tile(TileType::Ground);

    entity.insert_tile_depth(0);
    entity.insert_outside();

    entity
}

pub fn pc<E: EntityPopulate>(mut entity: E, position: Coord) -> E {
    entity.insert_position(position);

    entity.insert_tile(TileType::Player);

    entity.insert_tile_depth(2);
    entity.insert_collider();
    entity.insert_behaviour_state(BehaviourState::new());
    entity.insert_behaviour_type(BehaviourType::PlayerInput);
    entity.insert_drawable_knowledge(DrawableKnowledge::new());
    entity.insert_vision_distance(16);
    entity.insert_door_opener();
    entity.insert_control_map(ControlMap::new_default());
    entity.insert_pc();
    entity.insert_turn_time(TURN_DURATION_BASE);
    entity.insert_should_render();

    entity
}

pub fn terror_pillar(action: &mut EcsAction, ids: &EntityIdReserver, position: Coord) -> EntityId {

    let shadow_id = {
        let mut entity = action.entity_mut(ids.new_id());

        entity.insert_tile(TileType::TerrorFly);

        entity.insert_turn_time(TURN_DURATION_BASE / 2);
        entity.insert_transformation_state(TransformationState::Other);

        entity.id()
    };

    let mut entity = action.entity_mut(ids.new_id());
    entity.insert_position(position);

    entity.insert_tile(TileType::TerrorPillar);

    entity.insert_tile_depth(2);
    entity.insert_collider();
    entity.insert_behaviour_state(BehaviourState::new());
    entity.insert_behaviour_type(BehaviourType::SimpleNpc);
    entity.insert_vision_distance(8);
    entity.insert_simple_npc_knowledge(SimpleNpcKnowledge::new());
    entity.insert_path_traverse(PathTraverse::new());
    entity.insert_turn_time(TURN_DURATION_BASE * 2);
    entity.insert_shadow_entity(shadow_id);
    entity.insert_transformation_type(TransformationType::TerrorPillarTerrorFly);
    entity.insert_transformation_state(TransformationState::Real);
    entity.insert_transform_on_moon_change();

    entity.id()
}


pub fn door<E: EntityPopulate>(mut entity: E, position: Coord, state: DoorState) -> E {
    entity.insert_position(position);

    if state.is_open() {
        entity.insert_tile(TileType::OpenDoor);
        entity.insert_opacity(0.0);
    } else {
        entity.insert_tile(TileType::ClosedDoor);
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

    entity.insert_tile(TileType::Bullet);

    entity.insert_tile_depth(1);

    entity
}

pub fn clouds<E: EntityPopulate>(mut entity: E, width: usize, height: usize) -> E {

    const PERLIN_ZOOM: f64 = 0.05;
    const PERLIN_MIN: f64 = -0.1;
    const PERLIN_MAX: f64 = 0.1;
    const SCROLL_RATE: Vector2<f64> = Vector2 { x: 0.05, y: 0.02 };
    const MUTATE_RATE: f64 = 0.01;

    entity.insert_cloud_state(CloudState::new(width, height, PERLIN_ZOOM,
                                              PERLIN_MIN, PERLIN_MAX,
                                              SCROLL_RATE, MUTATE_RATE));
    entity.insert_behaviour_state(BehaviourState::new());
    entity.insert_behaviour_type(BehaviourType::Clouds);
    entity.insert_turn_time(TURN_DURATION_BASE);

    entity
}
