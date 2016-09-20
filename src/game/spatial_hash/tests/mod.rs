use game;
use game::Component::*;
use game::ComponentType as CType;
use game::{Component, ComponentType, SpatialHashMap, SpatialHashCell, Entity, EntityId, EntityRef,
           IterEntityRef, IdEntityRef, EntityRefMut, AddedComponents};

use geometry::Vector2;
use grid::{StaticGrid, DefaultGrid};

use table::{TableId, TableRefMut, TableRef, IdTableRef, IterTableRef};

use std::collections::{HashSet, hash_map};

const WIDTH: usize = 10;
const HEIGHT: usize = 10;

#[derive(Clone, Debug)]
struct TestRef {
    entity: Entity,
    id: EntityId,
}

impl<'a> TableRef<'a, ComponentType, Component> for &'a TestRef {
    fn has(self, entry_type: ComponentType) -> bool {
        self.entity.has(entry_type)
    }
    fn get(self, entry_type: ComponentType) -> Option<&'a Component> {
        self.entity.get(entry_type)
    }
}

impl<'a> IterTableRef<'a, ComponentType, Component> for &'a TestRef {
    type Iter = hash_map::Iter<'a, ComponentType, Component>;
    type TypeIter = hash_map::Keys<'a, ComponentType, Component>;
    type EntryIter = hash_map::Values<'a, ComponentType, Component>;

    fn slots(self) -> Self::Iter {
        self.entity.slots()
    }

    fn entries(self) -> Self::EntryIter {
        self.entity.entries()
    }

    fn types(self) -> Self::TypeIter {
        self.entity.types()
    }
}


impl<'a> IdTableRef<'a, ComponentType, Component> for &'a TestRef {
    fn id(self) -> TableId {
        self.id
    }
}

impl<'a> TableRefMut<'a, ComponentType, Component> for TestRef {
    fn add(&mut self, entry: Component) -> Option<Component> {
        self.entity.add(entry)
    }

    fn remove(&mut self, t: ComponentType) -> Option<Component> {
        self.entity.remove(t)
    }

    fn get_mut(&mut self, t: ComponentType) -> Option<&mut Component> {
        self.entity.get_mut(t)
    }
}

impl<'a> EntityRef<'a> for &'a TestRef {}
impl<'a> IterEntityRef<'a> for &'a TestRef {}
impl<'a> IdEntityRef<'a> for &'a TestRef {}
impl<'a> EntityRefMut<'a> for TestRef {}

// helper fns

fn make_spatial_hash() -> SpatialHashMap<StaticGrid<SpatialHashCell>> {
    SpatialHashMap::new(StaticGrid::new_default(WIDTH, HEIGHT))
}

fn make_entity(id: EntityId, x: isize, y: isize) -> TestRef {
    let e = entity![
        Position(Vector2::new(x, y)),
        Solid,
    ];

    TestRef {
        entity: e,
        id: id,
    }
}

fn set_from_vec(mut v: Vec<CType>) -> HashSet<CType> {
    let mut s = HashSet::new();
    for c in v.drain(..) {
        s.insert(c);
    }
    s
}

/// Create a new spatial hash.
#[test]
fn creation() {
    make_spatial_hash();
}

/// Add an entity to a spatial hash, ensuring the component counts are updated.
#[test]
fn add_entity() {
    let mut s = make_spatial_hash();
    let e = make_entity(0, 0, 0);

    s.add_entity(e.id, &e, 0);

    let cell = s.get((0, 0)).unwrap();

    assert_eq!(cell.entities.len(), 1);
    assert!(cell.entities.contains(&e.id));
    assert!(cell.has(CType::Solid));
    assert!(cell.has(CType::Position));
}

/// Add then immediatly remove an entity, ensuring consistent component counts.
#[test]
fn remove_entity() {
    let mut s = make_spatial_hash();
    let e = make_entity(0, 0, 0);

    s.add_entity(e.id, &e, 0);
    s.remove_entity(&e, 0);

    let cell = s.get((0, 0)).unwrap();

    assert_eq!(cell.entities.len(), 0);
    assert!(!cell.entities.contains(&e.id));
    assert!(!cell.has(CType::Solid));
    assert!(!cell.has(CType::Position));
}

/// Add several entities, then remove them all, ensuring consistent component counts.
#[test]
fn add_remove_many_entities() {
    const NUM_ENTITIES: EntityId = 100;
    let mut s = make_spatial_hash();
    let mut entities = Vec::with_capacity(NUM_ENTITIES as usize);

    // create entities
    for i in 0..NUM_ENTITIES {
        let e = make_entity(i, 0, 0);
        entities.push(e);
    }

    // add entities to spatial hash
    for e in entities.iter() {
        s.add_entity(e.id, e, 0);
    }

    // assert that entities are in spatial hash
    {
        let cell = s.get((0, 0)).unwrap();
        assert_eq!(cell.entities.len(), NUM_ENTITIES as usize);
        for e in &entities {
            assert!(cell.entities.contains(&e.id));
        }
        assert!(cell.has(CType::Solid));
        assert!(cell.has(CType::Position));
    }

    // remove entities from spatial hash
    for e in &entities {
        s.remove_entity(e, 0);
    }

    // assert that the entities are gone from spatial hash
    {
        let cell = s.get((0, 0)).unwrap();
        assert_eq!(cell.entities.len(), 0);
        for e in &entities {
            assert!(!cell.entities.contains(&e.id));
        }
        assert!(!cell.has(CType::Solid));
        assert!(!cell.has(CType::Position));
    }
}

