use geometry::{
    Direction,
    CardinalDirection,
    SubDirection,
    Vector2Index,
};

/// Different types of rounding functions
enum RoundType {
    /// Round down to the nearest integer
    Floor,

    /// Round down to the nearest integer unless the given number
    /// is already an integer, in which case subtract 1 from it
    ExclusiveFloor,
}

impl RoundType {
    fn round(&self, x: f64) -> f64 {
        match *self {
            RoundType::Floor => x.floor(),
            RoundType::ExclusiveFloor => (x - 1.0).ceil(),
        }
    }
}

#[derive(PartialEq)]
enum RotationType {
    Clockwise,
    AntiClockwise,
}

const NUM_OCTANTS: usize = 8;

/// Classification of an octant for recursive shadowcast
struct Octant {
    /// Direction to proceed with each scan
    depth_dir: Direction,

    /// Direction to proceed during a scan
    lateral_dir: Direction,

    /// Whether depth_dir is on x or y index
    depth_idx: Vector2Index,

    /// Whether lateral_dir is on x or y index
    lateral_idx: Vector2Index,

    /// Added to depth part of coord as depth increases
    depth_step: isize,

    /// Added to lateral part of coord during scan
    lateral_step: isize,

    /// During a scan, if the current cell has more opacity than the
    /// previous cell, use the gradient through this corner of the
    /// current cell to split the visible area.
    opacity_increase_corner: Direction,

    /// During a scan, if the current cell has less opacity than the
    /// previous cell, use the gradient through this corner of the
    /// current cell to split the visible area.
    opacity_decrease_corner: Direction,

    /// Side of a cell in this octant  facing the eye
    facing_side: Direction,

    /// Side of cell facing across eye
    across_side: Direction,

    /// Corner of cell closest to eye
    facing_corner: Direction,

    /// Rounding function to use at the start of a scan to convert a
    /// floating point derived from a gradient into part of a coord
    round_start: RoundType,

    /// Rounding function to use at the end of a scan to convert a
    /// floating point derived from a gradient into part of a coord
    round_end: RoundType,

    /// Type of rotation during a scan in this octant
    rotation: RotationType,
}

impl Octant {
    fn new(card_depth_dir: CardinalDirection, card_lateral_dir: CardinalDirection) -> Self {
        let depth_dir = card_depth_dir.direction();
        let lateral_dir = card_lateral_dir.direction();

        let depth_step = depth_dir.vector().get(card_depth_dir.vector2_index());
        let lateral_step = lateral_dir.vector().get(card_lateral_dir.vector2_index());

        let card_facing_side = card_depth_dir.opposite();
        let card_across_side = card_lateral_dir.opposite();

        let (round_start, round_end) = if lateral_step == 1 {
            (RoundType::Floor, RoundType::ExclusiveFloor)
        } else {
            assert!(lateral_step == -1);
            (RoundType::ExclusiveFloor, RoundType::Floor)
        };

        let rotation = if lateral_dir == depth_dir.left90() {
            RotationType::Clockwise
        } else {
            assert!(depth_dir == lateral_dir.left90());
            RotationType::AntiClockwise
        };

        Octant {
            depth_dir: depth_dir,
            lateral_dir: lateral_dir,

            depth_idx: card_depth_dir.vector2_index(),
            lateral_idx: card_lateral_dir.vector2_index(),

            depth_step: depth_step,
            lateral_step: lateral_step,

            opacity_increase_corner: card_depth_dir
                .combine(card_lateral_dir.opposite()).unwrap().direction(),

            opacity_decrease_corner: card_depth_dir.opposite()
                .combine(card_lateral_dir.opposite()).unwrap().direction(),

            facing_side: card_facing_side.direction(),
            across_side: card_across_side.direction(),
            facing_corner: card_facing_side.combine(card_across_side).unwrap().direction(),

            round_start: round_start,
            round_end: round_end,

            rotation: rotation,
        }
    }
}

pub struct RecursiveShadowcast {
    octants: [Octant; NUM_OCTANTS],
}
