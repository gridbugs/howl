use search::{Traverse, SearchContext, Query, Path, SearchError, CellInfo, Destination};

use search::tracker_grid::TrackerGrid;

use grid::{Grid, Coord};

use geometry::{Direction, LengthSquared};

use std::collections::BinaryHeap;
use std::cell::RefCell;
use std::cmp::Ordering;

#[derive(Debug)]
struct Node {
    coord: Coord,
    cost: f64,
    score: f64,
}

impl Node {
    fn new_dijkstra(coord: Coord, cost: f64) -> Self {
        Node {
            coord: coord,
            cost: cost,
            score: cost,
        }
    }

    fn new_astar(coord: Coord, cost: f64, heuristic: f64) -> Self {
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
    tracker: TrackerGrid,
}

impl State {
    fn new() -> Self {
        State {
            queue: BinaryHeap::new(),
            tracker: TrackerGrid::new(),
        }
    }

    fn clear(&mut self) {
        self.queue.clear();
        self.tracker.clear();
    }
}

pub struct WeightedGridSearchContext {
    state: RefCell<State>,
}

impl WeightedGridSearchContext {
    pub fn new() -> Self {
        WeightedGridSearchContext { state: RefCell::new(State::new()) }
    }

    fn dijkstra_predicate_search<T: Traverse, G: Grid<Item = T>>(&self,
                                                                 grid: &G,
                                                                 start: Coord,
                                                                 predicate: &Box<Fn(CellInfo<T>)
                                                                                    -> bool>,
                                                                 dirs: &[Direction])
                                                                 -> Option<Path> {
        let mut state = self.state.borrow_mut();
        state.clear();

        state.queue.push(Node::new_dijkstra(start, 0.0));
        state.tracker.see_with_cost(start, 0.0);

        while let Some(node) = state.queue.pop() {

            if state.tracker.is_visited(node.coord) {
                continue;
            }

            state.tracker.visit(node.coord);

            let info = CellInfo::new(grid.get_unsafe(node.coord), node.coord);
            if predicate(info) {
                return Some(state.tracker.make_path(node.coord).unwrap());
            }

            for dir in dirs {
                let nei_coord = node.coord + dir.vector();
                if let Some(cell) = grid.get(nei_coord) {
                    if let Some(cost) = cell.cost() {
                        let total_cost = node.cost + cost * dir.multiplier();
                        if state.tracker.see_with_parent(nei_coord, total_cost, node.coord, *dir) {
                            state.queue.push(Node::new_dijkstra(nei_coord, total_cost));
                        }
                    }
                }
            }
        }

        None
    }

    fn astar_coord_search<T: Traverse, G: Grid<Item = T>>(&self,
                                                          grid: &G,
                                                          start: Coord,
                                                          dest: Coord,
                                                          dirs: &[Direction])
                                                          -> Option<Path> {
        let mut state = self.state.borrow_mut();
        state.clear();

        state.queue.push(Node::new_astar(start, 0.0, 0.0));
        state.tracker.see_with_cost(start, 0.0);

        while let Some(node) = state.queue.pop() {


            if state.tracker.is_visited(node.coord) {
                continue;
            }

            state.tracker.visit(node.coord);

            if node.coord == dest {
                return Some(state.tracker.make_path(node.coord).unwrap());
            }


            for dir in dirs {
                let nei_coord = node.coord + dir.vector();
                if let Some(cell) = grid.get(nei_coord) {

                    if let Some(cost) = cell.cost() {
                        let total_cost = node.cost + cost * dir.multiplier();
                        if state.tracker.see_with_parent(nei_coord, total_cost, node.coord, *dir) {
                            let heuristic = ((nei_coord - dest).len_sq() as f64).sqrt();
                            state.queue.push(Node::new_astar(nei_coord, total_cost, heuristic));
                        }
                    }
                }
            }
        }

        None
    }
}


impl SearchContext for WeightedGridSearchContext {
    fn search<T: Traverse, G: Grid<Item = T>>(&self,
                                              grid: &G,
                                              query: &Query<T>)
                                              -> Result<Path, SearchError> {

        if let Some(initial_cell) = grid.get(query.start) {
            if !initial_cell.is_traversable() {
                return Err(SearchError::NonTraversableStart);
            }
            if query.matches(CellInfo::new(&initial_cell, query.start)) {
                return Err(SearchError::AtDestination);
            }
        } else {
            return Err(SearchError::StartOutOfGrid);
        }

        let result = match &query.end {
            &Destination::Predicate(ref predicate) => {
                self.dijkstra_predicate_search(grid, query.start, predicate, query.directions)
            }
            &Destination::Coord(coord) => {
                self.astar_coord_search(grid, query.start, coord, query.directions)
            }
        };

        if let Some(path) = result {
            Ok(path)
        } else {
            Err(SearchError::Exhausted)
        }
    }
}
