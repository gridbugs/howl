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
};

use game::Component::*;
use game::ComponentType as CType;

use grid::{
    StaticGrid,
    DefaultGrid,
};

use perlin::{
    Perlin3Grid,
    PerlinWrapType,
};

use geometry::{
    Vector2,
    Vector3,
};

use table::{
    TableTable,
    EntryAccessor,
};

use std::cell::RefCell;
use std::collections::HashSet;

pub type LevelSpatialHashMap =
    SpatialHashMap<StaticGrid<SpatialHashCell>>;

pub type LevelId = usize;

#[derive(Debug, Clone)]
pub struct Level {
    id: LevelId,
    pub width: usize,
    pub height: usize,
    pub schedule: RefCell<TurnSchedule>,
    pub spatial_hash: LevelSpatialHashMap,
    entities: LevelEntityTable,
    entity_ids: HashSet<EntityId>,
    perlin: Perlin3Grid,
    perlin_zoom: f64,
    perlin_min: f64,
    perlin_max: f64,
    perlin_change: Vector3<f64>,
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
            schedule: RefCell::new(TurnSchedule::new()),
            spatial_hash: SpatialHashMap::new(
                    StaticGrid::new_default(width, height)),
            entity_ids: HashSet::new(),
            entities: LevelEntityTable::new(),
            perlin: Perlin3Grid::new(width, height, PerlinWrapType::Regenerate).unwrap(),
            perlin_zoom: 0.05,
            perlin_min: -0.1,
            perlin_max: 0.1,
            perlin_change: Vector3::new(0.05, 0.02, 0.01),
        }
    }

    pub fn id(&self) -> LevelId {
        self.id
    }

    pub fn add(&mut self, id: EntityId, entity: Entity) -> Option<Entity> {
        self.entity_ids.insert(id);
        self.entities.add(id, entity)
    }

    pub fn remove(&mut self, id: EntityId) -> Option<Entity> {
        self.entity_ids.remove(&id);
        self.entities.remove(id)
    }

    pub fn get_mut(&mut self, id: EntityId) -> Option<LevelEntityRefMut> {
        self.entities.get_mut(id)
    }

    // Makes the bookkeeping info reflect the contents of entities
    pub fn finalise(&mut self, turn_count: u64)
    {
        for entity_id in self.entity_ids.clone() {
            let entity = self.entities.get(entity_id).unwrap();
            self.spatial_hash.add_entity(entity_id, entity, turn_count);
        }
    }

    pub fn apply_perlin_change(&mut self) {
        self.perlin.scroll(self.perlin_change.x, self.perlin_change.y);
        self.perlin.mutate(self.perlin_change.z);
    }

    fn noise(&self, x: isize, y: isize) -> Option<f64> {
        self.perlin.noise((x as f64) * self.perlin_zoom, (y as f64) * self.perlin_zoom)
    }

    pub fn moonlight(&self, x: isize, y: isize) -> bool {
        if let Some(noise) = self.noise(x, y) {
            if noise > self.perlin_min && noise < self.perlin_max {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn perlin_update(&self) -> UpdateSummary {
        let mut update = UpdateSummary::new();

        let outside = self.entities.accessor(CType::Outside);
        let position = self.entities.accessor(CType::Position);
        let moonlight = self.entities.accessor(CType::Moon);

        for id in outside.ids() {
            if let Some(Vector2 {x, y}) = position.access(*id).position() {
                let new = self.moonlight(x, y);
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

    pub fn update_spatial_hash(&mut self, update: &UpdateSummary, turn_count: u64) {
        self.spatial_hash.update(update, &self.entities, turn_count);
    }
}
