use game::{
    EntityWrapper,
    Component,
    ComponentType,
};

use table::{
    TableId,
    Table,
    TableTable,
    InvertedTableRef,
    InvertedTableRefMut,
    InvertedTableTable,
    TableRef,
    IterTableRef,
    TableRefMut,
    IdTableRef,
};

pub type InvertedEntityRef<'a> = InvertedTableRef<'a, ComponentType, Component>;
pub type InvertedEntityRefMut<'a> = InvertedTableRefMut<'a, ComponentType, Component>;
pub type InvertedEntityTable = InvertedTableTable<ComponentType, Component>;

pub type EntityId = TableId;
pub type Entity = Table<ComponentType, Component>;

pub trait EntityTable<'a>: TableTable<'a, ComponentType, Component> {}

impl<'a> EntityTable<'a> for InvertedEntityTable {}

pub trait EntityRef<'a>: TableRef<'a, ComponentType, Component> {}
pub trait IterEntityRef<'a>: IterTableRef<'a, ComponentType, Component> + EntityRef<'a> {}
pub trait EntityRefMut<'a>: TableRefMut<'a, ComponentType, Component> {}
pub trait IdEntityRef<'a>: IdTableRef<'a, ComponentType, Component> + IterEntityRef<'a> {}

impl<'a> EntityRef<'a> for &'a Entity {}
impl<'a> IterEntityRef<'a> for &'a Entity {}
impl<'a> EntityRefMut<'a> for Entity {}
impl<'a> EntityRefMut<'a> for &'a mut Entity {}

impl<'a> EntityRef<'a> for InvertedEntityRef<'a> {}
impl<'a> IterEntityRef<'a> for InvertedEntityRef<'a> {}
impl<'a> EntityRefMut<'a> for InvertedEntityRefMut<'a> {}
impl<'a> IdEntityRef<'a> for InvertedEntityRef<'a> {}

impl<'a, E> EntityWrapper<'a> for E
where E: EntityRef<'a>
{
    fn get_component(self, component_type: ComponentType) -> Option<&'a Component> {
        self.get(component_type)
    }


    fn has_component(self, component_type: ComponentType) -> bool {
        self.has(component_type)
    }
}

macro_rules! entity {
    () => { game::entity::Entity::new() };
    ( $( $x:expr ),* , ) => { entity!( $( $x ),* ) };
    ( $( $x:expr ),* ) => {{
        let mut entity = game::entity::Entity::new();
        $(entity.add($x);)*
        entity
    }};
}
