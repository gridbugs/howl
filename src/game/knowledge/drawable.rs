use game::*;
use game::data::*;
use ecs::Entity;

use spatial_hash::*;
use grid::{Grid, StaticGrid, DefaultGrid};
use util::{BestMap, TwoDimensionalCons};
use coord::Coord;

pub type DrawableKnowledge = GameKnowledge<DrawableKnowledgeLevel>;

#[derive(Serialize, Deserialize)]
pub struct DrawableKnowledgeCell {
    last_updated: u64,
    foreground: BestMap<isize, TileType>,
    background: BestMap<isize, TileType>,
    name: BestMap<isize, NameMessageType>,
    description: BestMap<isize, DescriptionMessageType>,
    health_overlay: BestMap<isize, HitPoints>,
}

impl DrawableKnowledgeCell {
    fn new() -> Self {
        DrawableKnowledgeCell {
            last_updated: 0,
            foreground: BestMap::new(),
            background: BestMap::new(),
            name: BestMap::new(),
            description: BestMap::new(),
            health_overlay: BestMap::new(),
        }
    }

    pub fn foreground(&self) -> Option<TileType> {
        self.foreground.value()
    }

    pub fn background(&self) -> Option<TileType> {
        self.background.value()
    }

    pub fn name(&self) -> Option<NameMessageType> {
        self.name.value()
    }

    pub fn description(&self) -> Option<DescriptionMessageType> {
        self.description.value()
    }

    pub fn health_overlay(&self) -> Option<HitPoints> {
        self.health_overlay.value()
    }

    pub fn last_updated(&self) -> u64 {
        self.last_updated
    }

    pub fn update(&mut self, world_cell: &SpatialHashCell, _accuracy: f64, action_env: ActionEnv) -> bool {

        let mut changed = false;

        if self.last_updated <= world_cell.last_updated() {

            self.foreground.clear();
            self.background.clear();
            self.name.clear();
            self.description.clear();
            self.health_overlay.clear();

            for entity in action_env.ecs.entity_iter(world_cell.entity_id_iter()) {
                entity.tile_depth().map(|depth| {
                    entity.tile().map(|tile| {
                        self.foreground.insert(depth, tile);
                        if tile.opaque_bg() {
                            self.background.insert(depth, tile);
                        }
                    });
                    entity.name().map(|name| {
                        self.name.insert(depth, name);
                    });
                    entity.description().map(|description| {
                        self.description.insert(depth, description);
                    });
                    if entity.contains_health_bar() {
                        entity.hit_points().map(|hit_points| {
                            self.health_overlay.insert(depth, hit_points);
                        });
                    }
                });
            }

            changed = true;
        }

        self.last_updated = action_env.id;

        changed
    }
}

impl Default for DrawableKnowledgeCell {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Serialize, Deserialize)]
pub struct DrawableKnowledgeLevel {
    grid: StaticGrid<DrawableKnowledgeCell>,
    default: DrawableKnowledgeCell,
    targets: Vec<Coord>,
    last_action_id: u64,
}

impl DrawableKnowledgeLevel {
    pub fn get_with_default(&self, coord: Coord) -> &DrawableKnowledgeCell {
        self.grid.get(coord).unwrap_or_else(|| &self.default)
    }

    pub fn sort_targets(&mut self, position: Coord) -> &[Coord] {
        self.targets.sort_by(|a, b| a.squared_distance(position).cmp(&b.squared_distance(position)));
        self.targets.as_slice()
    }

    pub fn can_see(&self, coord: Coord, action_env: ActionEnv) -> bool {
        self.get_with_default(coord).last_updated == action_env.id
    }

    pub fn can_remember(&self, coord: Coord) -> bool {
        self.get_with_default(coord).last_updated == 0
    }

    pub fn width(&self) -> usize {
        self.grid.width()
    }

    pub fn height(&self) -> usize {
        self.grid.height()
    }
}

impl LevelKnowledge for DrawableKnowledgeLevel {
    fn update_cell(&mut self, coord: Coord, world_cell: &SpatialHashCell, accuracy: f64, action_env: ActionEnv) -> bool {

        if let Some(knowledge_cell) = self.grid.get_mut(coord) {

            if knowledge_cell.last_updated == action_env.id {
                // this cell has already been updated
                return false;
            }

            let change = knowledge_cell.update(world_cell, accuracy, action_env);

            if self.last_action_id != action_env.id {
                self.targets.clear();
                self.last_action_id = action_env.id;
            }

            if world_cell.enemy() {
                self.targets.push(coord);
            }

            change
        } else {
            false
        }
    }
}

impl TwoDimensionalCons for DrawableKnowledgeLevel {
    fn new(width: usize, height: usize) -> Self {
        DrawableKnowledgeLevel {
            grid: StaticGrid::new_default(width, height),
            default: DrawableKnowledgeCell::new(),
            targets: Vec::new(),
            last_action_id: 0,
        }
    }
}

pub struct CellDrawInfo {
    pub foreground: Option<TileType>,
    pub background: Option<TileType>,
    pub visible: bool,
    pub front: bool,
    pub health_overlay: Option<HitPoints>,
}

impl Default for CellDrawInfo {
    fn default() -> Self {
        CellDrawInfo {
            foreground: None,
            background: None,
            visible: false,
            front: false,
            health_overlay: None,
        }
    }
}
