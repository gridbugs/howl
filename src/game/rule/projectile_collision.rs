use game::*;
use game::data::*;
use ecs::*;

pub fn projectile_collision(env: RuleEnv, action: &EcsAction, reactions: &mut Vec<Reaction>) -> RuleResult {

    for (projectile_id, position) in action.position().insertion_copy_iter() {

        if let Some(collider_id) = env.spatial_hash.get(position).any_projectile_collider() {

            if env.ecs.contains_projectile(projectile_id) {

                reactions.push(Reaction::new(ActionArgs::ProjectileCollision(
                            ProjectileCollision::new(projectile_id, collider_id)), 0));

                if env.ecs.contains_destroy_on_collision(projectile_id) {
                    // must happen after processing the collision
                    reactions.push(Reaction::new(ActionArgs::Destroy(projectile_id), 1));
                }

                return RULE_REJECT;
            }
        }

    }

    RULE_ACCEPT
}
