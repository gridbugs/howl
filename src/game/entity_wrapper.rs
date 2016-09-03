use game::{
    Entity,
    EntityId,
    Component,
    ComponentType,
    Speed,
    LevelId,
    StatusCounter,
};
use game::components::{
    DoorState,
    Form,
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

    pub fn on_level(&self) -> Option<LevelId> {
        if let Some(&Component::OnLevel(level_id)) =
            self.get(ComponentType::OnLevel)
        {
            Some(level_id)
        } else {
            None
        }
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

    pub fn has_moon(&self) -> bool {
        self.has(ComponentType::Moon)
    }

    pub fn form(&self) -> Option<Form> {
        if let Some(&Component::FormSlot(form)) =
            self.get(ComponentType::FormSlot)
        {
            Some(form)
        } else {
            None
        }
    }

    pub fn beast_transform(&self) -> Option<StatusCounter> {
        if let Some(&Component::BeastTransform(counter)) =
            self.get(ComponentType::BeastTransform)
        {
            Some(counter)
        } else {
            None
        }
    }

    pub fn human_transform(&self) -> Option<StatusCounter> {
        if let Some(&Component::HumanTransform(counter)) =
            self.get(ComponentType::HumanTransform)
        {
            Some(counter)
        } else {
            None
        }
    }
}
