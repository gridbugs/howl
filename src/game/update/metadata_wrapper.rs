use game::{
    Entity,
    UpdateSummary,
};

use game::update::{
    Metadata,
    Metadatum,
    MetadatumType,
};

impl Metadata {
    pub fn action_time(&self) -> u64 {
        if let Some(&Metadatum::ActionTime(t)) =
            self.get(MetadatumType::ActionTime)
        {
            t
        } else {
            0
        }
    }

    pub fn turn_time(&self) -> u64 {
        if let Some(&Metadatum::TurnTime(t)) =
            self.get(MetadatumType::TurnTime)
        {
            t
        } else {
            0
        }
    }

    pub fn is_axis_velocity(&self) -> bool {
        self.has(MetadatumType::AxisVelocityMovement)
    }

    pub fn burst_fire(&self) -> Option<(&Entity, u64, u64)> {
        if let Some(&Metadatum::BurstFire { ref prototype, count, period }) =
            self.get(MetadatumType::BurstFire)
        {
            Some((prototype, count, period))
        } else {
            None
        }
    }

    pub fn delay(&self) -> Option<&UpdateSummary> {
        if let Some(&Metadatum::Delay(ref update)) =
            self.get(MetadatumType::Delay)
        {
            Some(update)
        } else {
            None
        }
    }

    pub fn name(&self) -> Option<&'static str> {
        if let Some(&Metadatum::Name(name)) =
            self.get(MetadatumType::Name)
        {
            Some(name)
        } else {
            None
        }
    }
}
