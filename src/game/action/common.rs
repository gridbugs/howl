use std::f64;
use rand::Rng;
use ecs::*;
use game::*;
use game::data::*;
use direction::Direction;
use coord::Coord;
use math::Vector2;

pub fn walk(action: &mut EcsAction, entity: EntityRef, direction: Direction) {
    let current_position = entity.copy_position().expect("Entity missing position");
    let new_position = current_position + direction.vector();
    action.insert_position(entity.id(), new_position);
}

pub fn realtime_velocity_start(action: &mut EcsAction, entity: EntityRef, velocity: RealtimeVelocity, moves: usize) {
    action.insert_realtime_velocity(entity.id(), velocity);
    action.insert_realtime_moves_remaining(entity.id(), moves);
    action.insert_property_start_realtime_move();
}

pub fn realtime_velocity_stop(action: &mut EcsAction, entity_id: EntityId) {
    action.delete_realtime_velocity(entity_id);
    action.delete_realtime_moves_remaining(entity_id);
}

pub fn realtime_velocity_move(action: &mut EcsAction, entity: EntityRef, velocity: RealtimeVelocity) {

    if let Some(current_position) = entity.copy_position() {
        let current_velocity = entity.realtime_velocity().expect("Entity missing realtime_velocity");

        let (new_velocity, offset) = current_velocity.step();

        action.insert_realtime_velocity(entity.id(), new_velocity);
        action.insert_position(entity.id(), current_position + offset);

        if let Some(remaining) = entity.copy_realtime_moves_remaining() {
            if remaining > 0 {
                action.insert_realtime_moves_remaining(entity.id(), remaining - 1);
            }
        }

        let length = (offset.length_squared() as f64).sqrt();
        let delay = (velocity.ms_per_cell() as f64 * length) as u64;

        action.insert_property_action_time_ms(delay);
    }
}

pub fn destroy(action: &mut EcsAction, entity: EntityRef) {
    if let Some(ticket) = entity.copy_schedule_ticket() {
        action.insert_property_schedule_invalidate(ticket);
    }
    action.entity_delete(entity);
}

pub fn level_switch(action: &mut EcsAction, entity_id: EntityId, exit_id: EntityId, level_switch: LevelSwitch) {
    action.insert_property_level_switch_action(LevelSwitchAction {
        entity_id: entity_id,
        exit_id: exit_id,
        level_switch: level_switch,
    });
}

pub fn try_level_switch(action: &mut EcsAction, entity_id: EntityId) {
    action.insert_property_try_level_switch(entity_id);
}

pub fn projectile_collision(action: &mut EcsAction, projectile_collision: ProjectileCollision, ecs: &EcsCtx) {
    if ecs.contains_pc(projectile_collision.collider_id) && ecs.contains_bullet(projectile_collision.projectile_id) {
        let position = ecs.get_copy_position(projectile_collision.collider_id).expect("Missing component position");
        let message = if let Some(name) = ecs.get_copy_shooter_id(projectile_collision.projectile_id).and_then(|id| ecs.get_copy_name(id)) {
            ActionMessageType::ShotBy(name)
        } else {
            ActionMessageType::Shot
        };
        action.insert_property_action_description(ActionDescription::new(position, message));
    }
    action.insert_property_projectile_collision(projectile_collision);
    action.insert_property_no_commit();
}

pub fn damage(action: &mut EcsAction, to_damage: EntityRef, amount: usize) {

    if let Some(mut hit_points) = to_damage.copy_hit_points() {
        hit_points.dec(amount);
        action.insert_hit_points(to_damage.id(), hit_points);
    }
}

pub fn die(action: &mut EcsAction, entity: EntityRef) {
    if entity.contains_pc() {
        action.insert_property_player_died();
    } else {
        let ticket = entity.copy_schedule_ticket().expect("Entity missing schedule_ticket");
        action.insert_property_schedule_invalidate(ticket);
        action.entity_delete(entity);
    }
}

