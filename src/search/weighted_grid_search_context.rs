use search::{
    Traverse,
    SearchContext,
    Query,
    Path,
    SearchError,
    CellInfo,
    Destination,
};
use grid::{
    Grid,
    Coord,
    StaticGrid,
    DefaultGrid,
};

use geometry::{
    direction,
    LengthSquared,
};

use std::collections::BinaryHeap;
use std::cell::RefCell;
use std::cmp::Ordering;

#[derive(Clone, Copy)]
struct SeenCell {
    seen_seq: u64,
    visited_seq: u64,
    cost: f64,
    parent: Option<Coord>,
}

impl Default for SeenCell {
    fn default() -> Self {
        SeenCell {
            seen_seq: 0,
            visited_seq: 0,
            cost: 0.0,
            parent: None,
        }
    }
}

struct SeenSet {
    grid: StaticGrid<SeenCell>,
    seq: u64,
    explored: u64,
}

impl SeenSet {
    fn new() -> Self {
        SeenSet {
            grid: StaticGrid::new_default(INITIAL_GRID_WIDTH, INITIAL_GRID_HEIGHT),
            seq: 0,
            explored: 0,
        }
    }

    fn clear(&mut self) {
        self.seq += 1;
        self.explored = 0;
    }

    fn see(&mut self, coord: Coord) {
        let mut cell = &mut self.grid[coord];
        cell.seen_seq = self.seq;
        cell.parent = None;
    }

    fn see_with_cost(&mut self, coord: Coord, cost: f64) -> bool {
        if self.is_visited(coord) {
            return false;
        }

        if let Some(existing_cost) = self.cost(coord) {
            if cost < existing_cost {
                self.grid[coord].cost = cost;
                true
            } else {
                false
            }
        } else {
            self.see(coord);
            self.grid[coord].cost = cost;
            true
        }
    }

    fn see_with_parent(&mut self, coord: Coord, cost: f64, parent_coord: Coord) -> bool {
        if self.see_with_cost(coord, cost) {
            self.grid[coord].parent = Some(parent_coord);
            true
        } else {
            false
        }
    }

    fn is_seen(&self, coord: Coord) -> bool {
        self.grid[coord].seen_seq == self.seq
    }

    fn cost(&self, coord: Coord) -> Option<f64> {
        if self.is_seen(coord) {
            Some(self.grid[coord].cost)
        } else {
            None
        }
    }

    fn visit(&mut self, coord: Coord) {
        self.grid[coord].visited_seq = self.seq;
        self.explored += 1;
    }

    fn is_visited(&self, coord: Coord) -> bool {
        self.grid[coord].visited_seq == self.seq
    }

    fn set_parent(&mut self, coord: Coord, parent_coord: Coord) {
        self.grid[coord].parent = Some(parent_coord);
    }

    fn parent(&mut self, coord: Coord) -> Option<Coord> {
        if self.is_seen(coord) {
            self.grid[coord].parent
        } else {
            None
        }
    }

    fn make_path(&self, start: Coord) -> Option<Path> {
        let mut path = Vec::new();
        let mut coord = start;

        let cost = self.grid[coord].cost;

        loop {
            if !self.is_visited(coord) {
                return None;
            }

            path.push(coord);

            if let Some(parent) = self.grid[coord].parent {
                coord = parent;
            } else {
                path.reverse();
                return Some(Path::new(path, cost, self.explored));
            }
        }
    }
}

struct Node {
    coord: Coord,
    cost: f64,
    score: f64,
}

impl Node {
    fn new(coord: Coord, cost: f64) -> Self {
        Node {
            coord: coord,
            cost: cost,
            score: cost,
        }
    }

