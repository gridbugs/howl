use game::{
    Entity,
    UpdateSummary,
    MetadataWrapper,
};

use table::{
    Table,
    ToType,
    TableRef,
};

pub type Metadata = Table<MetadatumType, Metadatum>;

impl<'a> MetadataWrapper<'a> for &'a Metadata {
    fn get_metadata(self, md_type: MetadatumType) -> Option<&'a Metadatum> {
        self.get(md_type)
    }

    fn has_metadata(self, md_type: MetadatumType) -> bool {
        self.has(md_type)
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum MetadatumType {
    Name,
    TurnTime,
    ActionTime,
    AxisVelocityMovement,
    BurstFire,
    Delay,
}

#[derive(Clone)]
pub enum Metadatum {
    Name(&'static str),
    TurnTime(u64),
    ActionTime(u64),
    AxisVelocityMovement,
    BurstFire { prototype: Entity, count: u64, period: u64 },
    Delay(UpdateSummary),
}

impl ToType<MetadatumType> for Metadatum {
    fn to_type(&self) -> MetadatumType {
        match *self {
            Metadatum::Name(_) => MetadatumType::Name,
            Metadatum::TurnTime(_) => MetadatumType::TurnTime,
            Metadatum::ActionTime(_) => MetadatumType::ActionTime,
            Metadatum::AxisVelocityMovement => MetadatumType::AxisVelocityMovement,
            Metadatum::BurstFire { prototype: _, count: _, period: _ } => MetadatumType::BurstFire,
            Metadatum::Delay(_) => MetadatumType::Delay,
        }
    }
}
