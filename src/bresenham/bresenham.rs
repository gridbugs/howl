use math::{Vector2Index, Coord, CoordLine};

#[derive(Debug)]
struct Octant {
    major_sign: isize,
    minor_sign: isize,
    major_axis: Vector2Index,
    minor_axis: Vector2Index,
}

fn choose_octant(delta: Coord) -> Octant {
    let (major_axis, minor_axis) = if delta.x.abs() > delta.y.abs() {
        (Vector2Index::X, Vector2Index::Y)
    } else {
        (Vector2Index::Y, Vector2Index::X)
    };

    let major_sign = if delta.get(major_axis) < 0 { -1 } else { 1 };
    let minor_sign = if delta.get(minor_axis) < 0 { -1 } else { 1 };

    Octant {
        major_sign: major_sign,
        minor_sign: minor_sign,
        major_axis: major_axis,
        minor_axis: minor_axis,
    }
}

pub fn make_line(src: Coord, dst: Coord, line: &mut CoordLine) {

    line.clear(src);

    if src == dst {
        return;
    }

    let delta = dst - src;

    let octant = choose_octant(delta);

    let major_delta = delta.get(octant.major_axis).abs();
    let minor_delta = delta.get(octant.minor_axis).abs();

    let mut minor_offset = 0;
    let mut numerator = 0;
    let half_major_delta = major_delta / 2;
    let mut coord = Coord::new(0, 0);
    let mut major_offset = 0;

    let src_major_coord = src.get(octant.major_axis);
    let src_minor_coord = src.get(octant.minor_axis);

    for _ in 1..major_delta {

        numerator += minor_delta;
        if numerator >= major_delta {
            numerator -= major_delta;
            minor_offset += octant.minor_sign;
        }

        major_offset += octant.major_sign;
        let major_coord = src_major_coord + major_offset;
        coord.set(octant.major_axis, major_coord);

        let mut minor_coord = src_minor_coord + minor_offset;
        if numerator > half_major_delta {
            minor_coord += octant.minor_sign;
        }

        coord.set(octant.minor_axis, minor_coord);

        line.extend(coord);
    }

    line.extend(dst);
}
