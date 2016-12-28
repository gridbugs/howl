use std::cmp::Ordering;
use std::collections::{BinaryHeap, VecDeque, vec_deque};
use std::result;
use std::cell::RefCell;

use grid::{Grid, DynamicGrid};
use math::Coord;
use direction::{self, Direction};

#[derive(Debug)]
pub enum Error {
    Exhausted,
    NonTraversableStart,
    StartOutOfGrid,
}

pub type Result<T> = result::Result<T, Error>;

pub trait TraverseCost {
    fn traverse_cost(&self) -> Option<f64>;
    fn is_traversable(&self) -> bool {
        self.traverse_cost().is_some()
    }
}

struct Node {
    coord: Coord,
    cost: f64,
    score: f64,
}

impl Node {
    fn new(coord: Coord, cost: f64, score: f64) -> Self {
        Node {
            coord: coord,
            cost: cost,
            score: score,
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

struct Parent {
    coord: Coord,
    direction: Direction,
}

impl Parent {
    fn new(coord: Coord, direction: Direction) -> Self {
        Parent {
            coord: coord,
            direction: direction,
        }
    }
}

struct SearchCell {
    seen_seq: u64,
    visited_seq: u64,
    cost: f64,
    parent: Option<Parent>,
}

impl SearchCell {
    fn new() -> Self {
        SearchCell {
            seen_seq: 0,
            visited_seq: 0,
            cost: 0.0,
            parent: None,
        }
    }
}

impl Default for SearchCell {
    fn default() -> Self {
        Self::new()
    }
}

struct SearchGrid {
    grid: DynamicGrid<SearchCell>,
    seq: u64,
}

impl SearchGrid {
    fn new() -> Self {
        SearchGrid {
            grid: DynamicGrid::new(),
            seq: 0,
        }
    }

    fn clear(&mut self) {
        self.seq += 1;
    }

    fn see_initial(&mut self, coord: Coord) {
        let cell = self.grid.get_mut_with_default(coord);
        cell.seen_seq = self.seq;
        cell.parent = None;
        cell.cost = 0.0;
    }

    fn is_visited(&self, coord: Coord) -> bool {
        self.grid.get_with_default(coord).visited_seq == self.seq
    }

    fn visit(&mut self, coord: Coord) {
        self.grid.get_mut_with_default(coord).visited_seq = self.seq;
    }

    fn maybe_see(&mut self, coord: Coord, cost: f64, parent: Parent) -> bool {
        let cell = self.grid.get_mut_with_default(coord);

        if cell.visited_seq == self.seq {
            return false;
        }

        if cell.seen_seq == self.seq {
            if cost < cell.cost {
                cell.cost = cost;
                cell.parent = Some(parent);
                return true;
            }
        } else {
            cell.seen_seq = self.seq;
            cell.cost = cost;
            cell.parent = Some(parent);
            return true;
        }

        false
    }

    fn parent(&self, coord: Coord) -> Option<&Parent> {
        let cell = self.grid.get_with_default(coord);
        if cell.visited_seq == self.seq {
            cell.parent.as_ref()
        } else {
            None
        }
    }

    fn populate_path(&self, end: Coord, path: &mut GridPath) {
        path.clear();
        path.cost = self.grid.get_with_default(end).cost;

        let mut coord = end;

        loop {
            let cell = self.grid.get_with_default(coord);
            assert!(cell.visited_seq == self.seq);

            if let Some(ref parent) = cell.parent {
                path.nodes.push_front(GridPathNode {
                    coord: coord,
                    direction_to: parent.direction,
                });

                coord = parent.coord;
            } else {
                path.start = coord;
                return;
            }
        }
    }
}

pub struct GridSearchCfg {
    pub directions: &'static [Direction],
}

impl GridSearchCfg {
    pub fn all_directions() -> Self {
        GridSearchCfg { directions: &direction::DIRECTIONS }
    }
    pub fn cardinal_directions() -> Self {
        GridSearchCfg { directions: &direction::CARDINAL_DIRECTIONS }
    }
}

pub struct GridPathNode {
    pub coord: Coord,
    pub direction_to: Direction,
}

pub struct GridPath {
    start: Coord,
    nodes: VecDeque<GridPathNode>,
    cost: f64,
}

impl GridPath {
    pub fn new() -> Self {
        GridPath {
            start: Coord::new(0, 0),
            cost: 0.0,
            nodes: VecDeque::new(),
        }
    }

    pub fn clear(&mut self) {
        self.start = Coord::new(0, 0);
        self.cost = 0.0;
        self.nodes.clear();
    }

    pub fn start(&self) -> Coord {
        self.start
    }

    pub fn nodes(&self) -> vec_deque::Iter<GridPathNode> {
        self.nodes.iter()
    }

    pub fn get_node(&self, index: usize) -> Option<&GridPathNode> {
        self.nodes.get(index)
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn cost(&self) -> f64 {
        self.cost
    }
}

struct InnerGridSearchCtx {
    queue: BinaryHeap<Node>,
    grid: SearchGrid,
}

pub struct GridCellInfo<'a, T: 'a> {
    pub cell: &'a T,
    pub coord: Coord,
}

impl InnerGridSearchCtx {
    fn new() -> Self {
        InnerGridSearchCtx {
            queue: BinaryHeap::new(),
            grid: SearchGrid::new(),
        }
    }

    fn clear(&mut self) {
        self.queue.clear();
        self.grid.clear();
    }

    fn check_errors<T, G>(&mut self,
                             grid: &G,
                             start: Coord) -> Result<()>
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

    fn search_predicate<T, G, F>(&mut self,
                                 grid: &G,
                                 start: Coord,
                                 predicate: F,
                                 config: &GridSearchCfg,
                                 path: &mut GridPath) -> Result<()>
        where T: TraverseCost,
              G: Grid<Item = T>,
              F: Fn(GridCellInfo<G::Item>) -> bool
    {

        self.check_errors(grid, start)?;

        self.clear();

        self.queue.push(Node::new(start, 0.0, 0.0));
        self.grid.see_initial(start);

        while let Some(node) = self.queue.pop() {

            if self.grid.is_visited(node.coord) {
                continue;
            }

            self.grid.visit(node.coord);

            // Only coordinates valid for the grid being searched
            // are added to the queue, and the grid can't change
            // mid-search.
            let cell = unsafe { grid.get_unchecked(node.coord) };

            let info = GridCellInfo {
                cell: cell,
                coord: node.coord,
            };

            if predicate(info) {
                // found a satisfying cell
                self.grid.populate_path(node.coord, path);
                return Ok(());
            }

            for dir in config.directions {
                let nei_coord = node.coord + dir.vector();
                if let Some(cell) = grid.get(nei_coord) {
                    if let Some(cost) = cell.traverse_cost() {
                        let total_cost = node.cost + cost * dir.multiplier();
                        let parent = Parent::new(node.coord, *dir);
                        if self.grid.maybe_see(nei_coord, total_cost, parent) {
                            self.queue.push(Node::new(nei_coord, total_cost, total_cost));
                        }
                    }
                }
            }
        }

        Err(Error::Exhausted)
    }

    fn search_coord<T, G>(&mut self,
                                 grid: &G,
                                 start: Coord,
                                 destination: Coord,
                                 config: &GridSearchCfg,
                                 path: &mut GridPath) -> Result<()>
        where T: TraverseCost,
              G: Grid<Item = T>
    {

        self.check_errors(grid, start)?;

        self.clear();

        self.queue.push(Node::new(start, 0.0, 0.0));
        self.grid.see_initial(start);

        while let Some(node) = self.queue.pop() {

            if self.grid.is_visited(node.coord) {
                continue;
            }

            self.grid.visit(node.coord);

            if node.coord == destination {
                self.grid.populate_path(node.coord, path);
                return Ok(());
            }

            for dir in config.directions {
                let nei_coord = node.coord + dir.vector();
                if let Some(cell) = grid.get(nei_coord) {
                    if let Some(cost) = cell.traverse_cost() {
                        let total_cost = node.cost + cost * dir.multiplier();
                        let parent = Parent::new(node.coord, *dir);
                        if self.grid.maybe_see(nei_coord, total_cost, parent) {
                            let heuristic = ((nei_coord - destination).length_squared() as f64).sqrt();
                            self.queue.push(Node::new(nei_coord, total_cost, total_cost + heuristic));
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

pub struct GridSearchCtx {
    ctx: RefCell<InnerGridSearchCtx>,
}

impl GridSearchCtx {
    pub fn new() -> Self {
        GridSearchCtx {
            ctx: RefCell::new(InnerGridSearchCtx::new()),
        }
    }

    pub fn search_predicate<T, G, F>(&self,
                                     grid: &G,
                                     start: Coord,
                                     predicate: F,
                                     config: &GridSearchCfg,
                                     path: &mut GridPath) -> Result<()>
        where T: TraverseCost,
              G: Grid<Item = T>,
              F: Fn(GridCellInfo<G::Item>) -> bool
    {
        self.ctx.borrow_mut().search_predicate(grid, start, predicate, config, path)
    }

   pub  fn search_coord<T, G>(&mut self,
                                 grid: &G,
                                 start: Coord,
                                 destination: Coord,
                                 config: &GridSearchCfg,
                                 path: &mut GridPath) -> Result<()>
        where T: TraverseCost,
              G: Grid<Item = T>
    {
        self.ctx.borrow_mut().search_coord(grid, start, destination, config, path)
    }
}
