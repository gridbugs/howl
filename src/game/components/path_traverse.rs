use search::Path;
use geometry::Direction;
use grid::Coord;

#[derive(Clone)]
pub struct PathTraverse {
    path: Path,
    index: usize,
}

impl PathTraverse {
    pub fn new() -> Self {
        PathTraverse {
            path: Path::new(),
            index: 0,
        }
    }

    pub fn path_mut(&mut self) -> &mut Path {
        &mut self.path
    }

    pub fn reset(&mut self) {
        self.index = 0;
    }

    pub fn is_complete(&self) -> bool {
        self.index == self.path.nodes.len()
    }

    pub fn expected_coorg(&self) -> Option<Coord> {
        if self.is_complete() {
            None
        } else {
            Some(self.path.nodes[self.index].source())
        }
    }

    pub fn next_direction(&mut self) -> Option<Direction> {
        if self.is_complete() {
            None
        } else {
            let direction = self.path.nodes[self.index].in_direction;
            self.index += 1;
            Some(direction)
        }
    }
}
