use game::{
    Component,
    ComponentType,
    Speed,
    LevelId,
    StatusCounter,
    EntityRef,
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

pub trait EntityWrapper<'a> {
    fn position(&self) -> Option<Vector2<isize>>;
    fn on_level(&self) -> Option<LevelId>;
    fn is_pc(&self) -> bool;
    fn door_state(&self) -> Option<DoorState>;
    fn opacity(&self) -> f64;
    fn vision_distance(&self) -> Option<usize>;
    fn drawable_knowledge(&'a self) -> Option<Ref<DrawableKnowledge>>;
    fn drawable_knowledge_mut(&'a self) -> Option<RefMut<DrawableKnowledge>>;
    fn tile_depth(&self) -> Option<isize>;
    fn tile(&self) -> Option<ComplexTile>;
    fn axis_velocity(&self) -> Option<(Direction, Speed)>;
    fn is_door_opener(&self) -> bool;
    fn is_collider(&self) -> bool;
    fn is_destroy_on_collision(&self) -> bool;
    fn is_bullet(&self) -> bool;
    fn has_moon(&self) -> bool;
    fn form(&self) -> Option<Form>;
    fn beast_transform(&self) -> Option<StatusCounter>;
    fn human_transform(&self) -> Option<StatusCounter>;
}

impl<'a, E> EntityWrapper<'a> for E
where E: EntityRef<'a>
{
    fn position(&self) -> Option<Vector2<isize>> {
        if let Some(&Component::Position(ref vec)) =
            self.get(ComponentType::Position)
        {
            Some(*vec)
        } else {
            None
        }
    }

    fn on_level(&self) -> Option<LevelId> {
        if let Some(&Component::OnLevel(level_id)) =
            self.get(ComponentType::OnLevel)
        {
            Some(level_id)
        } else {
            None
        }
    }

    fn is_pc(&self) -> bool {
        self.has(ComponentType::PlayerActor)
    }

    fn door_state(&self) -> Option<DoorState> {
        if let Some(&Component::Door(state)) =
            self.get(ComponentType::Door)
        {
            Some(state)
        } else {
            None
        }
    }

    fn opacity(&self) -> f64 {
        if let Some(&Component::Opacity(o)) =
            self.get(ComponentType::Opacity)
        {
            o
        } else {
            0.0
        }
    }

    fn vision_distance(&self) -> Option<usize> {
        if let Some(&Component::VisionDistance(distance)) =
            self.get(ComponentType::VisionDistance)
        {
            Some(distance)
        } else {
            None
        }
    }

    fn drawable_knowledge(&'a self) -> Option<Ref<DrawableKnowledge>> {
        if let Some(&Component::DrawableKnowledge(ref knowledge)) =
            self.get(ComponentType::DrawableKnowledge)
        {
            Some(knowledge.borrow())
        } else {
            None
        }
    }

    fn drawable_knowledge_mut(&'a self) -> Option<RefMut<DrawableKnowledge>> {
        if let Some(&Component::DrawableKnowledge(ref knowledge)) =
            self.get(ComponentType::DrawableKnowledge)
        {
            Some(knowledge.borrow_mut())
        } else {
            None
        }
    }

    fn tile_depth(&self) -> Option<isize> {
         if let Some(&Component::TileDepth(depth)) =
            self.get(ComponentType::TileDepth)
        {
            Some(depth)
        } else {
            None
        }
    }

    fn tile(&self) -> Option<ComplexTile> {
         if let Some(&Component::Tile(tile)) =
            self.get(ComponentType::Tile)
        {
            Some(tile)
        } else {
            None
        }
    }

    fn axis_velocity(&self) -> Option<(Direction, Speed)> {
        if let Some(&Component::AxisVelocity {direction, speed}) =
            self.get(ComponentType::AxisVelocity)
        {
            Some((direction, speed))
        } else {
            None
        }
    }

    fn is_door_opener(&self) -> bool {
        self.has(ComponentType::DoorOpener)
    }

    fn is_collider(&self) -> bool {
        self.has(ComponentType::Collider)
    }

    fn is_destroy_on_collision(&self) -> bool {
        self.has(ComponentType::DestroyOnCollision)
    }

    fn is_bullet(&self) -> bool {
        self.has(ComponentType::Bullet)
    }

    fn has_moon(&self) -> bool {
        self.has(ComponentType::Moon)
    }

    fn form(&self) -> Option<Form> {
        if let Some(&Component::FormSlot(form)) =
            self.get(ComponentType::FormSlot)
        {
            Some(form)
        } else {
            None
        }
    }

    fn beast_transform(&self) -> Option<StatusCounter> {
        if let Some(&Component::BeastTransform(counter)) =
            self.get(ComponentType::BeastTransform)
        {
            Some(counter)
        } else {
            None
        }
    }

    fn human_transform(&self) -> Option<StatusCounter> {
        if let Some(&Component::HumanTransform(counter)) =
            self.get(ComponentType::HumanTransform)
        {
            Some(counter)
        } else {
            None
        }
    }
}
