use ecs::*;
use game::*;
use coord::Coord;

#[derive(Clone, Copy, Debug)]
pub struct TerrainMetadata {
    pub width: usize,
    pub height: usize,
    pub start_coord: Coord,
}

#[derive(Clone, Copy, Debug)]
pub enum TerrainType {
    Demo,
}

impl TerrainType {
    pub fn generate<S: TurnScheduleQueue>(self,
                                          ids: &EntityIdReserver,
                                          rng: &GameRng,
                                          schedule: &mut S,
                                          action: &mut EcsAction) -> TerrainMetadata {
        match self {
            TerrainType::Demo => generators::demo(ids, rng, schedule, action),
        }
    }
}
