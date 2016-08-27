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
}
