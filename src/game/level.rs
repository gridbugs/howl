use game::{
    EntityId,
    EntityContext,
    EntityTable,
    TurnSchedule,
    SpacialHashMap,
    SpacialHashCell,
    UpdateSummary,
    EntityWrapper,
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

use table::TableRef;

use std::cell::RefCell;
use std::collections::HashSet;

pub type LevelSpacialHashMap =
    SpacialHashMap<StaticGrid<SpacialHashCell>>;

pub type LevelId = u64;

#[derive(Debug, Clone)]
pub struct Level {
    pub id: Option<LevelId>,
    pub width: usize,
    pub height: usize,
    pub entities: HashSet<EntityId>,
    pub schedule: RefCell<TurnSchedule>,
    pub spacial_hash: LevelSpacialHashMap,
    perlin: Perlin3Grid,
    perlin_zoom: f64,
    perlin_min: f64,
    perlin_max: f64,
    perlin_change: Vector3<f64>,
}

impl Level {
    pub fn new(width: usize, height: usize) -> Level {
        Level {
            id: None,
            width: width,
            height: height,
            entities: HashSet::new(),
            schedule: RefCell::new(TurnSchedule::new()),
            spacial_hash: SpacialHashMap::new(
                    StaticGrid::new_default(width, height)),
            perlin: Perlin3Grid::new(width, height, PerlinWrapType::Regenerate).unwrap(),
            perlin_zoom: 0.05,
            perlin_min: -0.1,
            perlin_max: 0.1,
            perlin_change: Vector3::new(0.05, 0.02, 0.01),
        }
    }

    pub fn set_id(&mut self, id: EntityId) {
        self.id = Some(id);
        self.spacial_hash.set_id(id);
    }

    pub fn add(&mut self, id: EntityId) {
        self.entities.insert(id);
    }

    // Makes the bookkeeping info reflect the contents of entities
    pub fn finalise(&mut self, entities: &EntityTable, turn_count: u64) {
        for entity_id in self.entities.clone() {
            let entity = entities.get(entity_id).unwrap();
            self.spacial_hash.add_entity(entity, turn_count);
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

    pub fn component_entities(&self, component_type: CType) -> Option<&HashSet<EntityId>> {
        self.spacial_hash.component_entities.get(&component_type)
    }

    pub fn perlin_update(&self, entities: &EntityContext) -> UpdateSummary {
        let mut update = UpdateSummary::new();

        if let Some(entity_ids) = self.component_entities(CType::Outside) {

            for entity in entities.id_set_iter(entity_ids) {
                let entity = entity.unwrap();
                if let Some(Vector2 {x, y}) = entity.position() {
                    let new = self.moonlight(x, y);
                    let current = entity.has(CType::Moon);

                    if new == current {
                        continue;
                    }

                    if new {
                        update.add_component(entity.id().unwrap(), Moon);
                    } else {
                        update.remove_component(entity.id().unwrap(), CType::Moon);
                    }
                }
            }
        }

        update
    }
}
