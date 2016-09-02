use game::{
    Entity,
    EntityId,
    Component,
    ComponentType,
    Speed,
    Level,
    LevelSpacialHashMap,
};
use game::components::{
    DoorState,
    Moonlight,
};
use game::knowledge::DrawableKnowledge;

use geometry::{
    Vector2,
    Direction,
};
use renderer::ComplexTile;

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

    pub fn level_spacial_hash(&self) -> Option<Ref<LevelSpacialHashMap>> {
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

    pub fn drawable_knowledge(&self) -> Option<Ref<DrawableKnowledge>> {
        if let Some(&Component::DrawableKnowledge(ref knowledge)) =
            self.get(ComponentType::DrawableKnowledge)
        {
            Some(knowledge.borrow())
        } else {
            None
        }
    }

    pub fn drawable_knowledge_mut(&self) -> Option<RefMut<DrawableKnowledge>> {
        if let Some(&Component::DrawableKnowledge(ref knowledge)) =
            self.get(ComponentType::DrawableKnowledge)
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

    pub fn tile(&self) -> Option<ComplexTile> {
         if let Some(&Component::Tile(tile)) =
            self.get(ComponentType::Tile)
        {
            Some(tile)
        } else {
            None
        }
    }

    pub fn axis_velocity(&self) -> Option<(Direction, Speed)> {
        if let Some(&Component::AxisVelocity {direction, speed}) =
            self.get(ComponentType::AxisVelocity)
        {
            Some((direction, speed))
        } else {
            None
        }
    }

    pub fn is_door_opener(&self) -> bool {
        self.has(ComponentType::DoorOpener)
    }

    pub fn is_collider(&self) -> bool {
        self.has(ComponentType::Collider)
    }

    pub fn is_destroy_on_collision(&self) -> bool {
        self.has(ComponentType::DestroyOnCollision)
    }

    pub fn is_bullet(&self) -> bool {
        self.has(ComponentType::Bullet)
    }

    pub fn moonlight(&self) -> Option<Moonlight> {
        if let Some(&Component::MoonlightSlot(moonlight)) =
            self.get(ComponentType::MoonlightSlot)
        {
            Some(moonlight)
        } else {
            None
        }
    }

    pub fn has_moonlight(&self) -> bool {
        self.moonlight().is_some()
    }

    pub fn is_moonlight_light(&self) -> bool {
        if let Some(Moonlight::Light) = self.moonlight() {
            true
        } else {
            false
        }
    }
}
