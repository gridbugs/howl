use game::entity::{EntityId, EntityTable, Entity};
use game::entity::Component::*;
use game::entity::ComponentType as CType;
use game::update::UpdateProgram;
use game::update::UpdateStatement::*;
use game::update::UpdateSummary;
use game::update::UpdateSummary_;

use game::game_entity::GameEntity;

use game::components::door::DoorState;

use geometry::direction::Direction;
use renderer::tile::Tile;
use colour::ansi;

pub fn walk(entity_id: EntityId, direction: Direction, entities: &EntityTable)
    -> UpdateProgram
{
    let mut vec = entities.get(entity_id).position().unwrap();
    vec += direction.vector().convert::<isize>();

    UpdateProgram::new(vec![
        SetComponent(entity_id, Position(vec)),
    ])
}

pub fn walk_(entity: &Entity, direction: Direction) -> UpdateSummary {
    let mut summary = UpdateSummary::new();

    let vec = entity.position().unwrap() + direction.vector().convert::<isize>();
    summary.set_entity_component(entity.id.unwrap(), Position(vec));

    summary
}

pub fn open_door(door_id: EntityId)
    -> UpdateProgram
{
    UpdateProgram::new(vec![
        RemoveComponent(door_id, CType::Solid),
        RemoveComponent(door_id, CType::SolidTile),
        AddComponent(door_id, TransparentTile(Tile::new('-', ansi::WHITE))),
        SetComponent(door_id, Door(DoorState::Open)),
    ])
}

pub fn open_door_(door_id: EntityId) -> UpdateSummary_ {
    let mut summary = UpdateSummary_::new();

    summary.remove_component(door_id, CType::Solid);
    summary.remove_component(door_id, CType::SolidTile);
    summary.add_component(door_id, TransparentTile(Tile::new('-', ansi::WHITE)));
    summary.add_component(door_id, Door(DoorState::Open));

    summary
}

impl UpdateSummary_ {
    pub fn walk(&mut self, entity: &Entity, direction: Direction)  {
        let vec = entity.position().unwrap() + direction.vector().convert::<isize>();
        self.add_component(entity.id.unwrap(), Position(vec));
    }

    pub fn open_door(&mut self, door_id: EntityId) {
        self.remove_component(door_id, CType::Solid);
        self.remove_component(door_id, CType::SolidTile);
        self.add_component(door_id, TransparentTile(Tile::new('-', ansi::WHITE)));
        self.add_component(door_id, Door(DoorState::Open));
    }
}
