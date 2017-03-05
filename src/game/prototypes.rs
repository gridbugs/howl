use std::ops::DerefMut;
use std::cmp;

use coord::Coord;
use direction::Direction;

use ecs::*;
use game::*;
use game::data::*;

pub const ENV_TURN_OFFSET: u64 = 0;
pub const NPC_TURN_OFFSET: u64 = 1;
pub const PC_TURN_OFFSET: u64 = 2;
pub const PHYSICS_TURN_OFFSET: u64 = 3;
pub const ANIMATION_TURN_OFFSET: u64 = 4;

pub fn pc<E: EntityPopulate>(mut entity: E, position: Coord) -> E {
    entity.insert_position(position);

    entity.insert_tile(TileType::Van);

    entity.insert_tile_depth(2);
    entity.insert_collider();
    entity.insert_behaviour_state(BehaviourState::new());
    entity.insert_behaviour_type(BehaviourType::PlayerInput);
    entity.insert_turn_offset(PC_TURN_OFFSET);
    entity.insert_drawable_knowledge(DrawableKnowledge::new());
    entity.insert_vision_distance(cmp::max(GAME_WIDTH, GAME_HEIGHT));
    entity.insert_pc();
    entity.insert_turn_time(TURN_DURATION_BASE);
    entity.insert_should_render();
    entity.insert_message_log(MessageLog::new());
    entity.insert_projectile_collider();
    entity.insert_hit_points(HitPoints::new(10));
    entity.insert_bump_attackable();
    entity.insert_weapon_slots(DirectionTable::new());
    entity.insert_bank(0);

    entity.insert_can_run_over();

    entity.insert_current_speed(1);
    entity.insert_max_speed(3);
    entity.insert_redline_speed(3);
    entity.insert_num_tires(4);
    entity.insert_max_tires(4);
    entity.insert_facing(Direction::East);

    entity.insert_inventory(EntitySet::new());
    entity.insert_inventory_capacity(8);

    entity
}

pub fn shop<E: EntityPopulate>(mut entity: E, inventory: EntitySet) -> E {

    entity.insert_inventory(inventory);

    entity
}

pub fn zombie<E: EntityPopulate>(mut entity: E, position: Coord) -> E {
    entity.insert_position(position);

    entity.insert_tile(TileType::Zombie);

    entity.insert_tile_depth(2);
    entity.insert_collider();
    entity.insert_behaviour_state(BehaviourState::new());
    entity.insert_behaviour_type(BehaviourType::SimpleNpc);
    entity.insert_turn_offset(NPC_TURN_OFFSET);
    entity.insert_vision_distance(8);
    entity.insert_simple_npc_knowledge(SimpleNpcKnowledge::new());
    entity.insert_path_traverse(PathTraverse::new());
    entity.insert_turn_time(TURN_DURATION_BASE * 2);
    entity.insert_enemy();
    entity.insert_projectile_collider();
    entity.insert_hit_points(HitPoints::new(2));
    entity.insert_bump_attacker(1);

    entity.insert_can_be_run_over();
    entity.insert_bloodstain_on_death();

    entity
}

pub fn bullet<E: EntityPopulate>(mut entity: E, position: Coord, velocity: RealtimeVelocity, range: usize) -> E {

    entity.insert_position(position);
    entity.insert_realtime_velocity(velocity);
    entity.insert_realtime_moves_remaining(range);
    entity.insert_destroy_on_collision();
    entity.insert_projectile();
    entity.insert_collider();
    entity.insert_projectile_damage(1);
    entity.insert_destroy_when_stopped();

    entity.insert_tile(TileType::Bullet);

    entity.insert_tile_depth(1);

    entity
}

pub fn road<E: EntityPopulate>(mut entity: E, position: Coord, rng: &GameRng) -> E {
    entity.insert_position(position);

    let rest_tiles = [];

    let tile = *rng.select_or_select_uniform(0.95, &TileType::Road0, &rest_tiles);

    entity.insert_tile(tile);
    entity.insert_tile_depth(0);
    entity.insert_floor();

    entity
}

