use game::{
    Entity,
    UpdateSummary,
};

use table::{
    Table,
    ToType,
};

pub type Metadata = Table<MetadatumType, Metadatum>;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum MetadatumType {
    TurnTime,
    ActionTime,
    AxisVelocityMovement,
    BurstFire,
    Delay,
}

#[derive(Clone)]
pub enum Metadatum {
    TurnTime(u64),
    ActionTime(u64),
    AxisVelocityMovement,
    BurstFire { prototype: Entity, count: u64, period: u64 },
    Delay(UpdateSummary),
}

impl ToType<MetadatumType> for Metadatum {
    fn to_type(&self) -> MetadatumType {
        match *self {
            Metadatum::TurnTime(_) => MetadatumType::TurnTime,
            Metadatum::ActionTime(_) => MetadatumType::ActionTime,
            Metadatum::AxisVelocityMovement => MetadatumType::AxisVelocityMovement,
            Metadatum::BurstFire { prototype: _, count: _, period: _ } => MetadatumType::BurstFire,
            Metadatum::Delay(_) => MetadatumType::Delay,
        }
    }
}
