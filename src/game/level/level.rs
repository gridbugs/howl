use std::slice;

use game::*;
use game::data::*;
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
                       action_id: ActionId,
                       parent: Option<ParentLevelCtx>,
                       difficulty: usize) -> (Self, LevelConnectionReport) {

        let mut schedule = TurnSchedule::new();

        // generate the level's contents
        let TerrainMetadata { width, height, start_coord, connection_report } =
            terrain.generate(ids, rng, &mut schedule, action, parent, difficulty);

        // compose a level object
        let mut level = Level {
            ecs: EcsCtx::new(),
            spatial_hash: SpatialHashTable::new(width, height),
            turn_schedule: schedule,
        };

        // add the character's starting position to the action that will insert them
        action.insert_position(entity_id, start_coord);

        // insert the character to the schedule and level contents
        level.schedule_from_action_and_commit(action, entity_id, action_id);

        (level, connection_report)
    }

    pub fn schedule_from_action(&mut self, action: &mut EcsAction, entity_id: EntityId) {
        let turn_offset = action.get_copy_turn_offset(entity_id).expect("Missing component turn_offset");
        let ticket = self.turn_schedule.insert(entity_id, turn_offset);
        action.insert_schedule_ticket(entity_id, ticket);
    }

    pub fn schedule_from_action_and_commit(&mut self, action: &mut EcsAction, entity_id: EntityId, action_id: ActionId) {
        self.schedule_from_action(action, entity_id);
        self.commit(action, action_id);
    }

    pub fn insert_entity_at_exit_and_commit(&mut self, action: &mut EcsAction,
                                            entity_id: EntityId, exit_id: EntityId, action_id: ActionId) {

        // find the position of the exit
        let position = self.ecs.get_copy_position(exit_id).expect("Missing position component");

        // move the character to that position in the action that will insert them
        action.insert_position(entity_id, position);

        self.schedule_from_action_and_commit(action, entity_id, action_id);
    }

    pub fn commit(&mut self, action: &mut EcsAction, action_id: ActionId) {
        self.spatial_hash.update(&self.ecs, action, action_id);
        self.ecs.commit(action);
    }

    pub fn commit_into(&mut self, from: &mut EcsAction, to: &mut EcsAction, action_id: ActionId) {
        self.spatial_hash.update(&self.ecs, from, action_id);
        self.ecs.commit_into(from, to);
    }

    pub fn remove_entity(&mut self, entity_id: EntityId, action_id: ActionId) -> EcsAction {
        // remove the entity from the level's entity store, creating an action that
        // when applied, adds the entity to an entity store
        let mut entity_remove = EcsAction::new();
        let mut entity_insert = EcsAction::new();

        if let Some(weapon_slots) = self.ecs.borrow_weapon_slots(entity_id) {
            for (_, id) in weapon_slots.iter() {
                entity_remove.entity_delete_by_id(*id, &self.ecs);
            }
        }

        if let Some(inventory) = self.ecs.borrow_inventory(entity_id) {
            for id in inventory.iter() {
                entity_remove.entity_delete_by_id(id, &self.ecs);
            }
        }

        entity_remove.entity_delete_by_id(entity_id, &self.ecs);
        self.commit_into(&mut entity_remove, &mut entity_insert, action_id);

        entity_insert
    }

    pub fn connect(&mut self, level_id: LevelId, connections: &LevelConnectionReport) {
        for LevelConnection { original, new } in connections.iter() {
            let exit = LevelExit {
                level_id: level_id,
                exit_id: new,
            };

            self.ecs.insert_level_switch(original, LevelSwitch::ExistingLevel(exit));
        }
    }

    pub fn clear(&mut self) {
        self.ecs.clear();
        self.spatial_hash.clear();
        self.turn_schedule.reset();
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
