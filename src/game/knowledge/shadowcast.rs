/// Implementation of recursive shadowcast

use std::cell::UnsafeCell;
use std::cmp;

use math::{Coord, Vector2, Vector2Index};
use direction::{Direction, CardinalDirection, OrdinalDirection, SubDirection};

use game::{SpatialHashTable, LevelKnowledge, Turn};


// Different types of rounding functions
enum RoundType {
    // Round down to the nearest integer
    Floor,

    // Round down to the nearest integer unless the given number
    // is already an integer, in which case subtract 1 from it
    ExclusiveFloor,
}

impl RoundType {
    fn round(&self, x: f64) -> isize {
        match *self {
            RoundType::Floor => x.floor() as isize,
            RoundType::ExclusiveFloor => (x - 1.0).ceil() as isize,
        }
    }
}

#[derive(PartialEq)]
enum RotationType {
    Clockwise,
    AntiClockwise,
}

const NUM_OCTANTS: usize = 8;

// Classification of an octant for shadowcast
struct Octant {
    // Direction to proceed with each scan
    depth_dir: Direction,

    // Direction to proceed during a scan
    lateral_dir: Direction,

    // Whether depth_dir is on x or y index
    depth_idx: Vector2Index,

    // Whether lateral_dir is on x or y index
    lateral_idx: Vector2Index,

    // Added to depth part of coord as depth increases
    depth_step: isize,

    // Added to lateral part of coord during scan.
    lateral_step: isize,

    // Copy of lateral_step, casted to a float.
    lateral_step_float: f64,

    // During a scan, if the current cell has more opacity than the
    // previous cell, use the gradient through this corner of the
    // current cell to split the visible area.
    opacity_increase_corner: OrdinalDirection,

    // During a scan, if the current cell has less opacity than the
    // previous cell, use the gradient through this corner of the
    // current cell to split the visible area.
    opacity_decrease_corner: OrdinalDirection,

    // Side of a cell in this octant  facing the eye
    facing_side: Direction,

    // Side of cell facing across eye
    across_side: Direction,

    // Corner of cell closest to eye
    facing_corner: OrdinalDirection,

    // Rounding function to use at the start of a scan to convert a
    // floating point derived from a gradient into part of a coord
    round_start: RoundType,

    // Rounding function to use at the end of a scan to convert a
    // floating point derived from a gradient into part of a coord
    round_end: RoundType,

    // Type of rotation during a scan in this octant
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
            lateral_step_float: lateral_step as f64,

            opacity_increase_corner: card_depth_dir.combine(card_lateral_dir.opposite())
                .unwrap(),

            opacity_decrease_corner: card_depth_dir.opposite()
                .combine(card_lateral_dir.opposite())
                .unwrap(),

            facing_side: card_facing_side.direction(),
            across_side: card_across_side.direction(),
            facing_corner: card_facing_side.combine(card_across_side).unwrap(),

            round_start: round_start,
            round_end: round_end,

            rotation: rotation,
        }
    }

    fn compute_slope(&self, from: Vector2<f64>, to: Vector2<f64>) -> f64 {
        ((to.get(self.lateral_idx) - from.get(self.lateral_idx)) /
         (to.get(self.depth_idx) - from.get(self.depth_idx)))
            .abs()
    }
}

#[derive(Debug)]
struct Frame {
    depth: usize,
    min_slope: f64,
    max_slope: f64,
    visibility: f64,
}

impl Frame {
    fn new(depth: usize, min_slope: f64, max_slope: f64, visibility: f64) -> Self {
        Frame {
            depth: depth,
            min_slope: min_slope,
            max_slope: max_slope,
            visibility: visibility,
        }
    }
}

struct Limits {
    // limiting coordinates of world
    depth_min: isize,
    depth_max: isize,
    lateral_min: isize,
    lateral_max: isize,

    // eye centre position
    eye_centre: Vector2<f64>,
    eye_lateral_pos: f64,

    // eye index
    eye_depth_idx: isize,
}

impl Limits {
    fn new(eye: Coord, world: &SpatialHashTable, octant: &Octant) -> Self {
        let eye_centre = eye.cell_centre();
        Limits {
            depth_min: world.limits_min().get(octant.depth_idx),
            depth_max: world.limits_max().get(octant.depth_idx),
            lateral_min: world.limits_min().get(octant.lateral_idx),
            lateral_max: world.limits_max().get(octant.lateral_idx),
            eye_centre: eye_centre,
            eye_lateral_pos: eye_centre.get(octant.lateral_idx),
            eye_depth_idx: eye.get(octant.depth_idx),
        }
    }
}

