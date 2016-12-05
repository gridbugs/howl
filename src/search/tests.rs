use search::*;
use grid::*;
use math::Coord;

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
            let coord = Coord::new(j as isize, i as isize);
            grid.get_checked_mut(coord).ch = strings[i].chars().nth(j).unwrap();
        }
    }

    grid
}

fn dijkstra_a() -> f64 {

    let ctx = GridSearchCtx::new();
    let cfg = GridSearchCfg::all_directions();

    let grid = grid_a();

    let mut path = GridPath::new();

    ctx.search_predicate(&grid,
                         Coord::new(1, 1),
                         |c| c.coord == Coord::new(7, 1),
                         &cfg,
                         &mut path).unwrap();

    path.cost()
}

#[test]
fn dijkstra_optimality() {
    assert_eq!((dijkstra_a() * 100.0).floor(), 1724.0);
}