pub fn acid_animate<R: Rng>(action: &mut EcsAction, ecs: &EcsCtx, r: &mut R) {
    for id in ecs.id_iter_acid_animation() {

        // don't always change every tile
        if r.next_f64() > 0.5 {
            continue;
        }

        let animation = ecs.get_probabilistic_animation(id).expect("Entity missing probabilistic_animation");
        let tile = *animation.choose(r);
        action.insert_tile(id, tile);
    }
}

pub fn physics(action: &mut EcsAction) {
    action.insert_property_physics();
}

pub fn steer<R: Rng>(action: &mut EcsAction, entity: EntityRef, direction: SteerDirection, rng: &mut R) {
    if entity.contains_pc() {
        if entity.steer_check(rng).expect("Expected components for steer check") {
            action.insert_steering(entity.id(), direction);
        } else {
            let position = entity.copy_position().expect("Entity missing position");
            action.insert_property_action_description(ActionDescription::new(position, ActionMessageType::FailToTurn));
        }
    } else {
        action.insert_steering(entity.id(), direction);
    }
}

pub fn remove_steer(action: &mut EcsAction, entity_id: EntityId) {
    action.delete_steering(entity_id);
}

pub fn change_speed(action: &mut EcsAction, entity: EntityRef, change: ChangeSpeed) {
    let current_speed = entity.copy_current_speed().expect("Entity missing current_speed");
    let max_speed = entity.general_max_speed().expect("Entity missing max_speed");

    let new_speed = match change {
        ChangeSpeed::Accelerate => {
            if current_speed < max_speed {
                current_speed + 1
            } else {
                let position = entity.copy_position().expect("Entity missing position");
                action.insert_property_action_description(ActionDescription::new(position, ActionMessageType::FailToAccelerate));
                current_speed
            }
        }
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
    let position = entity.copy_position().expect("Missing component position");
    let ticket = entity.copy_schedule_ticket().expect("Entity missing schedule_ticket");
    action.insert_property_schedule_invalidate(ticket);
    action.entity_delete(entity);
    prototypes::bloodstain(action.entity_mut(ids.new_id()), position);
}

pub fn fire_burst<R: Rng>(action: &mut EcsAction, gun: EntityRef, shooter: EntityRef, direction: Direction, remaining: usize, speed: f64, period: u64, spread: usize, range: usize, bullet_type: BulletType, ids: &EntityIdReserver, r: &mut R) {

    let shooter_position = shooter.copy_position().expect("Missing component position");

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
        action.insert_property_then(Reaction::new(ActionArgs::FireBurst {
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
    action.insert_property_action_time_ms(period);
}

pub fn fire_gun<R: Rng>(action: &mut EcsAction, gun: EntityRef, shooter: EntityRef, direction: Direction, ids: &EntityIdReserver, r: &mut R) {
    let gun_type = gun.copy_gun_type().expect("Missing component gun_type");
    let range = gun.copy_gun_range().expect("Missing component gun_range");
    let shooter_position = shooter.copy_position().expect("Missing component position");
    match gun_type {
        GunType::Pistol => {
            const SPEED_CELLS_PER_SEC: f64 = 100.0;
            let mut velocity = RealtimeVelocity::new(direction.vector(), SPEED_CELLS_PER_SEC);
            let bullet_position = shooter_position + velocity.step_in_place();
            let id = ids.new_id();
            prototypes::bullet(action.entity_mut(id), bullet_position, velocity, range);
            action.insert_shooter_id(id, shooter.id());
            action.insert_property_action_time_ms(velocity.ms_per_cell());
        }
        GunType::Shotgun => {
            const SPEED_CELLS_PER_SEC: f64 = 50.0;
            const NUM_SHOTS: usize = 6;
            const SPREAD: usize = 2;
            let ideal_vector = direction.vector() * range as isize;
            for _ in 0..NUM_SHOTS {
                let x_spread = (r.gen::<usize>() % (SPREAD * 2)) as isize - SPREAD as isize;
                let y_spread = (r.gen::<usize>() % (SPREAD * 2)) as isize - SPREAD as isize;

                let vector = ideal_vector + Coord::new(x_spread, y_spread);
                let mut velocity = RealtimeVelocity::new(vector, SPEED_CELLS_PER_SEC);

                let bullet_position = shooter_position + velocity.step_in_place();
                let id = ids.new_id();
                prototypes::bullet(action.entity_mut(id), bullet_position, velocity, range);
                action.insert_shooter_id(id, shooter.id());
                action.insert_property_action_time_ms(velocity.ms_per_cell());
            }
        }
        GunType::MachineGun => {
            action.insert_property_then(Reaction::new(ActionArgs::FireBurst {
                gun_id: gun.id(),
                shooter_id: shooter.id(),
                direction: direction,
                remaining: 6,
                speed: 100.0,
                period: 20,
                spread: 1,
                range: range,
                bullet_type: BulletType::Bullet,
            }, 0));
        }
        GunType::Railgun => {
            action.insert_property_then(Reaction::new(ActionArgs::FireBurst {
                gun_id: gun.id(),
                shooter_id: shooter.id(),
                direction: direction,
                remaining: 20,
                speed: 200.0,
                period: 1,
                spread: 0,
                range: range,
                bullet_type: BulletType::RailgunSlug,
            }, 0));
        }
    }
}

pub fn complex_damage<R: Rng>(action: &mut EcsAction, entity: EntityRef, damage: usize, rng: &mut R) {
    let position = entity.copy_position().expect("Entity missing position");
    for _ in 0..damage {
        entity.damage_type(rng).map(|damage_type| match damage_type {
            DamageType::Health => {
                let mut hit_points = entity.copy_hit_points().expect("Entity missing hit_points");
                hit_points.dec(1);
                action.insert_hit_points(entity.id(), hit_points);
                action.insert_property_action_description(ActionDescription::new(position, ActionMessageType::PersonalDamage));
            }
            DamageType::Engine => {
                let mut engine = entity.copy_engine_health().expect("Entity missing engine_health");
                engine.dec(1);
                let new_max_speed = (engine.ucurrent() + 1) / 2;
                let max_speed = entity.general_max_speed().expect("Entity missing general_max_speed components");
                let current_speed = entity.copy_current_speed().expect("Entity missing current_speed");

                if new_max_speed < max_speed {
                    action.insert_property_action_description(ActionDescription::new(position, ActionMessageType::MaxSpeedDecreased));
                } else {
                    action.insert_property_action_description(ActionDescription::new(position, ActionMessageType::EngineDamage));
                }

                action.insert_engine_health(entity.id(), engine);

                if current_speed > new_max_speed {
                    action.insert_current_speed(entity.id(), new_max_speed);
                }
            }
            DamageType::Tyres => {
                let mut tyres = entity.copy_tyre_health().expect("Entity missing tyre_health");
                if tyres.ucurrent() > 0 {
                    tyres.dec(1);
                    action.insert_tyre_health(entity.id(), tyres);
                    action.insert_property_action_description(ActionDescription::new(position, ActionMessageType::TyreDamage));
                }
            }
            DamageType::Armour => {
                let armour = entity.copy_armour().expect("Entity missing armour");
                action.insert_armour(entity.id(), armour - 1);
                action.insert_property_action_description(ActionDescription::new(position, ActionMessageType::ArmourDamage));
            }
            DamageType::Deflect => {
                action.insert_property_action_description(ActionDescription::new(position, ActionMessageType::ArmourDeflect));
            }
        });
    }
}

pub fn bump(action: &mut EcsAction, victim: EntityRef, attacker: EntityRef) {
    if victim.contains_pc() {
        let position = victim.copy_position().expect("Entity missing position");
        if let Some(name) = attacker.copy_name() {
            if let Some(verb) = attacker.copy_bump_verb() {
                action.insert_property_action_description(ActionDescription::new(position, ActionMessageType::BumpedBy(name, verb)));
            }
        }
    }
}

pub fn acid_damage<R: Rng>(action: &mut EcsAction, entity: EntityRef, rng: &mut R) {
    const CHANCE_TO_DAMAGE: f64 = 0.25;

    if rng.next_f64() < CHANCE_TO_DAMAGE {
        let mut tyres = entity.copy_tyre_health().expect("Entity missing tyre_health");
        if tyres.current() > 0 {
            let position = entity.copy_position().expect("Entity missing position");
            action.insert_property_action_description(ActionDescription::new(position, ActionMessageType::TyreAcidDamage));
        }
        tyres.dec(1);
        action.insert_tyre_health(entity.id(), tyres);
    }
}

pub fn take_letter(action: &mut EcsAction, entity: EntityRef, letter: EntityRef) {
    let letter_count = entity.copy_letter_count().expect("Entity missing letter_count");
    action.insert_letter_count(entity.id(), letter_count + 1);
    action.entity_delete(letter);
}

pub fn explode(action: &mut EcsAction, entity: EntityRef) {
    if let Some(position) = entity.copy_position() {
        action.entity_delete(entity);
        action.insert_property_then(Reaction::new(ActionArgs::ExplodeSpawn(position), 0));
    }
}

pub fn explode_spawn(action: &mut EcsAction, coord: Coord, ids: &EntityIdReserver) {
    const SPEED_CELLS_PER_SEC: f64 = 20.0;
    const RANGE: usize = 6;
    const COUNT: usize = 32;
    const STEP: f64 = 2.0 * f64::consts::PI / COUNT as f64;
    let mut angle = 0.0;
    for _ in 0..COUNT {
        let v = Vector2::from_radial(10.0, angle);
        let velocity = RealtimeVelocity::new(Coord::new(v.x as isize, v.y as isize), SPEED_CELLS_PER_SEC);
        prototypes::explosion(action.entity_mut(ids.new_id()), coord, velocity, RANGE);
        angle += STEP;
    }
}

pub fn repair_tyre(action: &mut EcsAction, entity: EntityRef, amount: usize) {
    let mut tyres = entity.copy_tyre_health().expect("Entity missing tyre_health");
    tyres.inc(amount);
    action.insert_tyre_health(entity.id(), tyres);
    let position = entity.copy_position().expect("Entity missing position");
    action.insert_property_action_description(ActionDescription::new(position, ActionMessageType::TyreReplaced));
}

pub fn repair_engine(action: &mut EcsAction, entity: EntityRef, amount: usize) {
    let mut engine = entity.copy_engine_health().expect("Entity missing engine_health");
    engine.inc(amount);
    action.insert_engine_health(entity.id(), engine);
    let position = entity.copy_position().expect("Entity missing position");
    action.insert_property_action_description(ActionDescription::new(position, ActionMessageType::EngineRepaired));
}

pub fn consume(action: &mut EcsAction, entity: EntityRef, item: EntityRef) {
    let mut used = false;
    match item.copy_consumable_type().expect("Entity missing consumable_type") {
        ConsumableType::EngineRepairKit => {
            let engine = entity.engine_health().expect("Entity missing engine_health");
            if !engine.is_full() {
                action.insert_property_then(Reaction::new(ActionArgs::RepairEngine(entity.id(), 1), 0));
                used = true;
            }
        }
        ConsumableType::SpareTyre => {
            let tyres = entity.tyre_health().expect("Entity missing tyre_health");
            if !tyres.is_full() {
                action.insert_property_then(Reaction::new(ActionArgs::RepairTyre(entity.id(), 1), 0));
                used = true;
            }
        }
    }

    if used {
        entity.borrow_mut_inventory().expect("Entity missing inventory").remove(item.id());
        action.entity_delete(item);
    }
}
