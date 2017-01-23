use coord::{StraightLine, Coord};

pub struct RenderOverlay {
    pub aim_line: Option<StraightLine>,
    pub examine_cursor: Option<Coord>,
}

impl RenderOverlay {
    pub fn aim_line(aim_line: StraightLine) -> Self {
        RenderOverlay {
            aim_line: Some(aim_line),
            examine_cursor: None,
        }
    }

    pub fn examine_cursor(examine_cursor: Coord) -> Self {
        RenderOverlay {
            aim_line: None,
            examine_cursor: Some(examine_cursor),
        }
    }
}
