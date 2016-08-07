use game::{
    EntityId,
    Component,
    EntityTable,
};
use game::update::summary::UpdateSummary;
use game::table::ToType;

use std::mem;

pub struct UpdateProgram(Vec<UpdateStatement>);
pub type UpdateProgramFn = Box<Fn(&EntityTable) -> UpdateProgram>;

pub enum UpdateStatement {
    SetEntityComponent(EntityId, Component),
}

use self::UpdateStatement::*;

impl UpdateStatement {
    pub fn apply(self, summary: &mut UpdateSummary,
                       entities: &mut EntityTable)
    {
        match self {
            SetEntityComponent(entity_id, component) => {
                let mut entity = entities.get_mut(entity_id);
                if let Some(current) =
                    entity.get_mut(component.to_type())
                {
                    let original = mem::replace(current, component);
                    summary.change_entity(entity_id, original);
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
        summary
    }
}