struct Scan<'a> {
    depth_idx: isize,
    start_lateral_idx: isize,
    end_lateral_idx: isize,
    limits: &'a Limits,
    frame: &'a Frame,
}

impl<'a> Scan<'a> {
    fn new(limits: &'a Limits,
           frame: &'a Frame,
           octant: &'a Octant,
           distance: usize)
           -> Option<Self> {
        assert!(frame.min_slope >= 0.0);
        assert!(frame.min_slope <= 1.0);
        assert!(frame.max_slope >= 0.0);
        assert!(frame.max_slope <= 1.0);

        // Don't scan past the view distance
        if frame.depth > distance {
            return None;
        }

        // Absolute index in depth direction of current row
        let depth_abs_idx = limits.eye_depth_idx + (frame.depth as isize) * octant.depth_step;

        // Don't scan off the edge of the world
        if depth_abs_idx < limits.depth_min || depth_abs_idx > limits.depth_max {
            return None;
        }

        // Offset of inner side of current row.
        // The 0.5 comes from the fact that the eye is in the centre of its cell.
        let inner_depth_offset = frame.depth as f64 - 0.5;

        // Offset of the outer side of the current row.
        // We add 1 to the inner offset, as row's are 1 unit wide.
        let outer_depth_offset = inner_depth_offset + 1.0;

        // Lateral index to start scanning from.
        // We always scan from from cardinal axis to ordinal axis.
        let rel_scan_start_idx = frame.min_slope * inner_depth_offset;
        let abs_scan_start_idx = octant.round_start
            .round(limits.eye_lateral_pos + rel_scan_start_idx * octant.lateral_step_float);

        // Make sure the scan starts inside the grid.
        // We always scan away from the eye in the lateral direction, so if the scan
        // starts off the grid, the entire scan will be off the grid, so can be skipped.
        if abs_scan_start_idx < limits.lateral_min || abs_scan_start_idx > limits.lateral_max {
            return None;
        }

        // Lateral index at which to stop scanning.
        let rel_scan_end_idx = frame.max_slope * outer_depth_offset;
        let abs_scan_end_idx = octant.round_end
            .round(limits.eye_lateral_pos + rel_scan_end_idx * octant.lateral_step_float);

        // Constrain the end of the scan within the limits of the grid
        let abs_scan_end_idx = cmp::min(cmp::max(abs_scan_end_idx, limits.lateral_min),
                                        limits.lateral_max);

        Some(Scan {
            depth_idx: depth_abs_idx,
            start_lateral_idx: abs_scan_start_idx,
            end_lateral_idx: abs_scan_end_idx,
            limits: limits,
            frame: frame,
        })
    }
}

struct OctantArgs<'a> {
    octant: &'a Octant,
    world: &'a SpatialHashTable,
    eye: Coord,
    distance: usize,
    distance_squared: isize,
    initial_min_slope: f64,
    initial_max_slope: f64,
}

impl<'a> OctantArgs<'a> {
    fn new(octant: &'a Octant,
           world: &'a SpatialHashTable,
           eye: Coord,
           distance: usize,
           initial_min_slope: f64,
           initial_max_slope: f64)
           -> Self {
        OctantArgs {
            octant: octant,
            world: world,
            eye: eye,
            distance: distance,
            distance_squared: (distance * distance) as isize,
            initial_min_slope: initial_min_slope,
            initial_max_slope: initial_max_slope,
        }
    }
}

pub struct Shadowcast {
    octants: [Octant; NUM_OCTANTS],
    stack: UnsafeCell<Vec<Frame>>,
}

