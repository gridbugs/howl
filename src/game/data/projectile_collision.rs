use ecs::*;

#[derive(Clone, Copy, Debug)]
pub struct ProjectileCollision {
    pub projectile: EntityId,
    pub collider: EntityId,
}

impl ProjectileCollision {
    pub fn new(projectile: EntityId, collider: EntityId) -> Self {
        ProjectileCollision {
            projectile: projectile,
            collider: collider,
        }
    }
}
