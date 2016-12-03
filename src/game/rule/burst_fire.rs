use game::*;
use ecs::*;

const BURST_DELAY_MS: u64 = 100;

pub struct BurstFire;
impl Rule for BurstFire {
    fn check(&self, _env: RuleEnv, action: &EcsAction, resolution: &mut RuleResolution) -> Result<()> {

        if let Some(burst_fire) = action.burst_fire() {
            if burst_fire.count > 0 {
                resolution.add_reaction(Reaction::new(
                        ActionArgs::FireBullet(burst_fire.entity_id, burst_fire.direction), 0));

                if burst_fire.count > 1 {
                    resolution.add_reaction(Reaction::new(
                            ActionArgs::BurstBullets(burst_fire.entity_id, burst_fire.direction, burst_fire.count - 1),
                            BURST_DELAY_MS));
                }
            }
        }

        Ok(())
    }
}
