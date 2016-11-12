use ecs::{ComponentTypeSet, component_type, EcsCtx};

use geometry::Vector2;

#[test]
fn component_set() {
    let mut set = ComponentTypeSet::new();

    set.insert_position();
    set.insert_solid();

    assert!(set.contains_solid());
    assert!(set.contains_position());
    assert!(!set.contains_opacity());
}

#[test]
fn component_set_iter() {
    let mut set = ComponentTypeSet::new();

    set.insert_position();
    set.insert_solid();

    let mut iter = set.iter();

    // components are iterated through in alphabetical order
    assert_eq!(iter.next(), Some(component_type::POSITION));
    assert_eq!(iter.next(), Some(component_type::SOLID));
    assert!(iter.next().is_none());
}

#[test]
fn entity_create_destroy() {
    let mut ctx = EcsCtx::new();

    let id = {
        let mut entity = ctx.alloc_entity_mut();
        entity.insert_solid();
        entity.insert_position(Vector2::new(0, 0));

        assert!(entity.contains_solid());
        assert!(entity.contains_position());
        assert!(!entity.contains_opacity());

        assert_eq!(*entity.position().unwrap(), Vector2::new(0, 0));
        assert!(!entity.is_empty());

        entity.id()
    };

    ctx.remove_entity(id);

    let entity = ctx.entity(id).unwrap();

    assert!(!entity.contains_solid());
    assert!(!entity.contains_position());
    assert!(entity.position().is_none());
    assert!(entity.is_empty());
}
