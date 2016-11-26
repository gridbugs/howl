use ecs::*;
use game::*;
use util::LeakyReserver;
use math::Coord;

struct Env {
    sh: SpatialHashTable,
    ctx: EcsCtx,
    ids: LeakyReserver<EntityId>,
}

impl Env {
    fn new() -> Self {
        Env {
            sh: SpatialHashTable::new(),
            ctx: EcsCtx::new(),
            ids: LeakyReserver::new(),
        }
    }
}

#[test]
fn insert_remove() {
    let mut env = Env::new();

    let mut action = EcsAction::new();
    let coord = Coord::new(1, 2);

    let id = {
        let mut entity = action.entity_mut(env.ids.reserve());
        entity.insert_position(coord);
        entity.insert_solid();

        entity.id()
    };

    assert!(!env.sh.get(coord).solid());
    env.sh.update(&env.ctx, &action, 0);
    env.ctx.commit(&mut action); // this resets the action so it can be reused
    assert!(env.sh.get(coord).solid());

    action.entity_mut(id).remove_solid();

    env.sh.update(&env.ctx, &action, 0);
    env.ctx.commit(&mut action);
    assert!(!env.sh.get(coord).solid());
}

#[test]
fn insert_move() {
    let mut env = Env::new();

    let mut action = EcsAction::new();
    let start_coord = Coord::new(1, 2);
    let end_coord = Coord::new(1, 3);

    let id = {
        let mut entity = action.entity_mut(env.ids.reserve());
        entity.insert_position(start_coord);
        entity.insert_solid();

        entity.id()
    };

    env.sh.update(&env.ctx, &action, 0);
    env.ctx.commit(&mut action);

    action.entity_mut(id).insert_position(end_coord);

    env.sh.update(&env.ctx, &action, 0);
    env.ctx.commit(&mut action);

    assert!(!env.sh.get(start_coord).solid());
    assert!(env.sh.get(end_coord).solid());
}

#[test]
fn remove_position() {
    let mut env = Env::new();

    let mut action = EcsAction::new();
    let start_coord = Coord::new(1, 2);

    let id = {
        let mut entity = action.entity_mut(env.ids.reserve());
        entity.insert_position(start_coord);
        entity.insert_solid();

        entity.id()
    };

    env.sh.update(&env.ctx, &action, 0);
    env.ctx.commit(&mut action);

    action.entity_mut(id).remove_position();

    env.sh.update(&env.ctx, &action, 0);
    env.ctx.commit(&mut action);

    assert!(!env.sh.get(start_coord).solid());
}

#[test]
fn insert_solid() {
    let mut env = Env::new();

    let mut action = EcsAction::new();
    let start_coord = Coord::new(1, 2);

    let id = {
        let mut entity = action.entity_mut(env.ids.reserve());
        entity.insert_position(start_coord);

        entity.id()
    };

    env.sh.update(&env.ctx, &action, 0);
    env.ctx.commit(&mut action);

    action.entity_mut(id).insert_solid();

    assert!(!env.sh.get(start_coord).solid());

    env.sh.update(&env.ctx, &action, 0);
    env.ctx.commit(&mut action);

    assert!(env.sh.get(start_coord).solid());
}

