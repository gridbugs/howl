use game::{
    Entity,
    UpdateSummary,
};

use game::update::{
    Metadatum,
    MetadatumType,
};

use table::TableRef;

impl UpdateSummary {
    pub fn action_time(&self) -> u64 {
        if let Some(&Metadatum::ActionTime(t)) =
            self.metadata.get(MetadatumType::ActionTime)
        {
            t
        } else {
            0
        }
    }

    pub fn turn_time(&self) -> u64 {
        if let Some(&Metadatum::TurnTime(t)) =
            self.metadata.get(MetadatumType::TurnTime)
        {
            t
        } else {
            0
        }
    }

    pub fn is_axis_velocity(&self) -> bool {
        self.metadata.has(MetadatumType::AxisVelocityMovement)
    }

    pub fn burst_fire(&self) -> Option<(&Entity, u64, u64)> {
        if let Some(&Metadatum::BurstFire { ref prototype, count, period }) =
            self.metadata.get(MetadatumType::BurstFire)
        {
            Some((prototype, count, period))
        } else {
            None
        }
    }

    pub fn delay(&self) -> Option<&UpdateSummary> {
        if let Some(&Metadatum::Delay(ref update)) =
            self.metadata.get(MetadatumType::Delay)
        {
            Some(update)
        } else {
            None
        }
    }

    pub fn name(&self) -> Option<&'static str> {
        if let Some(&Metadatum::Name(name)) =
            self.metadata.get(MetadatumType::Name)
        {
            Some(name)
        } else {
            None
        }
    }
}
