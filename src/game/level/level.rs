use std::slice;

use game::*;
use ecs::*;
use spatial_hash::*;
use util::*;

pub struct Level {
    pub ecs: EcsCtx,
    pub spatial_hash: SpatialHashTable,
    pub turn_schedule: TurnSchedule,
}

#[derive(Serialize, Deserialize)]
pub struct SerializableLevel {
    ecs: SerializableEcsCtx,
    spatial_hash: SpatialHashTable,
    turn_schedule: SerializableSchedule<EntityId>,
}

impl From<Level> for SerializableLevel {
    fn from(level: Level) -> Self {
        let Level { ecs, spatial_hash, turn_schedule } = level;
        SerializableLevel {
            ecs: SerializableEcsCtx::from(ecs),
            spatial_hash: spatial_hash,
            turn_schedule: SerializableSchedule::from(turn_schedule),
        }
    }
}

impl From<SerializableLevel> for Level {
    fn from(level: SerializableLevel) -> Self {
        let SerializableLevel { ecs, spatial_hash, turn_schedule } = level;
        Level {
            ecs: EcsCtx::from(ecs),
            spatial_hash: spatial_hash,
            turn_schedule: TurnSchedule::from(turn_schedule),
        }
    }
}

impl Level {
    pub fn new_with_entity(terrain: TerrainType,
                       entity_id: EntityId,
                       action: &mut EcsAction,
                       ids: &EntityIdReserver,
                       rng: &GameRng,
                       action_id: u64,
                       parent: Option<ParentLevelCtx>) -> (Self, LevelConnectionReport) {

        let mut schedule = TurnSchedule::new();

        let TerrainMetadata { width, height, start_coord, connection_report } =
            terrain.generate(ids, rng, &mut schedule, action, parent);

        let mut sh = SpatialHashTable::new(width, height);
        let mut ecs = EcsCtx::new();

        action.insert_position(entity_id, start_coord);

        let turn_offset = action.turn_offset(entity_id).expect("Missing component turn_offset");
        let pc_ticket = schedule.insert(entity_id, turn_offset);
        action.insert_schedule_ticket(entity_id, pc_ticket);

        sh.update(&ecs, &action, action_id);

        ecs.commit(action);

        (Level {
            ecs: ecs,
            spatial_hash: sh,
            turn_schedule: schedule,
        }, connection_report)
    }

    pub fn commit(&mut self, action: &mut EcsAction, action_id: u64) {
        self.spatial_hash.update(&self.ecs, action, action_id);
        self.ecs.commit(action);
    }

    pub fn commit_into(&mut self, from: &mut EcsAction, to: &mut EcsAction, action_id: u64) {
        self.spatial_hash.update(&self.ecs, from, action_id);
        self.ecs.commit_into(from, to);
    }
}

pub struct LevelConnectionReport {
    connections: Vec<LevelConnection>,
}

// Connection between a level switching entity (e.g. stairs) in an existing level and a new level
#[derive(Clone, Copy)]
pub struct LevelConnection {
    pub original: EntityId,
    pub new: EntityId,
}

impl LevelConnectionReport {
    pub fn new() -> Self {
        LevelConnectionReport {
            connections: Vec::new(),
        }
    }

    pub fn iter(&self) -> LevelConnectionReportIter {
        LevelConnectionReportIter(self.connections.iter())
    }

    pub fn connect(&mut self, original: EntityId, new: EntityId) {
        self.connections.push(LevelConnection {
            original: original,
            new: new,
        });
    }
}

pub struct LevelConnectionReportIter<'a>(slice::Iter<'a, LevelConnection>);

impl<'a> Iterator for LevelConnectionReportIter<'a> {
    type Item = LevelConnection;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|r| *r)
    }
}
