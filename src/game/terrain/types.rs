use ecs::*;
use game::*;
use coord::Coord;

#[derive(Clone, Copy, Debug)]
pub struct TerrainMetadata {
    pub width: usize,
    pub height: usize,
    pub start_coord: Coord,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum TerrainType {
    DemoA,
    DemoB,
}

impl TerrainType {
    pub fn generate<S: TurnScheduleQueue>(self,
                                          ids: &EntityIdReserver,
                                          rng: &GameRng,
                                          schedule: &mut S,
                                          action: &mut EcsAction) -> TerrainMetadata {
        match self {
            TerrainType::DemoA => generators::demo_a(ids, rng, schedule, action),
            TerrainType::DemoB => generators::demo_b(ids, rng, schedule, action),
        }
    }
}
