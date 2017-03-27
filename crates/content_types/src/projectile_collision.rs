use ecs_core::*;

#[derive(Clone, Copy, Debug)]
pub struct ProjectileCollision {
    pub projectile_id: EntityId,
    pub collider_id: EntityId,
}

impl ProjectileCollision {
    pub fn new(projectile_id: EntityId, collider_id: EntityId) -> Self {
        ProjectileCollision {
            projectile_id: projectile_id,
            collider_id: collider_id,
        }
    }
}
