use ecs::*;

pub fn transform_terror_pillar_terror_fly(action: &mut EcsAction, entity: EntityRef) {

    let shadow_id = entity.shadow_entity().expect("Entity missing shadow_entity");
    action.swap_tile(entity.id(), shadow_id);
    action.swap_turn_time(entity.id(), shadow_id);
    action.swap_transformation_state(entity.id(), shadow_id);
}

pub fn transform_tree(action: &mut EcsAction, entity: EntityRef) {

    let shadow_id = entity.shadow_entity().expect("Entity missing shadow_entity");
    action.swap_transformation_state(entity.id(), shadow_id);
    action.swap_tile(entity.id(), shadow_id);
    action.swap_opacity(entity.id(), shadow_id);
}
