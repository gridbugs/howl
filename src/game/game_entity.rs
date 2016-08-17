use game::{
    Entity,
    EntityId,
    Component,
    ComponentType,
    SpacialHashMap,
};
use game::components::{
    Level,
    DoorState,
};
use game::knowledge::DefaultKnowledge;

use geometry::Vector2;
use renderer::Tile;
use colour::ansi::AnsiColour;

use std::cell::{
    Ref,
    RefMut,
};

// Convenience wrappers around entities
impl Entity {

    pub fn id(&self) -> EntityId {
        self.id.unwrap()
    }

    pub fn position(&self) -> Option<Vector2<isize>> {
        if let Some(&Component::Position(ref vec)) =
            self.get(ComponentType::Position)
        {
            Some(*vec)
        } else {
            None
        }
    }

    pub fn on_level(&self) -> Option<EntityId> {
        if let Some(&Component::OnLevel(level_id)) =
            self.get(ComponentType::OnLevel)
        {
            Some(level_id)
        } else {
            None
        }
    }

    pub fn level_data(&self) -> Option<&Level> {
        if let Some(&Component::LevelData(ref level)) =
            self.get(ComponentType::LevelData)
        {
            Some(level)
        } else {
            None
        }
    }

    pub fn level_data_mut(&mut self) -> Option<&mut Level> {
        if let Some(&mut Component::LevelData(ref mut level)) =
            self.get_mut(ComponentType::LevelData)
        {
            Some(level)
        } else {
            None
        }
    }

    pub fn level_spacial_hash(&self) -> Option<Ref<SpacialHashMap>> {
        self.level_data().map(|level| {
            level.spacial_hash.borrow()
        })
    }

    pub fn is_pc(&self) -> bool {
        self.has(ComponentType::PlayerActor)
    }

    pub fn door_state(&self) -> Option<DoorState> {
        if let Some(&Component::Door(state)) =
            self.get(ComponentType::Door)
        {
            Some(state)
        } else {
            None
        }
    }

    pub fn opacity(&self) -> f64 {
        if let Some(&Component::Opacity(o)) =
            self.get(ComponentType::Opacity)
        {
            o
        } else {
            0.0
        }
    }

    pub fn vision_distance(&self) -> Option<usize> {
        if let Some(&Component::VisionDistance(distance)) =
            self.get(ComponentType::VisionDistance)
        {
            Some(distance)
        } else {
            None
        }
    }

    pub fn default_knowledge(&self) -> Option<RefMut<DefaultKnowledge>> {
        if let Some(&Component::DefaultKnowledge(ref knowledge)) =
            self.get(ComponentType::DefaultKnowledge)
        {
            Some(knowledge.borrow_mut())
        } else {
            None
        }
    }

    pub fn tile_depth(&self) -> Option<isize> {
         if let Some(&Component::TileDepth(depth)) =
            self.get(ComponentType::TileDepth)
        {
            Some(depth)
        } else {
            None
        }
    }

    pub fn tile(&self) -> Option<Tile> {
         if let Some(&Component::TransparentTile(tile)) =
            self.get(ComponentType::TransparentTile)
        {
            Some(tile)
        } else if let Some(&Component::SolidTile {tile, background: _}) =
            self.get(ComponentType::SolidTile)
        {
            Some(tile)
        } else {
            None
        }
    }

    pub fn background(&self) -> Option<AnsiColour> {
        if let Some(&Component::SolidTile {tile: _, background}) =
            self.get(ComponentType::SolidTile)
        {
            Some(background)
        } else {
            None
        }
    }
}
