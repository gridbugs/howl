use search::{WeightedGridSearchContext, Path, Config, TraverseCost};
use grid::{Coord, StaticGrid, CopyGrid};

#[derive(Clone, Copy, Debug)]
struct Cell {
    ch: char,
}

impl TraverseCost for Cell {
    fn traverse_cost(&self) -> Option<f64> {
        match self.ch {
            '#' => None,
            '1' => Some(1.0),
            '2' => Some(2.0),
            '3' => Some(3.0),
            '4' => Some(4.0),
            '5' => Some(5.0),
            '6' => Some(6.0),
            '7' => Some(7.0),
            '8' => Some(8.0),
            '9' => Some(9.0),
            _ => unimplemented!(),
        }
    }
}

fn grid_a() -> StaticGrid<Cell> {
    let strings = ["#########",
                   "#1299811#",
                   "#1223811#",
                   "#1223811#",
                   "#1111811#",
                   "#1111811#",
                   "#1111811#",
                   "#########"];

    let width = strings[0].len();
    let height = strings.len();
    let mut grid = StaticGrid::new_copy(width, height, Cell { ch: '#' });

    for i in 0..height {
        for j in 0..width {
            grid[(j, i)] = Cell { ch: strings[i].chars().nth(j).unwrap() };
        }
    }

    grid
}

#[test]
fn optimal_search() {
    let mut ctx = WeightedGridSearchContext::new();
    let grid = grid_a();

    let config = Config::new_all_directions();
    let mut path = Path::new();

    ctx.search_coord(&grid,
                      Coord::new(1, 1),
                      Coord::new(7, 1),
                      &config,
                      &mut path)
        .unwrap();
    let cost_a = path.cost;

    ctx.search_predicate(&grid,
                          Coord::new(1, 1),
                          |info| info.coord == Coord::new(7, 1),
                          &config,
                          &mut path)
        .unwrap();
    let cost_b = path.cost;

    assert_eq!((cost_a * 100.0).floor(), (cost_b * 100.0).floor());
}