    fn with_heuristic(coord: Coord, cost: f64, heuristic: f64) -> Self {
        Node {
            coord: coord,
            cost: cost,
            score: cost + heuristic,
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.score > other.score {
            Ordering::Less
        } else if self.score < other.score {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl Eq for Node {}



struct State {
    queue: BinaryHeap<Node>,
    seen: SeenSet,
}

impl State {
    fn new() -> Self {
        State {
            queue: BinaryHeap::new(),
            seen: SeenSet::new(),
        }
    }

    fn clear(&mut self) {
        self.queue.clear();
        self.seen.clear();
    }
}

pub struct WeightedGridSearchContext {
    state: RefCell<State>,
}

const INITIAL_GRID_WIDTH: usize = 100;
const INITIAL_GRID_HEIGHT: usize = 60;

impl WeightedGridSearchContext {
    pub fn new() -> Self {
        WeightedGridSearchContext {
            state: RefCell::new(State::new()),
        }
    }

    fn dijkstra_predicate_search<T: Traverse, G: Grid<Item=T>>(
        &self, grid: &G,
        start: Coord,
        predicate: &Box<Fn(CellInfo<T>) -> bool>) -> Option<Path>
    {
        let mut state = self.state.borrow_mut();
        state.clear();

        state.queue.push(Node::new(start, 0.0));
        state.seen.see_with_cost(start, 0.0);

        while let Some(node) = state.queue.pop() {

            if state.seen.is_visited(node.coord) {
                continue;
            }

            state.seen.visit(node.coord);

            let info = CellInfo::new(grid.get_unsafe(node.coord), node.coord);
            if predicate(info) {
                return Some(state.seen.make_path(node.coord).unwrap());
            }

            for dir in direction::iter() {
                let nei_coord = node.coord + dir.vector();
                let cell = grid.get_unsafe(nei_coord);
                if let Some(cost) = cell.cost() {
                    let total_cost = node.cost + cost * dir.multiplier();
                    if state.seen.see_with_parent(nei_coord, total_cost, node.coord) {
                        state.queue.push(Node::new(nei_coord, total_cost));
                    }
                }
            }
        }

        None
    }

    fn astar_coord_search<T: Traverse, G: Grid<Item=T>>(
        &self, grid: &G, start: Coord, dest: Coord) -> Option<Path>
    {
        let mut state = self.state.borrow_mut();
        state.clear();

        let start_heuristic = ((start - dest).len_sq() as f64).sqrt();
        state.queue.push(Node::with_heuristic(start, 0.0, start_heuristic));
        state.seen.see_with_cost(start, 0.0);

        while let Some(node) = state.queue.pop() {

            if state.seen.is_visited(node.coord) {
                continue;
            }

            state.seen.visit(node.coord);

            if node.coord == dest {
                return Some(state.seen.make_path(node.coord).unwrap());
            }

            for dir in direction::iter() {
                let nei_coord = node.coord + dir.vector();
                let cell = grid.get_unsafe(nei_coord);
                if let Some(cost) = cell.cost() {
                    let total_cost = node.cost + cost * dir.multiplier();
                    if state.seen.see_with_parent(nei_coord, total_cost, node.coord) {
                        let heuristic = ((nei_coord - dest).len_sq() as f64).sqrt();
                        state.queue.push(Node::with_heuristic(nei_coord, total_cost, heuristic));
                    }
                }
            }
        }

        None
    }
}


impl SearchContext for WeightedGridSearchContext {
    fn search<T: Traverse, G: Grid<Item=T>>(&self, grid: &G, query: &Query<T>) -> Result<Path, SearchError> {

        if let Some(initial_cell) = grid.get(query.start) {
            if !initial_cell.is_traversable() {
                return Err(SearchError::NonTraversableStart);
            }
        } else {
            return Err(SearchError::StartOutOfGrid);
        }

        let result = match &query.end {
            &Destination::Predicate(ref predicate) => {
                self.dijkstra_predicate_search(grid, query.start, predicate)
            },
            &Destination::Coord(coord) => {
                self.astar_coord_search(grid, query.start, coord)
            },
        };

        if let Some(path) = result {
            Ok(path)
        } else {
            Err(SearchError::Exhausted)
        }
    }
}
