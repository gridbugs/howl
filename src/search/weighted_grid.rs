use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::result;

use search::tracker_grid::TrackerGrid;
use search::path::Path;
use grid::{Grid, Coord};
use geometry::{direction, Direction, LengthSquared};

#[derive(Debug)]
pub enum Error {
    StartOutOfGrid,
    NonTraversableStart,
    Exhausted,
}

pub type Result<T> = result::Result<T, Error>;

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

pub trait TraverseCost {
    fn traverse_cost(&self) -> Option<f64>;
    fn is_traversable(&self) -> bool {
        self.traverse_cost().is_some()
    }
}

pub struct WeightedGridSearchContext {
    queue: BinaryHeap<Node>,
    tracker: TrackerGrid,
}

pub struct Config {
    pub directions: &'static [Direction],
}

pub struct CellInfo<'a, T: 'a> {
    pub value: &'a T,
    pub coord: Coord,
}

impl<'a, T: 'a> CellInfo<'a, T> {
    pub fn new(value: &'a T, coord: Coord) -> Self {
        CellInfo {
            value: value,
            coord: coord,
        }
    }
}

impl Config {
    pub fn new_all_directions() -> Self {
        Config { directions: &direction::DIRECTIONS }
    }
}

impl WeightedGridSearchContext {
    pub fn new() -> Self {
        WeightedGridSearchContext {
            queue: BinaryHeap::new(),
            tracker: TrackerGrid::new(),
        }
    }

    fn clear(&mut self) {
        self.queue.clear();
        self.tracker.clear();
    }

    fn check_errors<T, G>(grid: &G, start: Coord) -> Result<()>
        where T: TraverseCost,
              G: Grid<Item = T>
    {
        if let Some(initial_cell) = grid.get(start) {
            if !initial_cell.is_traversable() {
                return Err(Error::NonTraversableStart);
            }
        } else {
            return Err(Error::StartOutOfGrid);
        }

        Ok(())
    }

    pub fn search_predicate<T, G, F>(&mut self,
                                     grid: &G,
                                     start: Coord,
                                     predicate: F,
                                     config: &Config,
                                     path: &mut Path)
                                     -> Result<()>
        where T: TraverseCost,
              G: Grid<Item = T>,
              F: Fn(CellInfo<T>) -> bool
    {
        try!(Self::check_errors(grid, start));

        self.clear();
        self.queue.push(Node::new_dijkstra(start, 0.0));
        self.tracker.see_with_cost(start, 0.0);

        while let Some(node) = self.queue.pop() {

            if self.tracker.is_visited(node.coord) {
                continue;
            }

            self.tracker.visit(node.coord);

            let info = CellInfo::new(grid.get_unsafe(node.coord), node.coord);
            if predicate(info) {
                self.tracker.populate_path(node.coord, path);
                return Ok(());
            }

            for dir in config.directions {
                let nei_coord = node.coord + dir.vector();
                if let Some(cell) = grid.get(nei_coord) {
                    if let Some(cost) = cell.traverse_cost() {
                        let total_cost = node.cost + cost * dir.multiplier();
                        if self.tracker.see_with_parent(nei_coord, total_cost, node.coord, *dir) {
                            self.queue.push(Node::new_dijkstra(nei_coord, total_cost));
                        }
                    }
                }
            }
        }

        Err(Error::Exhausted)
    }

    pub fn search_coord<T, G>(&mut self,
                              grid: &G,
                              start: Coord,
                              destination: Coord,
                              config: &Config,
                              path: &mut Path)
                              -> Result<()>
        where T: TraverseCost,
              G: Grid<Item = T>
    {
        try!(Self::check_errors(grid, start));

        self.clear();
        self.queue.push(Node::new_astar(start, 0.0, 0.0));
        self.tracker.see_with_cost(start, 0.0);

        while let Some(node) = self.queue.pop() {

            if self.tracker.is_visited(node.coord) {
                continue;
            }

            self.tracker.visit(node.coord);

            if node.coord == destination {
                self.tracker.populate_path(node.coord, path);
                return Ok(());
            }

            for dir in config.directions {
                let nei_coord = node.coord + dir.vector();
                if let Some(cell) = grid.get(nei_coord) {

                    if let Some(cost) = cell.traverse_cost() {
                        let total_cost = node.cost + cost * dir.multiplier();
                        if self.tracker.see_with_parent(nei_coord, total_cost, node.coord, *dir) {
                            let heuristic = ((nei_coord - destination).len_sq() as f64).sqrt();
                            self.queue.push(Node::new_astar(nei_coord, total_cost, heuristic));
                        }
                    }
                }
            }
        }

        Err(Error::Exhausted)
    }
}