pub fn dirt<E: EntityPopulate>(mut entity: E, position: Coord, rng: &GameRng) -> E {
    entity.insert_position(position);

    let rest_tiles = [
        TileType::Dirt1,
    ];

    let tile = *rng.select_or_select_uniform(0.95, &TileType::Dirt0, &rest_tiles);

    entity.insert_tile(tile);
    entity.insert_tile_depth(0);
    entity.insert_floor();

    entity
}

pub fn acid<E: EntityPopulate>(mut entity: E, position: Coord, rng: &GameRng) -> E {
    entity.insert_position(position);

    let animation = FirstWeightedProbabilisticChoice::new(0.90, TileType::Acid0, vec![TileType::Acid1]);

    entity.insert_tile(*animation.choose(rng.inner_mut().deref_mut()));
    entity.insert_probabilistic_animation(animation);
    entity.insert_acid_animation();
    entity.insert_tile_depth(0);
    entity.insert_floor();

    entity
}

pub fn wreck<E: EntityPopulate>(mut entity: E, position: Coord, rng: &GameRng) -> E {
    entity.insert_position(position);

    let tiles = [
        TileType::Wreck0,
        TileType::Wreck1,
        TileType::Wreck2,
    ];

    let tile = *rng.select_uniform(&tiles);

    entity.insert_tile(tile);
    entity.insert_tile_depth(0);
    entity.insert_solid();

    entity
}

pub fn acid_animator<E: EntityPopulate>(mut entity: E) -> E {
    entity.insert_behaviour_type(BehaviourType::AcidAnimate);
    entity.insert_behaviour_state(BehaviourState::new());
    entity.insert_turn_time(TURN_DURATION_BASE);
    entity.insert_turn_offset(ANIMATION_TURN_OFFSET);

    entity
}

pub fn physics<E: EntityPopulate>(mut entity: E) -> E {
    entity.insert_behaviour_type(BehaviourType::Physics);
    entity.insert_behaviour_state(BehaviourState::new());
    entity.insert_turn_time(TURN_DURATION_BASE);
    entity.insert_turn_offset(PHYSICS_TURN_OFFSET);

    entity
}

pub fn bloodstain<E: EntityPopulate>(mut entity: E, position: Coord) -> E {
    entity.insert_position(position);
    entity.insert_tile(TileType::Bloodstain);
    entity.insert_tile_depth(1);

    entity
}

pub fn pistol<E: EntityPopulate>(mut entity: E) -> E {

    entity.insert_gun_type(GunType::Pistol);
    entity.insert_name(NameMessageType::Pistol);
    entity.insert_description(DescriptionMessageType::Pistol);

    entity
}

pub fn shotgun<E: EntityPopulate>(mut entity: E) -> E {

    entity.insert_gun_type(GunType::Shotgun);
    entity.insert_name(NameMessageType::Shotgun);
    entity.insert_description(DescriptionMessageType::Shotgun);

    entity
}

pub fn machine_gun<E: EntityPopulate>(mut entity: E) -> E {

    entity.insert_gun_type(GunType::MachineGun);
    entity.insert_name(NameMessageType::MachineGun);
    entity.insert_description(DescriptionMessageType::MachineGun);

    entity
}

pub fn railgun<E: EntityPopulate>(mut entity: E) -> E {

    entity.insert_gun_type(GunType::Railgun);
    entity.insert_name(NameMessageType::Railgun);
    entity.insert_description(DescriptionMessageType::Railgun);

    entity
}

pub fn goal<E: EntityPopulate>(mut entity: E, position: Coord, level_switch: LevelSwitch) -> E {
    entity.insert_position(position);
    entity.insert_level_switch(level_switch);
    entity.insert_level_switch_auto();

    entity
}
