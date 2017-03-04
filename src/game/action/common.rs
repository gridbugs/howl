use rand::Rng;
use ecs::*;
use game::*;
use game::data::*;
use spatial_hash::*;
use direction::Direction;
use coord::Coord;

pub fn walk(action: &mut EcsAction, entity: EntityRef, direction: Direction) {
    let current_position = entity.position().expect("Entity missing position");
    let new_position = current_position + direction.vector();
    action.insert_position(entity.id(), new_position);
}

pub fn close(action: &mut EcsAction, entity_id: EntityId, direction: Direction) {
    action.set_close(Close::new(entity_id, direction));
}

pub fn fire_bullet(action: &mut EcsAction,
                   entity: EntityRef,
                   delta: Coord,
                   ids: &EntityIdReserver) {

    const SPEED_CELLS_PER_SEC: f64 = 40.0;

    let mut velocity = RealtimeVelocity::new(delta, SPEED_CELLS_PER_SEC);
    let firer_position = entity.position().expect("Entity missing position");
    let bullet_position = firer_position + velocity.step_in_place();

    prototypes::bullet(action.entity_mut(ids.new_id()), bullet_position, velocity);
}

pub fn realtime_velocity_move(action: &mut EcsAction, entity: EntityRef, velocity: RealtimeVelocity) {

    let current_position = entity.position().expect("Entity missing position");
    let current_velocity = entity.realtime_velocity().expect("Entity missing realtime_velocity");

    let (new_velocity, offset) = current_velocity.step();

    action.insert_realtime_velocity(entity.id(), new_velocity);
    action.insert_position(entity.id(), current_position + offset);

    action.set_action_time_ms(velocity.ms_per_cell());
}

pub fn destroy(action: &mut EcsAction, entity: EntityRef) {
    action.remove_entity(entity);
}

pub fn move_tear<R: Rng>(action: &mut EcsAction, entity_id: EntityId, ecs: &EcsCtx, spatial_hash: &SpatialHashTable, r: &mut R) {

    let mut tear_state = ecs.tear_state_borrow_mut(entity_id)
        .expect("Entity missing tear_state");

    tear_state.progress(r, 1.0);

    for (coord, cell) in izip!(spatial_hash.coord_iter(), spatial_hash.cell_iter()) {
        let tear = !tear_state.is_tear(coord);
        if cell.floor() && cell.tear() != tear {
            let floor = cell.any_floor().expect("Expected floor entity");
            if tear {
                action.insert_tear(floor);
            } else {
                action.remove_tear(floor);
            }
        }
    }
}

pub fn level_switch(action: &mut EcsAction, entity_id: EntityId, exit_id: EntityId, level_switch: LevelSwitch) {
    action.set_level_switch_action(LevelSwitchAction {
        entity_id: entity_id,
        exit_id: exit_id,
        level_switch: level_switch,
    });
}

pub fn try_level_switch(action: &mut EcsAction, entity_id: EntityId) {
    action.set_try_level_switch(entity_id);
}

pub fn projectile_collision(action: &mut EcsAction, projectile_collision: ProjectileCollision) {
    action.set_projectile_collision(projectile_collision);
    action.set_no_commit();
}

pub fn damage(action: &mut EcsAction, to_damage: EntityRef, amount: usize) {

    let mut hit_points = to_damage.hit_points().expect("Entity missing hit_points");

    hit_points.dec(amount);

    action.insert_hit_points(to_damage.id(), hit_points);
}

pub fn die(action: &mut EcsAction, entity: EntityRef) {
    if entity.contains_pc() {
        action.set_player_died();
    } else {
        let ticket = entity.schedule_ticket().expect("Entity missing schedule_ticket");
        action.set_schedule_invalidate(ticket);
        action.remove_entity(entity);
    }
}
