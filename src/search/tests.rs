use search::*;
use grid::*;
use coord::Coord;

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

struct Env {
    ctx: GridSearchCtx,
    cfg: GridSearchCfg,
    path: GridPath,
}

impl Env {
    pub fn new() -> Self {
        Env {
            ctx: GridSearchCtx::new(),
            cfg: GridSearchCfg::all_directions(),
            path: GridPath::new(),
        }
    }
}

#[test]
fn dijkstra_optimality() {

    let mut env = Env::new();

    let grid = grid_a();

    env.ctx.search_predicate(&grid,
                         Coord::new(1, 1),
                         |c| c.coord == Coord::new(7, 1),
                         &env.cfg,
                         &mut env.path).unwrap();

    assert_eq!((env.path.cost() * 100.0).floor(), 1724.0);
}

#[test]
fn astar_optimality() {

    let mut env = Env::new();

    let grid = grid_a();

    env.ctx.search_coord(&grid,
                         Coord::new(1, 1),
                         Coord::new(7, 1),
                         &env.cfg,
                         &mut env.path).unwrap();

    assert_eq!((env.path.cost() * 100.0).floor(), 1724.0);
}
