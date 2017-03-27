use rand::Rng;

use ecs_core::*;
use ecs_content::*;
use game::*;
use math::Coord;

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
    Road,
}

impl TerrainType {
    pub fn generate<S: TurnScheduleQueue, R: Rng>(self,
                                          ids: &EntityIdReserver,
                                          r: &mut R,
                                          schedule: &mut S,
                                          action: &mut EcsAction,
                                          _parent: Option<ParentLevelCtx>,
                                          difficulty: usize) -> TerrainMetadata {
        match self {
            TerrainType::DemoA => generators::demo_a(ids, r, schedule, action),
            TerrainType::Road => generators::road(ids, r, schedule, action, difficulty),
        }
    }
}
