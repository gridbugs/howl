use game::{
    Component,
    ComponentType,
    Speed,
    StatusCounter,
    ActorType,
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
use tile::ComplexTile;

use std::cell::{
    Ref,
    RefMut,
};

// Convenience wrappers around entities

pub trait EntityWrapper<'a> : Sized {

    fn get_component(self, component_type: ComponentType) -> Option<&'a Component>;
    fn has_component(self, component_Type: ComponentType) -> bool;

    fn position(self) -> Option<Vector2<isize>> {
        if let Some(&Component::Position(ref vec)) =
            self.get_component(ComponentType::Position)
        {
            Some(*vec)
        } else {
            None
        }
    }

    fn actor_type(self) -> Option<ActorType> {
         if let Some(&Component::Actor(actor)) =
            self.get_component(ComponentType::Actor)
        {
            Some(actor)
        } else {
            None
        }
    }

    fn is_pc(self) -> bool {
        if let Some(ActorType::Player) = self.actor_type() {
            true
        } else {
            false
        }
    }

    fn door_state(self) -> Option<DoorState> {
        if let Some(&Component::Door(state)) =
            self.get_component(ComponentType::Door)
        {
            Some(state)
        } else {
            None
        }
    }

    fn opacity(self) -> f64 {
        if let Some(&Component::Opacity(o)) =
            self.get_component(ComponentType::Opacity)
        {
            o
        } else {
            0.0
        }
    }

    fn vision_distance(self) -> Option<usize> {
        if let Some(&Component::VisionDistance(distance)) =
            self.get_component(ComponentType::VisionDistance)
        {
            Some(distance)
        } else {
            None
        }
    }

    fn drawable_knowledge(self) -> Option<Ref<'a, DrawableKnowledge>> {
        if let Some(&Component::DrawableKnowledge(ref knowledge)) =
            self.get_component(ComponentType::DrawableKnowledge)
        {
            Some(knowledge.borrow())
        } else {
            None
        }
    }

    fn drawable_knowledge_mut(self) -> Option<RefMut<'a, DrawableKnowledge>> {
        if let Some(&Component::DrawableKnowledge(ref knowledge)) =
            self.get_component(ComponentType::DrawableKnowledge)
        {
            Some(knowledge.borrow_mut())
        } else {
            None
        }
    }

    fn tile_depth(self) -> Option<isize> {
         if let Some(&Component::TileDepth(depth)) =
            self.get_component(ComponentType::TileDepth)
        {
            Some(depth)
        } else {
            None
        }
    }

    fn tile(self) -> Option<ComplexTile> {
         if let Some(&Component::Tile(tile)) =
            self.get_component(ComponentType::Tile)
        {
            Some(tile)
        } else {
            None
        }
    }

    fn axis_velocity(self) -> Option<(Direction, Speed)> {
        if let Some(&Component::AxisVelocity {direction, speed}) =
            self.get_component(ComponentType::AxisVelocity)
        {
            Some((direction, speed))
        } else {
            None
        }
    }

    fn is_door_opener(self) -> bool {
        self.has_component(ComponentType::DoorOpener)
    }

    fn is_collider(self) -> bool {
        self.has_component(ComponentType::Collider)
    }

    fn is_destroy_on_collision(self) -> bool {
        self.has_component(ComponentType::DestroyOnCollision)
    }

    fn is_bullet(self) -> bool {
        self.has_component(ComponentType::Bullet)
    }

    fn has_moon(self) -> bool {
        self.has_component(ComponentType::Moon)
    }

    fn form(self) -> Option<Form> {
        if let Some(&Component::FormSlot(form)) =
            self.get_component(ComponentType::FormSlot)
        {
            Some(form)
        } else {
            None
        }
    }

    fn beast_transform(self) -> Option<StatusCounter> {
        if let Some(&Component::BeastTransform(counter)) =
            self.get_component(ComponentType::BeastTransform)
        {
            Some(counter)
        } else {
            None
        }
    }

    fn human_transform(self) -> Option<StatusCounter> {
        if let Some(&Component::HumanTransform(counter)) =
            self.get_component(ComponentType::HumanTransform)
        {
            Some(counter)
        } else {
            None
        }
    }
}