#[test]
fn track_opacity() {
    let mut env = Env::new();

    let mut action = EcsAction::new();
    let start_coord = Coord::new(1, 2);
    let end_coord = Coord::new(1, 3);

    // initialise with no opacity
    let id = {
        let mut entity = action.entity_mut(env.ids.reserve());
        entity.insert_position(start_coord);

        entity.id()
    };

    env.sh.update(&env.ctx, &action, 0);
    env.ctx.commit(&mut action);

    assert_eq!((env.sh.get(start_coord).opacity() * 10.0).round(), 0.0 * 10.0);

    // add an opacity of 0.5
    action.entity_mut(id).insert_opacity(0.5);

    env.sh.update(&env.ctx, &action, 0);
    env.ctx.commit(&mut action);

    assert_eq!((env.sh.get(start_coord).opacity() * 10.0).round(), 0.5 * 10.0);

    // decrease opacity to 0.2
    action.entity_mut(id).insert_opacity(0.2);

    env.sh.update(&env.ctx, &action, 0);
    env.ctx.commit(&mut action);

    assert_eq!((env.sh.get(start_coord).opacity() * 10.0).round(), 0.2 * 10.0);

    // move the entity
    action.entity_mut(id).insert_position(end_coord);

    env.sh.update(&env.ctx, &action, 0);
    env.ctx.commit(&mut action);
    assert_eq!((env.sh.get(start_coord).opacity() * 10.0).round(), 0.0 * 10.0);
    assert_eq!((env.sh.get(end_coord).opacity() * 10.0).round(), 0.2 * 10.0);
}

#[test]
fn insert_move_multiple() {
     let mut env = Env::new();

    let mut action = EcsAction::new();
    let start_coord = Coord::new(1, 2);
    let end_coord = Coord::new(1, 3);

    // add solid entity
    let id_a = {
        let mut entity = action.entity_mut(env.ids.reserve());
        entity.insert_position(start_coord);
        entity.insert_solid();

        entity.id()
    };

    env.sh.update(&env.ctx, &action, 0);
    env.ctx.commit(&mut action);

    assert!(env.sh.get(start_coord).solid());

    // add second solid entity in same cell
    let id_b = {
        let mut entity = action.entity_mut(env.ids.reserve());
        entity.insert_position(start_coord);
        entity.insert_solid();

        entity.id()
    };

    env.sh.update(&env.ctx, &action, 0);
    env.ctx.commit(&mut action);

    assert!(env.sh.get(start_coord).solid());

    // move original entity
    action.entity_mut(id_a).insert_position(end_coord);

    env.sh.update(&env.ctx, &action, 0);
    env.ctx.commit(&mut action);

    assert!(env.sh.get(start_coord).solid());
    assert!(env.sh.get(end_coord).solid());

    // move second entity
    action.entity_mut(id_b).insert_position(end_coord);

    env.sh.update(&env.ctx, &action, 0);
    env.ctx.commit(&mut action);

    assert!(!env.sh.get(start_coord).solid());
    assert!(env.sh.get(end_coord).solid());

    // move both entities in single action
    action.entity_mut(id_a).insert_position(start_coord);
    action.entity_mut(id_b).insert_position(start_coord);

    env.sh.update(&env.ctx, &action, 0);
    env.ctx.commit(&mut action);

    assert!(env.sh.get(start_coord).solid());
    assert!(!env.sh.get(end_coord).solid());
}

#[test]
fn entity_set() {
    let mut env = Env::new();

    let mut action = EcsAction::new();

    let coord_a = Coord::new(1, 2);
    let coord_b = Coord::new(1, 3);

    let id_a = {
        let mut entity = action.entity_mut(env.ids.reserve());
        entity.insert_position(coord_a);
        entity.id()
    };

    let id_b = {
        let mut entity = action.entity_mut(env.ids.reserve());
        entity.insert_position(coord_a);
        entity.id()
    };

    assert!(env.sh.get(coord_a).entities().is_empty());

    env.sh.update(&env.ctx, &action, 0);
    env.ctx.commit(&mut action);

    {
        let entities = env.sh.get(coord_a).entities();
        assert!(entities.contains(&id_a));
        assert!(entities.contains(&id_b));
        assert!(entities.len() == 2);
    }

    action.entity_mut(id_b).insert_position(coord_b);
    env.sh.update(&env.ctx, &action, 0);
    env.ctx.commit(&mut action);

    {
        let entities_a = env.sh.get(coord_a).entities();
        let entities_b = env.sh.get(coord_b).entities();

        assert!(entities_a.len() == 1);
        assert!(entities_b.len() == 1);
        assert!(entities_a.contains(&id_a));
        assert!(entities_b.contains(&id_b));
    }
}
