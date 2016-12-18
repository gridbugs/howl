use ecs::*;
use game::*;

#[derive(Clone, Copy)]
pub enum TransformationType {
    TerrorPillarTerrorFly,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TransformationState {
    Real,
    Other,
}

impl TransformationType {
    pub fn to_action_args(self, entity_id: EntityId) -> ActionArgs {
        match self {
            TransformationType::TerrorPillarTerrorFly => {
                ActionArgs::TransformTerrorPillarTerrorFly(entity_id)
            }
        }
    }
}
