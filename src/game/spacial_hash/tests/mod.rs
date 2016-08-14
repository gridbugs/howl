use game;
use game::Component::*;
use game::ComponentType as CType;
use game::{
    SpacialHashMap,
    Entity,
};

use geometry::Vector2;

use std::collections::HashSet;

const WIDTH: usize = 10;
const HEIGHT: usize = 10;

// helper fns

fn make_spacial_hash() -> SpacialHashMap {
    SpacialHashMap::new(WIDTH, HEIGHT)
}

fn make_entity(x: isize, y: isize) -> Entity {
    let mut e = entity![
        Position(Vector2::new(x, y)),
        Solid,
    ];
    e.id = Some(0);

    e
}

fn set_from_vec(mut v: Vec<CType>) -> HashSet<CType> {
    let mut s = HashSet::new();
    for c in v.drain(..) {
        s.insert(c);
    }
    s
}

/// Create a new spacial hash.
#[test]
fn creation() {
    make_spacial_hash();
}

/// Add an entity to a spacial hash, ensuring the component counts are updated.
#[test]
fn add_entity() {
    let mut s = make_spacial_hash();
    let e = make_entity(0, 0);

    s.add_entity(&e);

    let cell = s.get((0, 0)).unwrap();

    assert_eq!(cell.entities.len(), 1);
    assert!(cell.entities.contains(&e.id.unwrap()));
    assert!(cell.has(CType::Solid));
    assert!(cell.has(CType::Position));
}

/// Add then immediatly remove an entity, ensuring consistent component counts.
#[test]
fn remove_entity() {
    let mut s = make_spacial_hash();
    let e = make_entity(0, 0);

    s.add_entity(&e);
    s.remove_entity(&e);

    let cell = s.get((0, 0)).unwrap();

    assert_eq!(cell.entities.len(), 0);
    assert!(!cell.entities.contains(&e.id.unwrap()));
    assert!(!cell.has(CType::Solid));
    assert!(!cell.has(CType::Position));
}

/// Add several entities, then remove them all, ensuring consistent component counts.
#[test]
fn add_remove_many_entities() {
    const NUM_ENTITIES: usize = 100;
    let mut s = make_spacial_hash();
    let mut entities = Vec::with_capacity(NUM_ENTITIES);

    // create entities
    for i in 0..NUM_ENTITIES {
        let mut e = make_entity(0, 0);
        e.id = Some(i as u64);
        entities.push(e);
    }

    // add entities to spacial hash
    for e in &entities {
        s.add_entity(&e);
    }

    // assert that entities are in spacial hash
    {
        let cell = s.get((0, 0)).unwrap();
        assert_eq!(cell.entities.len(), NUM_ENTITIES);
        for e in &entities {
            assert!(cell.entities.contains(&e.id.unwrap()));
        }
        assert!(cell.has(CType::Solid));
        assert!(cell.has(CType::Position));
    }

    // remove entities from spacial hash
    for e in &entities {
        s.remove_entity(&e);
    }

    // assert that the entities are gone from spacial hash
    {
        let cell = s.get((0, 0)).unwrap();
        assert_eq!(cell.entities.len(), 0);
        for e in &entities {
            assert!(!cell.entities.contains(&e.id.unwrap()));
        }
        assert!(!cell.has(CType::Solid));
        assert!(!cell.has(CType::Position));
    }
}

/// Add an entity, then change its components, ensuring consistent component counts.
#[test]
fn change_entity() {
    let mut s = make_spacial_hash();
    let e = make_entity(0, 0);

    s.add_entity(&e);

    s.add_components(&e, &entity![
        Collider,
    ]);

    s.remove_components(&e, &set_from_vec(vec![CType::Solid]));

    let cell = s.get((0, 0)).unwrap();
    assert!(cell.has(CType::Collider));
    assert!(!cell.has(CType::Solid));
}

/// Add an entity, then add an existing component, then remove the component, ensuring consistent
/// component counts.
#[test]
fn add_remove_components() {
    let mut s = make_spacial_hash();
    let mut e = make_entity(0, 0);

    s.add_components(&e, &entity![
        Collider,
    ]);
    e.add(Collider);
    assert!(s.get((0, 0)).unwrap().has(CType::Collider));

    // add the same component again
    s.add_components(&e, &entity![
        Collider,
    ]);
    e.add(Collider);
    assert!(s.get((0, 0)).unwrap().has(CType::Collider));

    s.remove_components(&e, &set_from_vec(vec![CType::Collider]));
    e.remove(CType::Collider);
    assert!(!s.get((0, 0)).unwrap().has(CType::Collider));
}

