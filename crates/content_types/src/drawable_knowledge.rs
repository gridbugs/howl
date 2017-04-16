use messages::*;
use grid::{Grid, StaticGrid, DefaultGrid};
use util::BestMap;
use math::Coord;
use tile::TileType;
use hit_points::HitPoints;
use knowledge::GameKnowledge;
use dimension_constructor::TwoDimensionalCons;

pub type DrawableKnowledge = GameKnowledge<DrawableKnowledgeLevel>;

#[derive(Serialize, Deserialize)]
pub struct DrawableKnowledgeCell {
    pub last_updated: u64,
    pub foreground: BestMap<isize, TileType>,
    pub background: BestMap<isize, TileType>,
    pub name: BestMap<isize, NameMessageType>,
    pub description: BestMap<isize, DescriptionMessageType>,
    pub health_overlay: BestMap<isize, HitPoints>,
}

impl DrawableKnowledgeCell {
    pub fn new() -> Self {
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
}

impl Default for DrawableKnowledgeCell {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Serialize, Deserialize)]
pub struct DrawableKnowledgeLevel {
    pub grid: StaticGrid<DrawableKnowledgeCell>,
    pub default: DrawableKnowledgeCell,
    pub targets: Vec<Coord>,
    pub last_action_id: u64,
}

impl DrawableKnowledgeLevel {
    pub fn get_with_default(&self, coord: Coord) -> &DrawableKnowledgeCell {
        self.grid.get(coord).unwrap_or_else(|| &self.default)
    }

    pub fn sort_targets(&mut self, position: Coord) -> &[Coord] {
        self.targets.sort_by(|a, b| a.squared_distance(position).cmp(&b.squared_distance(position)));
        self.targets.as_slice()
    }

    pub fn can_remember(&self, coord: Coord) -> bool {
        self.get_with_default(coord).last_updated == 0
    }

    pub fn can_see(&self, coord: Coord, current_time: u64) -> bool {
        self.get_with_default(coord).last_updated == current_time
    }

    pub fn width(&self) -> usize {
        self.grid.width()
    }

    pub fn height(&self) -> usize {
        self.grid.height()
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
