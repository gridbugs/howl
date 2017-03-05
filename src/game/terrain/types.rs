use ecs::*;
use game::*;
use coord::Coord;

pub struct TerrainMetadata {
    pub width: usize,
    pub height: usize,
    pub start_coord: Coord,
    pub connection_report: LevelConnectionReport,
}

pub struct ParentLevelCtx<'a> {
    pub level: &'a Level,
    pub level_id: LevelId,
    pub exit_id: EntityId,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum TerrainType {
    DemoA,
}

impl TerrainType {
    pub fn generate<S: TurnScheduleQueue>(self,
                                          ids: &EntityIdReserver,
                                          rng: &GameRng,
                                          schedule: &mut S,
                                          action: &mut EcsAction,
                                          _parent: Option<ParentLevelCtx>) -> TerrainMetadata {
        match self {
            TerrainType::DemoA => generators::demo_a(ids, rng, schedule, action),
        }
    }
}
