use std::collections::HashSet;

use ecs::*;
use math::Coord;
use grid::{Grid, DynamicGrid};

pub struct SpatialHashCell {
    // sum of opacities of everything in this cell
    opacity: f64,

    // count of solid entities in this cell
    solid: usize,

    entities: EntitySet,
}

pub struct SpatialHashTable {
    grid: DynamicGrid<SpatialHashCell>,
}

impl SpatialHashCell {
    fn new() -> Self {
        SpatialHashCell {
            opacity: 0.0,
            solid: 0,
            entities: EntitySet::new(),
        }
    }

    pub fn opacity(&self) -> f64 {
        self.opacity
    }

    pub fn solid(&self) -> bool {
        self.solid != 0
    }
}

impl Default for SpatialHashCell {
    fn default() -> Self {
        Self::new()
    }
}

impl SpatialHashTable {
    pub fn new() -> Self {
        SpatialHashTable {
            grid: DynamicGrid::new(),
        }
    }

    pub fn get(&self, coord: Coord) -> Option<&SpatialHashCell> {
        self.grid.get(coord)
    }

    pub fn limits_min(&self) -> Coord {
        self.grid.limits_min()
    }

    pub fn limits_max(&self) -> Coord {
        self.grid.limits_max()
    }

    fn get_mut_with_default(&mut self, coord: Coord) -> &mut SpatialHashCell {
        self.grid.get_mut_with_default(coord)
    }

    fn update_insertions(&mut self,
                         ctx: &EcsCtx,
                         insertions: &ActionInsertionTable,
                         insertion_types: &ComponentTypeSet) {

    }

    fn update_removals(&mut self,
                       ctx: &EcsCtx,
                       removals: &ActionRemovalTable,
                       removal_types: &ComponentTypeSet) {

    }

    fn update_removed_entities(&mut self,
                               ctx: &EcsCtx,
                               entities: &HashSet<EntityId>) {

    }

    pub fn update(&mut self, ctx: &EcsCtx, action: &EcsAction) {
        self.update_insertions(ctx,
                               &action.insertions,
                               &action.insertion_types);
        self.update_removals(ctx,
                             &action.removals,
                             &action.removal_types);
        self.update_removed_entities(ctx, &action.removed_entities);
    }
}
