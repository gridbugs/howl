use game::{Component, ComponentType, Speed, StatusCounter};
use game::components::{DoorState, Form, PathTraverse};
use game::knowledge::{DrawableKnowledge, SimpleNpcKnowledge};

use geometry::{Vector2, Direction};
use tile::ComplexTile;
use terminal;
use behaviour;

use std::cell::{Ref, RefMut};
use std::collections::HashSet;

// Convenience wrappers around entities

pub trait EntityWrapper<'a>: Sized {
    fn get_component(self, component_type: ComponentType) -> Option<&'a Component>;

    fn has_component(self, component_type: ComponentType) -> bool {
        self.get_component(component_type).is_some()
    }

    fn position(self) -> Option<Vector2<isize>> {
        if let Some(&Component::Position(ref vec)) = self.get_component(ComponentType::Position) {
            Some(*vec)
        } else {
            None
        }
    }

    fn is_pc(self) -> bool {
        if let Some(&Component::PlayerCharacter) =
               self.get_component(ComponentType::PlayerCharacter) {
            true
        } else {
            false
        }
    }

    fn door_state(self) -> Option<DoorState> {
        if let Some(&Component::Door(state)) = self.get_component(ComponentType::Door) {
            Some(state)
        } else {
            None
        }
    }

    fn opacity(self) -> f64 {
        if let Some(&Component::Opacity(o)) = self.get_component(ComponentType::Opacity) {
            o
        } else {
            0.0
        }
    }

    fn vision_distance(self) -> Option<usize> {
        if let Some(&Component::VisionDistance(distance)) =
               self.get_component(ComponentType::VisionDistance) {
            Some(distance)
        } else {
            None
        }
    }

    fn drawable_knowledge(self) -> Option<Ref<'a, DrawableKnowledge>> {
        if let Some(&Component::DrawableKnowledge(ref knowledge)) =
               self.get_component(ComponentType::DrawableKnowledge) {
            Some(knowledge.borrow())
        } else {
            None
        }
    }

    fn drawable_knowledge_mut(self) -> Option<RefMut<'a, DrawableKnowledge>> {
        if let Some(&Component::DrawableKnowledge(ref knowledge)) =
               self.get_component(ComponentType::DrawableKnowledge) {
            Some(knowledge.borrow_mut())
        } else {
            None
        }
    }

    fn simple_npc_knowledge(self) -> Option<Ref<'a, SimpleNpcKnowledge>> {
        if let Some(&Component::SimpleNpcKnowledge(ref knowledge)) =
               self.get_component(ComponentType::SimpleNpcKnowledge) {
            Some(knowledge.borrow())
        } else {
            None
        }
    }

    fn simple_npc_knowledge_mut(self) -> Option<RefMut<'a, SimpleNpcKnowledge>> {
        if let Some(&Component::SimpleNpcKnowledge(ref knowledge)) =
               self.get_component(ComponentType::SimpleNpcKnowledge) {
            Some(knowledge.borrow_mut())
        } else {
            None
        }
    }

    fn tile_depth(self) -> Option<isize> {
        if let Some(&Component::TileDepth(depth)) = self.get_component(ComponentType::TileDepth) {
            Some(depth)
        } else {
            None
        }
    }

    fn tile(self) -> Option<ComplexTile> {
        if let Some(&Component::Tile(tile)) = self.get_component(ComponentType::Tile) {
            Some(tile)
        } else {
            None
        }
    }

    fn axis_velocity(self) -> Option<(Direction, Speed)> {
        if let Some(&Component::AxisVelocity { direction, speed }) =
               self.get_component(ComponentType::AxisVelocity) {
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
        if let Some(&Component::FormSlot(form)) = self.get_component(ComponentType::FormSlot) {
            Some(form)
        } else {
            None
        }
    }

    fn beast_transform(self) -> Option<StatusCounter> {
        if let Some(&Component::BeastTransform(counter)) =
               self.get_component(ComponentType::BeastTransform) {
            Some(counter)
        } else {
            None
        }
    }

    fn human_transform(self) -> Option<StatusCounter> {
        if let Some(&Component::HumanTransform(counter)) =
               self.get_component(ComponentType::HumanTransform) {
            Some(counter)
        } else {
            None
        }
    }

    fn walk_speed(self) -> u64 {
        if let Some(&Component::WalkSpeed(speed)) = self.get_component(ComponentType::WalkSpeed) {
            speed
        } else {
            0
        }
    }

    fn input_source(self) -> Option<terminal::InputSource> {
        if let Some(&Component::InputSource(is)) = self.get_component(ComponentType::InputSource) {
            Some(is)
        } else {
            None
        }
    }

    fn behaviour_state_mut(self) -> Option<RefMut<'a, behaviour::State>> {
        if let Some(&Component::BehaviourState(ref state)) =
               self.get_component(ComponentType::BehaviourState) {
            Some(state.borrow_mut())
        } else {
            None
        }
    }

    fn has_behaviour(self) -> bool {
        self.has_component(ComponentType::Behaviour)
    }

    fn target_set(self) -> Option<Ref<'a, HashSet<Vector2<isize>>>> {
        if let Some(&Component::TargetSet(ref set)) = self.get_component(ComponentType::TargetSet) {
            Some(set.borrow())
        } else {
            None
        }
    }

    fn target_set_mut(self) -> Option<RefMut<'a, HashSet<Vector2<isize>>>> {
        if let Some(&Component::TargetSet(ref set)) = self.get_component(ComponentType::TargetSet) {
            Some(set.borrow_mut())
        } else {
            None
        }
    }

    fn path_traverse(self) -> Option<Ref<'a, PathTraverse>> {
        if let Some(&Component::PathTraverse(ref traverse)) =
               self.get_component(ComponentType::PathTraverse) {
            Some(traverse.borrow())
        } else {
            None
        }
    }

    fn path_traverse_mut(self) -> Option<RefMut<'a, PathTraverse>> {
        if let Some(&Component::PathTraverse(ref traverse)) =
               self.get_component(ComponentType::PathTraverse) {
            Some(traverse.borrow_mut())
        } else {
            None
        }
    }
}
