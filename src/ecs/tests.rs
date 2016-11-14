use ecs::{ComponentTypeSet, component_type};

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
