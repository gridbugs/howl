use ecs::entity::{EntityId, ComponentType, Component};

#[derive(Debug)]
pub enum UpdateStage {
    Propose,
    Commit,
}

#[derive(Debug)]
pub enum Update {
    SetEntityComponent {
        entity_id: EntityId,
        component_type: ComponentType,
        component_value: Component,
    }
}
