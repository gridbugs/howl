use rand::Rng;

use content_types::*;
use ecs_content::*;

pub trait EntityExtra {
    fn player_max_speed(&self) -> Option<usize>;
    fn general_max_speed(&self) -> Option<usize>;
    fn armour_hit_chance(&self) -> Option<Option<f64>>;
    fn steer_chance(&self) -> Option<f64>;
    fn damage_type<R: Rng>(&self, r: &mut R) -> Option<DamageType>;
    fn steer_check<R: Rng>(&self, r: &mut R) -> Option<bool> {
        self.steer_chance().map(|chance| r.next_f64() < chance)
    }
}

impl<'a> EntityExtra for EntityRef<'a> {
    fn player_max_speed(&self) -> Option<usize> {
        self.copy_engine_health().map(|hp| (hp.ucurrent() + 1) / 2)
    }
    fn general_max_speed(&self) -> Option<usize> {
        self.copy_max_speed().or_else(|| self.player_max_speed())
    }
    fn armour_hit_chance(&self) -> Option<Option<f64>> {
        self.copy_armour().map(|a| {
            if a != 0 {
                Some(1.0 / (a + 1) as f64)
            } else {
                None
            }
        })
    }
    fn steer_chance(&self) -> Option<f64> {
        self.tyre_health().map(|t| (t.ucurrent() + 1) as f64 / (t.umax() + 1) as f64)
    }
    fn damage_type<R: Rng>(&self, r: &mut R) -> Option<DamageType> {
        self.armour_hit_chance().and_then(|maybe_chance| {
            if let Some(chance) = maybe_chance {
                if r.next_f64() < chance {
                    Some(DamageType::Armour)
                } else {
                    Some(DamageType::Deflect)
                }
            } else {
                self.engine_health().and_then(|engine| self.tyre_health().and_then(|tyres| self.hit_points().and_then(|health| {
                    let total = engine.ucurrent() + tyres.ucurrent() + health.ucurrent();
                    if total == 0 {
                        return None;
                    }
                    let mut roll = r.gen::<usize>() % total;
                    if roll < engine.ucurrent() {
                        return Some(DamageType::Engine);
                    }
                    roll -= engine.ucurrent();
                    if roll < tyres.ucurrent() {
                        return Some(DamageType::Tyres);
                    }
                    Some(DamageType::Health)
                })))
            }
        })
    }
}
