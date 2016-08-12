use game::entity::{
    Entity,
    EntityId,
    Component,
    ComponentType,
};
use game::components::level::Level;
use game::spacial_hash::SpacialHashMap;

use geometry::Vector2;

use std::cell::Ref;

pub trait GameComponent {
    fn position_vector(&self) -> Option<Vector2<isize>>;
}

impl GameComponent for Component {
    fn position_vector(&self) -> Option<Vector2<isize>> {
        if let &Component::Position(ref vec) = self {
            Some(*vec)
        } else {
            None
        }
    }
}

// Convenience wrappers around entities
pub trait GameEntity {
    fn position(&self) -> Option<Vector2<isize>>;
    fn on_level(&self) -> Option<EntityId>;
    fn level_data(&self) -> Option<&Level>;
    fn level_data_mut(&mut self) -> Option<&mut Level>;
    fn level_spacial_hash(&self) -> Option<Ref<SpacialHashMap>>;
    fn is_pc(&self) -> bool;
}

impl GameEntity for Entity {
    fn position(&self) -> Option<Vector2<isize>> {
        if let Some(&Component::Position(ref vec)) =
            self.get(ComponentType::Position)
        {
            Some(*vec)
        } else {
            None
        }
    }

    fn on_level(&self) -> Option<EntityId> {
        if let Some(&Component::OnLevel(level_id)) =
            self.get(ComponentType::OnLevel)
        {
            Some(level_id)
        } else {
            None
        }
    }

    fn level_data(&self) -> Option<&Level> {
        if let Some(&Component::LevelData(ref level)) =
            self.get(ComponentType::LevelData)
        {
            Some(level)
        } else {
            None
        }
    }

    fn level_data_mut(&mut self) -> Option<&mut Level> {
        if let Some(&mut Component::LevelData(ref mut level)) =
            self.get_mut(ComponentType::LevelData)
        {
            Some(level)
        } else {
            None
        }
    }

    fn level_spacial_hash(&self) -> Option<Ref<SpacialHashMap>> {
        self.level_data().map(|level| {
            level.spacial_hash.borrow()
        })
    }

    fn is_pc(&self) -> bool {
        self.has(ComponentType::PlayerActor)
    }
}
