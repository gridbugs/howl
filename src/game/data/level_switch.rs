use game::*;
use ecs::*;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum LevelSwitch {
    NewLevel(TerrainType),
    ExistingLevel(ExistingLevel),
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct ExistingLevel {
    pub level_id: LevelId,
    pub entrance_entity_id: EntityId,
}

#[derive(Clone, Copy, Debug)]
pub struct LevelSwitchAction {
    pub level_switch: LevelSwitch,
    pub trigger_id: EntityId,
}