/// Add an entity, then change its position, ensuring consistent component counts.
#[test]
fn move_entity() {
    let mut s = make_spacial_hash();
    let e = make_entity(0, 0);

    s.add_entity(&e);

    s.add_components(&e, &entity![
        Position(Vector2::new(1, 1)),
    ]);

    // assert that the starting cell is empty
    let cell = s.get((0, 0)).unwrap();
    assert_eq!(cell.entities.len(), 0);
    assert!(!cell.entities.contains(&e.id.unwrap()));
    assert!(!cell.has(CType::Solid));
    assert!(!cell.has(CType::Position));

    let cell = s.get((1, 1)).unwrap();
    assert_eq!(cell.entities.len(), 1);
    assert!(cell.entities.contains(&e.id.unwrap()));
    assert!(cell.has(CType::Solid));
    assert!(cell.has(CType::Position));
}

/// Add an entity, then change its position and another component, ensuring
/// consistent component counts.
#[test]
fn move_entity_adding_component() {
    let mut s = make_spacial_hash();
    let e = make_entity(0, 0);

    s.add_entity(&e);

    s.add_components(&e, &entity![
        Position(Vector2::new(1, 1)),
        Collider,
    ]);

    let cell = s.get((0, 0)).unwrap();
    assert_eq!(cell.entities.len(), 0);
    assert!(!cell.entities.contains(&e.id.unwrap()));
    assert!(!cell.has(CType::Solid));
    assert!(!cell.has(CType::Collider));
    assert!(!cell.has(CType::Position));

    let cell = s.get((1, 1)).unwrap();
    assert_eq!(cell.entities.len(), 1);
    assert!(cell.entities.contains(&e.id.unwrap()));
    assert!(cell.has(CType::Solid));
    assert!(cell.has(CType::Position));
    assert!(cell.has(CType::Position));
}

/// Add 2 identical entities to the same cell, then strip the components of one (besides position),
/// ensuring consistent component counts.
#[test]
fn add_entities_remove_components() {
    let mut s = make_spacial_hash();
    let mut e0 = make_entity(0, 0);
    e0.id = Some(0);
    let mut e1 = make_entity(0, 0);
    e1.id = Some(1);

    s.add_entity(&e0);
    s.add_entity(&e1);
    assert!(s.get((0, 0)).unwrap().has(CType::Solid));

    s.remove_components(&e1, &set_from_vec(vec![CType::Solid]));
    e1.remove(CType::Solid);
    assert!(s.get((0, 0)).unwrap().has(CType::Solid));

    // second remove shouldn't have any effect
    s.remove_components(&e1, &set_from_vec(vec![CType::Solid]));
    e1.remove(CType::Solid);
    assert!(s.get((0, 0)).unwrap().has(CType::Solid));
}

/// Add entities to a cell, change one of their opacity, then remove one, ensuring the opacity
/// tracked by the the spacial hash cell remains consistent.
#[test]
fn opacity() {
    let mut s = make_spacial_hash();

    let mut e0 = make_entity(0, 0);
    e0.id = Some(0);
    let mut e1 = make_entity(0, 0);
    e1.id = Some(1);

    e0.add(Opacity(0.1));
    e1.add(Opacity(0.2));

    s.add_entity(&e0);
    s.add_entity(&e1);

    assert_eq!((s.get((0, 0)).unwrap().opacity*10.0).round(), 0.3*10.0);

    s.add_components(&e0, &entity![
        Opacity(0.4),
    ]);
    e0.add(Opacity(0.4));
    assert_eq!((s.get((0, 0)).unwrap().opacity*10.0).round(), 0.6*10.0);

    s.remove_components(&e1, &set_from_vec(vec![CType::Opacity]));
    e1.remove(CType::Opacity);
    assert_eq!((s.get((0, 0)).unwrap().opacity*10.0).round(), 0.4*10.0);

    s.add_components(&e1, &entity![
        Opacity(0.5),
    ]);
    e1.add(Opacity(0.5));
    assert_eq!((s.get((0, 0)).unwrap().opacity*10.0).round(), 0.9*10.0);

    s.remove_entity(&e1);
    assert_eq!((s.get((0, 0)).unwrap().opacity*10.0).round(), 0.4*10.0);
}
