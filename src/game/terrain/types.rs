use rand::Rng;
use ecs_core::*;
use engine_defs::*;
use ecs_content::*;
use game::*;
use math::Coord;
use content_types::TerrainType;

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

pub fn generate_terrain<S: TurnScheduleQueue, R: Rng>(terrain: TerrainType,
                                                      ids: &EntityIdReserver,
                                                      r: &mut R,
                                                      schedule: &mut S,
                                                      action: &mut EcsAction,
                                                      _parent: Option<ParentLevelCtx>,
                                                      difficulty: usize) -> TerrainMetadata {
    match terrain {
        TerrainType::DemoA => generators::demo_a(ids, r, schedule, action),
        TerrainType::Road => generators::road(ids, r, schedule, action, difficulty),
    }
}
