use game::RangeType;
use coord::StraightLine;

pub struct AimLine {
    pub line: StraightLine,
    pub range: RangeType,
}

pub struct RenderOverlay {
    pub aim_line: Option<AimLine>,
}
