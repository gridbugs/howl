use search::{
    Query,
    Traverse,
    TraverseType,
    WeightedGridSearchContext,
    SearchContext,
};
use search::TraverseType::*;
use grid::{
    Coord,
    StaticGrid,
    CopyGrid,
};

#[derive(Clone, Copy, Debug)]
struct Cell {
    ch: char,
}

impl Traverse for Cell {
    fn get_type(&self) -> TraverseType {
        match self.ch {
            '#' => NonTraversable,
            '1' => Traversable(1.0),
            '2' => Traversable(2.0),
            '3' => Traversable(3.0),
            '4' => Traversable(4.0),
            '5' => Traversable(5.0),
            '6' => Traversable(6.0),
            '7' => Traversable(7.0),
            '8' => Traversable(8.0),
            '9' => Traversable(9.0),
            _ => unimplemented!()
        }
    }
}

fn grid_a() -> StaticGrid<Cell> {
    let strings = [
        "#########",
        "#1299811#",
        "#1223811#",
        "#1223811#",
        "#1111811#",
        "#1111811#",
        "#1111811#",
        "#########",
    ];

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
    let query_a: Query<Cell> = Query::new_to_coord(
                                Coord::new(1, 1),
                                Coord::new(7, 1));

    let query_b: Query<Cell> =
        Query::new_to_predicate(Coord::new(1, 1), |info| {
        info.coord == Coord::new(7, 1)
    });

    let ctx = WeightedGridSearchContext::new();

    let grid = grid_a();

    let result_a = ctx.search(&grid, &query_a).unwrap();
    let result_b = ctx.search(&grid, &query_b).unwrap();

    assert_eq!((result_a.cost * 100.0).floor(), (result_b.cost * 100.0).floor());
}
