use ecs::EntityId;

pub struct BurstFire {
    prototype: EntityId,
    count: u64,
    period: u64,
}
