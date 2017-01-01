use game::RangeType;
use math::CoordLine;

pub struct AimLine {
    pub line: CoordLine,
    pub range: RangeType,
}

pub struct RenderOverlay {
    pub aim_line: Option<AimLine>,
}
