use coord::{StraightLine, Coord};

pub enum RenderOverlay {
    AimLine(StraightLine),
    ExamineCursor(Coord),
    Death,
}
