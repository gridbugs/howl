use std::cmp;
use rand::Rng;
use ecs::*;
use game::*;
use game::data::*;
use direction::Direction;
use coord::Coord;

pub fn walk(action: &mut EcsAction, entity: EntityRef, direction: Direction) {
    let current_position = entity.position().expect("Entity missing position");
    let new_position = current_position + direction.vector();
    action.insert_position(entity.id(), new_position);
}

pub fn realtime_velocity_start(action: &mut EcsAction, entity: EntityRef, velocity: RealtimeVelocity, moves: usize) {
    action.insert_realtime_velocity(entity.id(), velocity);
    action.insert_realtime_moves_remaining(entity.id(), moves);
    action.set_start_realtime_move();
}

pub fn realtime_velocity_stop(action: &mut EcsAction, entity_id: EntityId) {
    action.remove_realtime_velocity(entity_id);
    action.remove_realtime_moves_remaining(entity_id);
}

pub fn realtime_velocity_move(action: &mut EcsAction, entity: EntityRef, velocity: RealtimeVelocity) {

    let current_position = entity.position().expect("Entity missing position");
    let current_velocity = entity.realtime_velocity().expect("Entity missing realtime_velocity");

    let (new_velocity, offset) = current_velocity.step();

    action.insert_realtime_velocity(entity.id(), new_velocity);
    action.insert_position(entity.id(), current_position + offset);

    if let Some(remaining) = entity.realtime_moves_remaining() {
        if remaining > 0 {
            action.insert_realtime_moves_remaining(entity.id(), remaining - 1);
        }
    }

    action.set_action_time_ms(velocity.ms_per_cell());
}

pub fn destroy(action: &mut EcsAction, entity: EntityRef) {
    action.remove_entity(entity);
}

pub fn level_switch(action: &mut EcsAction, entity_id: EntityId, exit_id: EntityId, level_switch: LevelSwitch) {
    action.set_level_switch_action(LevelSwitchAction {
        entity_id: entity_id,
        exit_id: exit_id,
        level_switch: level_switch,
    });
}

pub fn try_level_switch(action: &mut EcsAction, entity_id: EntityId) {
    action.set_try_level_switch(entity_id);
}

pub fn projectile_collision(action: &mut EcsAction, projectile_collision: ProjectileCollision) {
    action.set_projectile_collision(projectile_collision);
    action.set_no_commit();
}

pub fn damage(action: &mut EcsAction, to_damage: EntityRef, amount: usize) {

    let mut hit_points = to_damage.hit_points().expect("Entity missing hit_points");

    hit_points.dec(amount);

    action.insert_hit_points(to_damage.id(), hit_points);
}

pub fn die(action: &mut EcsAction, entity: EntityRef) {
    if entity.contains_pc() {
        action.set_player_died();
    } else {
        let ticket = entity.schedule_ticket().expect("Entity missing schedule_ticket");
        action.set_schedule_invalidate(ticket);
        action.remove_entity(entity);
    }
}

pub fn acid_animate<R: Rng>(action: &mut EcsAction, ecs: &EcsCtx, r: &mut R) {
    for id in ecs.acid_animation_id_iter() {

        // don't always change every tile
        if r.next_f64() > 0.1 {
            continue;
        }

        let animation = ecs.probabilistic_animation(id).expect("Entity missing probabilistic_animation");
        let tile = *animation.choose(r);
        action.insert_tile(id, tile);
    }
}

pub fn physics(action: &mut EcsAction) {
    action.set_physics();
}

pub fn steer(action: &mut EcsAction, entity: EntityRef, direction: SteerDirection) {
    let current_position = entity.position().expect("Entity missing position");
    let new_position = current_position + Direction::from(direction).vector();
    action.insert_position(entity.id(), new_position);
    action.set_steer();
}

pub fn change_speed(action: &mut EcsAction, entity: EntityRef, change: ChangeSpeed) {
    let current_speed = entity.current_speed().expect("Entity missing current_speed");
    let max_speed = entity.max_speed().expect("Entity missing max_speed");

    let new_speed = match change {
        ChangeSpeed::Accelerate => cmp::min(current_speed + 1, max_speed),
        ChangeSpeed::Decelerate => {
            if current_speed == 0 {
                0
            } else {
                current_speed - 1
            }
        }
    };

    action.insert_current_speed(entity.id(), new_speed);
}

pub fn become_bloodstain(action: &mut EcsAction, entity: EntityRef, ids: &EntityIdReserver) {
    let position = entity.position().expect("Missing component position");
    let ticket = entity.schedule_ticket().expect("Entity missing schedule_ticket");
    action.set_schedule_invalidate(ticket);
    action.remove_entity(entity);
    prototypes::bloodstain(action.entity_mut(ids.new_id()), position);
}

