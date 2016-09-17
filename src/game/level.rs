use game::{
    Entity,
    EntityId,
    TurnSchedule,
    SpatialHashMap,
    SpatialHashCell,
    UpdateSummary,
    ComponentWrapper,
    EntityStore,
    LevelEntityTable,
    LevelEntityRef,
    LevelEntityRefMut,
    Rule,
    ReserveEntityId,
    RuleContext,
    RuleResult,
};

use game::Component::*;
use game::ComponentType as CType;

use game::clouds::CloudContext;

use grid::{
    StaticGrid,
    DefaultGrid,
};

use geometry::Vector2;

use table::{
    TableTable,
    EntryAccessor,
    TableRefMut,
};

pub type LevelSpatialHashMap =
    SpatialHashMap<StaticGrid<SpatialHashCell>>;

pub type LevelId = usize;

#[derive(Debug, Clone)]
pub struct Level {
    id: LevelId,
    pub width: usize,
    pub height: usize,
    pub schedule: TurnSchedule,
    pub spatial_hash: LevelSpatialHashMap,
    entities: LevelEntityTable,
    clouds: CloudContext,
}

impl<'a> EntityStore<'a> for Level {
    type Ref = LevelEntityRef<'a>;
    fn get(&'a self, id: EntityId) -> Option<Self::Ref> {
        self.entities.get(id)
    }

    fn spatial_hash(&self) -> &LevelSpatialHashMap {
        &self.spatial_hash
    }
}

impl Level {
    pub fn new(width: usize, height: usize, id: LevelId) -> Level {
        Level {
            id: id,
            width: width,
            height: height,
            schedule: TurnSchedule::new(),
            spatial_hash: SpatialHashMap::new(
                    StaticGrid::new_default(width, height)),
            entities: LevelEntityTable::new(),
            clouds: CloudContext::new(width, height),
        }
    }

    pub fn id(&self) -> LevelId {
        self.id
    }

    pub fn add_external(&mut self, id: EntityId, entity: Entity, turn: u64) -> Option<Entity> {
        self.spatial_hash.add_entity(id, &entity, turn);
        self.add(id, entity)
    }

    pub fn add(&mut self, id: EntityId, entity: Entity) -> Option<Entity> {
        self.entities.add(id, entity)
    }

    pub fn remove(&mut self, id: EntityId) -> Option<Entity> {
        self.entities.remove(id)
    }

    pub fn get_mut(&mut self, id: EntityId) -> Option<LevelEntityRefMut> {
        self.entities.get_mut(id)
    }

    pub fn is_cloud(&self, x: isize, y: isize) -> bool {
        self.clouds.is_cloud(x, y)
    }

    pub fn update_clouds_action(&mut self) -> UpdateSummary {

        self.clouds.mutate();

        let mut update = UpdateSummary::new();

        let outside = self.entities.accessor(CType::Outside);
        let position = self.entities.accessor(CType::Position);
        let moonlight = self.entities.accessor(CType::Moon);

        for id in outside.ids() {
            if let Some(Vector2 {x, y}) = position.access(*id).position() {
                let new = !self.is_cloud(x, y);
                let current = moonlight.has(*id);

                if new == current {
                    continue;
                }

                if new {
                    update.add_component(*id, Moon);
                } else {
                    update.remove_component(*id, CType::Moon);
                }
            }
        }

        update
    }

    pub fn commit_update(&mut self, mut update: UpdateSummary, turn: u64) {
        self.spatial_hash.update(&update, &self.entities, turn);

        for (id, entity) in update.added_entities.drain() {
            self.add(id, entity);
        }

        for entity_id in update.removed_entities.iter() {
            self.remove(*entity_id).
                expect("Tried to remove non-existent entity.");
        }

        for (entity_id, mut components) in update.added_components.drain() {
            let mut entity = self.get_mut(entity_id).unwrap();
            for (_, component) in components.drain() {
                entity.add(component);
            }
        }

        for (entity_id, component_types) in update.removed_components.iter() {
            let mut entity = self.get_mut(*entity_id).unwrap();
            for component_type in component_types.iter() {
                entity.remove(*component_type);
            }
        }
    }

    pub fn check_rules<'a, I: IntoIterator<Item=&'a Box<Rule>>>(
        &self,
        update: &UpdateSummary,
        rules: I,
        ids: &ReserveEntityId) -> RuleResult
    {
        let mut reactions = Vec::new();
        let rule_context = RuleContext::new(&update, &self, ids);
        for rule in rules {
            let mut result = rule.check(rule_context);

            if result.is_accept() {
                for reaction in result.drain_reactions() {
                    reactions.push(reaction);
                }
            } else {
                return result;
            }
        }
        RuleResult::after_many(reactions)
    }

    fn update_spatial_hash(&mut self, update: &UpdateSummary, turn_count: u64) {
        self.spatial_hash.update(update, &self.entities, turn_count);
    }
}
