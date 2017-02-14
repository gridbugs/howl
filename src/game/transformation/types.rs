use ecs::*;
use game::*;

#[derive(Clone, Copy, RustcEncodable, RustcDecodable)]
pub enum TransformationType {
    TerrorPillarTerrorFly,
    Tree,
}

#[derive(Clone, Copy, PartialEq, Eq, RustcEncodable, RustcDecodable)]
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
            TransformationType::Tree => {
                ActionArgs::TransformTree(entity_id)
            }
        }
    }
}