pub fn fire_burst<R: Rng>(action: &mut EcsAction, gun: EntityRef, shooter: EntityRef, direction: Direction, remaining: usize, speed: f64, period: u64, spread: usize, range: usize, bullet_type: BulletType, ids: &EntityIdReserver, r: &mut R) {

    let shooter_position = shooter.position().expect("Missing component position");

    let ideal_vector = direction.vector() * range as isize;
    let x_spread = (r.gen::<usize>() % (spread * 2 + 1)) as isize - spread as isize;
    let y_spread = (r.gen::<usize>() % (spread * 2 + 1)) as isize - spread as isize;
    let vector = ideal_vector + Coord::new(x_spread, y_spread);
    let mut velocity = RealtimeVelocity::new(vector, speed);

    let bullet_position = shooter_position + velocity.step_in_place();

    let bullet_id = ids.new_id();
    prototypes::bullet(action.entity_mut(bullet_id), bullet_position, velocity, range);

    match bullet_type {
        BulletType::RailgunSlug => {
            if direction == Direction::East || direction == Direction::West {
                action.insert_tile(bullet_id, TileType::RailgunSlugHorizontal);
            } else if direction == Direction::North || direction == Direction::South {
                action.insert_tile(bullet_id, TileType::RailgunSlugVertical);
            } else {
                panic!("Invalid direction");
            }
        }
        _ => {}
    }

    let next_remaining = remaining - 1;

    if next_remaining > 0 {
        action.set_then(Reaction::new(ActionArgs::FireBurst {
            gun_id: gun.id(),
            shooter_id: shooter.id(),
            direction: direction,
            remaining: next_remaining,
            speed: speed,
            period: period,
            spread: spread,
            range: range,
            bullet_type: bullet_type,
        }, period));
    }
    action.set_action_time_ms(period);
}

pub fn fire_gun<R: Rng>(action: &mut EcsAction, gun: EntityRef, shooter: EntityRef, direction: Direction, ids: &EntityIdReserver, r: &mut R) {
    let gun_type = gun.gun_type().expect("Missing component gun_type");
    let shooter_position = shooter.position().expect("Missing component position");
    match gun_type {
        GunType::Pistol => {
            const SPEED_CELLS_PER_SEC: f64 = 100.0;
            const RANGE: usize = 20;
            let mut velocity = RealtimeVelocity::new(direction.vector(), SPEED_CELLS_PER_SEC);
            let bullet_position = shooter_position + velocity.step_in_place();
            prototypes::bullet(action.entity_mut(ids.new_id()), bullet_position, velocity, RANGE);
            action.set_action_time_ms(velocity.ms_per_cell());
        }
        GunType::Shotgun => {
            const SPEED_CELLS_PER_SEC: f64 = 100.0;
            const RANGE: usize = 6;
            const NUM_SHOTS: usize = 10;
            const SPREAD: usize = 3;
            let ideal_vector = direction.vector() * RANGE as isize;
            for _ in 0..NUM_SHOTS {
                let x_spread = (r.gen::<usize>() % (SPREAD * 2)) as isize - SPREAD as isize;
                let y_spread = (r.gen::<usize>() % (SPREAD * 2)) as isize - SPREAD as isize;

                let vector = ideal_vector + Coord::new(x_spread, y_spread);
                let mut velocity = RealtimeVelocity::new(vector, SPEED_CELLS_PER_SEC);

                let bullet_position = shooter_position + velocity.step_in_place();
                prototypes::bullet(action.entity_mut(ids.new_id()), bullet_position, velocity, RANGE);
                action.set_action_time_ms(velocity.ms_per_cell());
            }
        }
        GunType::MachineGun => {
            action.set_then(Reaction::new(ActionArgs::FireBurst {
                gun_id: gun.id(),
                shooter_id: shooter.id(),
                direction: direction,
                remaining: 6,
                speed: 100.0,
                period: 20,
                spread: 2,
                range: 10,
                bullet_type: BulletType::Bullet,
            }, 0));
        }
        GunType::Railgun => {
            action.set_then(Reaction::new(ActionArgs::FireBurst {
                gun_id: gun.id(),
                shooter_id: shooter.id(),
                direction: direction,
                remaining: 20,
                speed: 200.0,
                period: 1,
                spread: 0,
                range: 50,
                bullet_type: BulletType::RailgunSlug,
            }, 0));
        }
    }
}
