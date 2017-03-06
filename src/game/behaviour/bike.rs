use game::*;
use game::data::*;
use behaviour::LeafResolution;
use direction::Direction;
use coord::Coord;

const RANGE: isize = 8;

pub fn bike_chace<K: KnowledgeRenderer>() -> BehaviourLeaf<K> {
    BehaviourLeaf::new(move |input| {

        let position = input.entity.position().unwrap();
        let knowledge = input.entity.simple_npc_knowledge_borrow().unwrap();
        let level_knowledge = knowledge.level(input.level_id);
        let speed = input.entity.current_speed().unwrap();
        let max_speed = input.entity.max_speed().unwrap();
        let weapon_slots = input.entity.weapon_slots_borrow().unwrap();

        let target = if let Some(t) = level_knowledge.any_target() {
            t
        } else {
            // if you can't see a target, do nothing
            return LeafResolution::Yield(MetaAction::ActionArgs(ActionArgs::ChangeSpeed(input.entity.id(), ChangeSpeed::Decelerate)))
        };

        let next_coord = position + Coord::new(1, 0);
        if level_knowledge.get_with_default(next_coord).solid() {
            if position.y > target.y + RANGE {
                return LeafResolution::Yield(MetaAction::ActionArgs(ActionArgs::Steer(input.entity.id(), SteerDirection::Up)))
            } else {
                return LeafResolution::Yield(MetaAction::ActionArgs(ActionArgs::Steer(input.entity.id(), SteerDirection::Down)))
            }
        }

        if position.x > target.x {
            // we are in front of the target

            if position.y == target.y && position.x - target.x < RANGE {
                let gun_id = *weapon_slots.get(Direction::North).unwrap();
                return LeafResolution::Yield(MetaAction::ActionArgs(ActionArgs::FireGun {
                    gun_id: gun_id,
                    shooter_id: input.entity.id(),
                    direction: Direction::West,
                }));
            }

            if position.x - target.x > (position.y - target.y).abs() as isize {
                if position.y > target.y {
                    return LeafResolution::Yield(MetaAction::ActionArgs(ActionArgs::Steer(input.entity.id(), SteerDirection::Up)))
                } else if position.y < target.y {
                    return LeafResolution::Yield(MetaAction::ActionArgs(ActionArgs::Steer(input.entity.id(), SteerDirection::Down)))
                }
            }

            if speed > 1 {
                return LeafResolution::Yield(MetaAction::ActionArgs(ActionArgs::ChangeSpeed(input.entity.id(), ChangeSpeed::Decelerate)))
            }

            if position.y > target.y + RANGE {
                if speed == 0 {
                    return LeafResolution::Yield(MetaAction::ActionArgs(ActionArgs::ChangeSpeed(input.entity.id(), ChangeSpeed::Accelerate)))
                }
                return LeafResolution::Yield(MetaAction::ActionArgs(ActionArgs::Steer(input.entity.id(), SteerDirection::Up)))
            } else if position.y < target.y - RANGE {
                if speed == 0 {
                    return LeafResolution::Yield(MetaAction::ActionArgs(ActionArgs::ChangeSpeed(input.entity.id(), ChangeSpeed::Accelerate)))
                }
                return LeafResolution::Yield(MetaAction::ActionArgs(ActionArgs::Steer(input.entity.id(), SteerDirection::Down)))
            }

        } else if position.x < target.x {
            // we are behind the target

            if position.y == target.y && target.x - position.x < RANGE {
                let gun_id = *weapon_slots.get(Direction::North).unwrap();
                return LeafResolution::Yield(MetaAction::ActionArgs(ActionArgs::FireGun {
                    gun_id: gun_id,
                    shooter_id: input.entity.id(),
                    direction: Direction::East,
                }));
            }

            if position.x - target.x > (position.y - target.y).abs() as isize {
                if position.y > target.y {
                    return LeafResolution::Yield(MetaAction::ActionArgs(ActionArgs::Steer(input.entity.id(), SteerDirection::Up)))
                } else if position.y < target.y {
                    return LeafResolution::Yield(MetaAction::ActionArgs(ActionArgs::Steer(input.entity.id(), SteerDirection::Down)))
                }
            }

            // speed up to our max speed
            if speed < max_speed {
                return LeafResolution::Yield(MetaAction::ActionArgs(ActionArgs::ChangeSpeed(input.entity.id(), ChangeSpeed::Accelerate)))
            }

            if position.y > target.y + RANGE {
                return LeafResolution::Yield(MetaAction::ActionArgs(ActionArgs::Steer(input.entity.id(), SteerDirection::Up)))
            } else if position.y < target.y - RANGE {
                return LeafResolution::Yield(MetaAction::ActionArgs(ActionArgs::Steer(input.entity.id(), SteerDirection::Down)))
            }

        } else {
            // we are lined up with the target

            if position.y > target.y + RANGE {
                return LeafResolution::Yield(MetaAction::ActionArgs(ActionArgs::Steer(input.entity.id(), SteerDirection::Up)))
            } else if position.y < target.y - RANGE {
                return LeafResolution::Yield(MetaAction::ActionArgs(ActionArgs::Steer(input.entity.id(), SteerDirection::Down)))
            }

            let weapon_slots = input.entity.weapon_slots_borrow().unwrap();

            if position.y > target.y + 1 {
                let gun_id = *weapon_slots.get(Direction::North).unwrap();
                return LeafResolution::Yield(MetaAction::ActionArgs(ActionArgs::FireGun {
                    gun_id: gun_id,
                    shooter_id: input.entity.id(),
                    direction: Direction::North,
                }));
            } else if position.y < target.y - 1 {
                let gun_id = *weapon_slots.get(Direction::South).unwrap();
                return LeafResolution::Yield(MetaAction::ActionArgs(ActionArgs::FireGun {
                    gun_id: gun_id,
                    shooter_id: input.entity.id(),
                    direction: Direction::South,
                }));
            } else {
                return LeafResolution::Yield(MetaAction::ActionArgs(ActionArgs::Steer(input.entity.id(), SteerDirection::Up)))
            }
        }

        LeafResolution::Yield(MetaAction::ActionArgs(ActionArgs::Null))
    })
}
