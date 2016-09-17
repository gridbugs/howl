use game::{
    Entity,
};

use game::update::{
    Metadatum,
    MetadatumType,
};

pub trait MetadataWrapper<'a> : Sized {

    fn get_metadata(self, md_type: MetadatumType) -> Option<&'a Metadatum>;

    fn has_metadata(self, md_type: MetadatumType) -> bool {
        self.get_metadata(md_type).is_some()
    }

    fn action_time(self) -> u64 {
        if let Some(&Metadatum::ActionTime(t)) =
            self.get_metadata(MetadatumType::ActionTime)
        {
            t
        } else {
            0
        }
    }

    fn turn_time(self) -> u64 {
        if let Some(&Metadatum::TurnTime(t)) =
            self.get_metadata(MetadatumType::TurnTime)
        {
            t
        } else {
            0
        }
    }

    fn is_axis_velocity(self) -> bool {
        self.has_metadata(MetadatumType::AxisVelocityMovement)
    }

    fn burst_fire(self) -> Option<(&'a Entity, u64, u64)> {
        if let Some(&Metadatum::BurstFire { ref prototype, count, period }) =
            self.get_metadata(MetadatumType::BurstFire)
        {
            Some((prototype, count, period))
        } else {
            None
        }
    }

    fn name(self) -> Option<&'static str> {
        if let Some(&Metadatum::Name(name)) =
            self.get_metadata(MetadatumType::Name)
        {
            Some(name)
        } else {
            None
        }
    }
}