impl Shadowcast {
    pub fn new() -> Self {
        Shadowcast {
            // The order octants appear is the order one would visit
            // each octant if they started at -PI radians and moved
            // in the positive (anticlockwise) direction.
            octants: [Octant::new(CardinalDirection::West, CardinalDirection::South),
                      Octant::new(CardinalDirection::South, CardinalDirection::West),
                      Octant::new(CardinalDirection::South, CardinalDirection::East),
                      Octant::new(CardinalDirection::East, CardinalDirection::South),
                      Octant::new(CardinalDirection::East, CardinalDirection::North),
                      Octant::new(CardinalDirection::North, CardinalDirection::East),
                      Octant::new(CardinalDirection::North, CardinalDirection::West),
                      Octant::new(CardinalDirection::West, CardinalDirection::North)],
            stack: UnsafeCell::new(Vec::new()),
        }
    }

    fn pop(&self) -> Option<Frame> {
        unsafe { &mut *self.stack.get() }.pop()
    }

    fn push(&self, frame: Frame) {
        unsafe { &mut *self.stack.get() }.push(frame);
    }

    fn scan<K: LevelKnowledge>(&self, args: &OctantArgs, scan: &Scan, knowledge: &mut K, turn: Turn) {
        let mut coord = Coord::new(0, 0);
        coord.set(args.octant.depth_idx, scan.depth_idx);

        let mut first_iteration = true;
        let mut previous_opaque = false;
        let mut previous_visibility = -1.0;
        let mut idx = scan.start_lateral_idx;
        let mut min_slope = scan.frame.min_slope;

        let final_idx = scan.end_lateral_idx + args.octant.lateral_step;

        while idx != final_idx {

            let last_iteration = idx == scan.end_lateral_idx;

            // update the coord to the current grid position
            coord.set(args.octant.lateral_idx, idx);

            // look up spatial hash cell
            let cell = args.world.get(coord);

            // report the cell as visible
            if (coord - args.eye).length_squared() < args.distance_squared {
                knowledge.update_cell(coord, cell, scan.frame.visibility, turn);
            }

            // compute current visibility
            let current_visibility = (scan.frame.visibility - cell.opacity()).max(0.0);
            let current_opaque = current_visibility == 0.0;

            // process changes in visibility
            if !first_iteration {
                // determine corner of current cell we'll be looking through
                let corner = if current_visibility > previous_visibility {
                    Some(args.octant.opacity_decrease_corner)
                } else if current_visibility < previous_visibility {
                    Some(args.octant.opacity_increase_corner)
                } else {
                    // no change in visibility - nothing happens
                    None
                };

                if let Some(corner) = corner {
                    let corner_coord = coord.cell_corner(corner);
                    let slope = args.octant.compute_slope(scan.limits.eye_centre, corner_coord);
                    assert!(slope >= 0.0);
                    assert!(slope <= 1.0);

                    if !previous_opaque {
                        // unless this marks the end of an opaque region, push
                        // the just-completed region onto the stack so it can
                        // be expanded in a future scan
                        self.push(Frame::new(scan.frame.depth + 1,
                                             min_slope,
                                             slope,
                                             previous_visibility));
                    }

                    min_slope = slope;
                }
            }

            if last_iteration && !current_opaque {
                // push the final region of the scan to the stack
                self.push(Frame::new(scan.frame.depth + 1,
                                     min_slope,
                                     scan.frame.max_slope,
                                     current_visibility));
            }

            previous_opaque = current_opaque;
            previous_visibility = current_visibility;
            first_iteration = false;

            idx += args.octant.lateral_step;
        }

    }

    fn detect_visible_area_octant<K: LevelKnowledge>(&self, args: &OctantArgs, knowledge: &mut K, turn: Turn) {
        let limits = Limits::new(args.eye, args.world, args.octant);

        // Initial stack frame
        self.push(Frame::new(1, args.initial_min_slope, args.initial_max_slope, 1.0));

        while let Some(frame) = self.pop() {
            if let Some(scan) = Scan::new(&limits, &frame, args.octant, args.distance) {
                // Scan::new can yield None if the scan would be entirely off the grid
                // outside the view distance.
                self.scan(args, &scan, knowledge, turn);
            }
        }
    }

    pub fn observe<K: LevelKnowledge>(&self,
                                      eye: Coord,
                                      world: &SpatialHashTable,
                                      distance: usize,
                                      knowledge: &mut K,
                                      turn: Turn) {

        knowledge.update_cell(eye, world.get(eye), 1.0, turn);

        for octant in &self.octants {
            let args = OctantArgs::new(octant, world, eye, distance, 0.0, 1.0);
            self.detect_visible_area_octant(&args, knowledge, turn);
        }
    }
}
