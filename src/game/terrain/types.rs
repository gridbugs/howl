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

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum TerrainType {
    DemoA,
    DemoB,
    DemoC,
}

impl TerrainType {
    pub fn generate<S: TurnScheduleQueue>(self,
                                          ids: &EntityIdReserver,
                                          rng: &GameRng,
                                          schedule: &mut S,
                                          action: &mut EcsAction,
                                          parent: Option<ParentLevelCtx>) -> TerrainMetadata {
        match self {
            TerrainType::DemoA => generators::demo_a(ids, rng, schedule, action),
            TerrainType::DemoB => generators::demo_b(ids, rng, schedule, action),
            TerrainType::DemoC => generators::demo_c(ids, rng, schedule, action, parent.expect("Expected parent level")),
        }
    }
}