/// Add an entity, then change its components, ensuring consistent component counts.
#[test]
fn change_entity() {
    let mut s = make_spatial_hash();
    let e = make_entity(0, 0, 0);

    s.add_entity(e.id, &e, 0);

    s.add_components(&e,
                     &AddedComponents::from_entity(entity![
        Collider,
    ]),
                     0);

    s.remove_components(&e, &set_from_vec(vec![CType::Solid]), 0);

    let cell = s.get((0, 0)).unwrap();
    assert!(cell.has(CType::Collider));
    assert!(!cell.has(CType::Solid));
}

/// Add an entity, then add an existing component, then remove the component, ensuring consistent
/// component counts.
#[test]
fn add_remove_components() {
    let mut s = make_spatial_hash();
    let mut e = make_entity(0, 0, 0);

    s.add_components(&e,
                     &AddedComponents::from_entity(entity![
        Collider,
    ]),
                     0);
    e.add(Collider);
    assert!(s.get((0, 0)).unwrap().has(CType::Collider));

    // add the same component again
    s.add_components(&e,
                     &AddedComponents::from_entity(entity![
        Collider,
    ]),
                     0);
    e.add(Collider);
    assert!(s.get((0, 0)).unwrap().has(CType::Collider));

    s.remove_components(&e, &set_from_vec(vec![CType::Collider]), 0);
    e.remove(CType::Collider);
    assert!(!s.get((0, 0)).unwrap().has(CType::Collider));
}

/// Add an entity, then change its position, ensuring consistent component counts.
#[test]
fn move_entity() {
    let mut s = make_spatial_hash();
    let e = make_entity(0, 0, 0);

    s.add_entity(e.id, &e, 0);

    s.add_components(&e,
                     &AddedComponents::from_entity(entity![
        Position(Vector2::new(1, 1)),
    ]),
                     0);

    // assert that the starting cell is empty
    let cell = s.get((0, 0)).unwrap();
    assert_eq!(cell.entities.len(), 0);
    assert!(!cell.entities.contains(&e.id));
    assert!(!cell.has(CType::Solid));
    assert!(!cell.has(CType::Position));

    let cell = s.get((1, 1)).unwrap();
    assert_eq!(cell.entities.len(), 1);
    assert!(cell.entities.contains(&e.id));
    assert!(cell.has(CType::Solid));
    assert!(cell.has(CType::Position));
}

/// Add an entity, then change its position and another component, ensuring
/// consistent component counts.
#[test]
fn move_entity_adding_component() {
    let mut s = make_spatial_hash();
    let e = make_entity(0, 0, 0);

    s.add_entity(e.id, &e, 0);

    s.add_components(&e,
                     &AddedComponents::from_entity(entity![
        Position(Vector2::new(1, 1)),
        Collider,
    ]),
                     0);

    let cell = s.get((0, 0)).unwrap();
    assert_eq!(cell.entities.len(), 0);
    assert!(!cell.entities.contains(&e.id));
    assert!(!cell.has(CType::Solid));
    assert!(!cell.has(CType::Collider));
    assert!(!cell.has(CType::Position));

    let cell = s.get((1, 1)).unwrap();
    assert_eq!(cell.entities.len(), 1);
    assert!(cell.entities.contains(&e.id));
    assert!(cell.has(CType::Solid));
    assert!(cell.has(CType::Position));
    assert!(cell.has(CType::Position));
}

/// Add 2 identical entities to the same cell, then strip the components of one (besides position),
/// ensuring consistent component counts.
#[test]
fn add_entities_remove_components() {
    let mut s = make_spatial_hash();
    let e0 = make_entity(0, 0, 0);
    let mut e1 = make_entity(1, 0, 0);

    s.add_entity(e0.id, &e0, 0);
    s.add_entity(e1.id, &e1, 0);
    assert!(s.get((0, 0)).unwrap().has(CType::Solid));

    s.remove_components(&e1, &set_from_vec(vec![CType::Solid]), 0);
    e1.remove(CType::Solid);
    assert!(s.get((0, 0)).unwrap().has(CType::Solid));

    // second remove shouldn't have any effect
    s.remove_components(&e1, &set_from_vec(vec![CType::Solid]), 0);
    e1.remove(CType::Solid);
    assert!(s.get((0, 0)).unwrap().has(CType::Solid));
}

/// Add entities to a cell, change one of their opacity, then remove one, ensuring the opacity
/// tracked by the the spatial hash cell remains consistent.
#[test]
fn opacity() {
    let mut s = make_spatial_hash();

    let mut e0 = make_entity(0, 0, 0);
    let mut e1 = make_entity(1, 0, 0);

    e0.add(Opacity(0.1));
    e1.add(Opacity(0.2));

    s.add_entity(e0.id, &e0, 0);
    s.add_entity(e1.id, &e1, 0);

    assert_eq!((s.get((0, 0)).unwrap().opacity * 10.0).round(), 0.3 * 10.0);

    s.add_components(&e0,
                     &AddedComponents::from_entity(entity![
        Opacity(0.4),
    ]),
                     0);
    e0.add(Opacity(0.4));
    assert_eq!((s.get((0, 0)).unwrap().opacity * 10.0).round(), 0.6 * 10.0);

    s.remove_components(&e1, &set_from_vec(vec![CType::Opacity]), 0);
    e1.remove(CType::Opacity);
    assert_eq!((s.get((0, 0)).unwrap().opacity * 10.0).round(), 0.4 * 10.0);

    s.add_components(&e1,
                     &AddedComponents::from_entity(entity![
        Opacity(0.5),
    ]),
                     0);
    e1.add(Opacity(0.5));
    assert_eq!((s.get((0, 0)).unwrap().opacity * 10.0).round(), 0.9 * 10.0);

    s.remove_entity(&e1, 0);
    assert_eq!((s.get((0, 0)).unwrap().opacity * 10.0).round(), 0.4 * 10.0);
}
