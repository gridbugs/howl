use game::{
    EntityId,
    Component,
    ComponentType,
    EntityTable,
};
use game::update::summary::UpdateSummary;
use game::table::ToType;

use std::mem;

pub struct UpdateProgram(Vec<UpdateStatement>);

pub enum UpdateStatement {
    SetComponent(EntityId, Component),
    AddComponent(EntityId, Component),
    RemoveComponent(EntityId, ComponentType),
}

use self::UpdateStatement::*;

impl UpdateStatement {
    pub fn apply(self, summary: &mut UpdateSummary,
                       entities: &mut EntityTable)
    {
        match self {
            SetComponent(entity_id, component) => {
                let mut entity = entities.get_mut(entity_id);
                if let Some(current) =
                    entity.get_mut(component.to_type())
                {
                    let original = mem::replace(current, component);
                    summary.change_entity(entity_id, original);
                } else {
                    panic!("SetComponent called on non-existent component");
                }
            },
            AddComponent(entity_id, component) => {
                let mut entity = entities.get_mut(entity_id);
                if entity.has(component.to_type()) {
                    panic!("AddComponent called with component already present");
                } else {
                    summary.add_component(entity_id, component.to_type());
                    entity.add(component);
                }
            },
            RemoveComponent(entity_id, component_type) => {
                let mut entity = entities.get_mut(entity_id);
                if entity.has(component_type) {
                    let component = entity.remove(component_type).unwrap();
                    summary.remove_component(entity_id, component);
                } else {
                    panic!("RemoveComponent called with component not present");
                }
            },
        }
    }
}

impl UpdateProgram {
    pub fn new(statements: Vec<UpdateStatement>) -> Self {
        UpdateProgram(statements)
    }

    pub fn new_empty() -> Self {
        UpdateProgram(vec![])
    }

    pub fn append(&mut self, statement: UpdateStatement) {
        self.0.push(statement);
    }

    pub fn apply(mut self, entities: &mut EntityTable) -> UpdateSummary {
        let mut summary = UpdateSummary::new();
        for statement in self.0.drain(..) {
            statement.apply(&mut summary, entities);
        }
        summary.update_spacial_hashes(entities);
        summary
    }
}
