use game::*;
use game::data::*;
use ecs::*;

pub fn projectile_collision_trigger(env: RuleEnv, action: &EcsAction, reactions: &mut Vec<Reaction>) -> RuleResult {

    for (projectile_id, position) in action.position_profile().insertion_copy_iter() {

        if let Some(collider_id) = env.spatial_hash.get(position).any_projectile_collider() {

            let projectile = env.ecs.post_action_entity(projectile_id, action);

            if projectile.contains_projectile() {

                if projectile.contains_destroy_on_collision() {
                    // must happen after processing the collision
                    reactions.push(Reaction::new(ActionArgs::Destroy(projectile_id), 1));
                }

                let collision_action = ActionArgs::ProjectileCollision(ProjectileCollision::new(projectile_id, collider_id));

                return rule_consume(collision_action);
            }
        }
    }

    RULE_ACCEPT
}

pub fn projectile_collision(env: RuleEnv, action: &EcsAction, reactions: &mut Vec<Reaction>) -> RuleResult {

    if let Some(ProjectileCollision { projectile_id, collider_id }) = action.projectile_collision() {

        let projectile = env.ecs.post_action_entity(projectile_id, action);

        if let Some(damage) = projectile.projectile_damage() {
            if env.ecs.contains_complex_damage(collider_id) {
                reactions.push(Reaction::new(ActionArgs::ComplexDamage(collider_id, damage), 0));
            } else if env.ecs.contains_hit_points(collider_id) {
                reactions.push(Reaction::new(ActionArgs::Damage(collider_id, damage), 0));
            }
        }
    }

    RULE_ACCEPT
}
