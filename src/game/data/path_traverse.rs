use search::GridPath;
use direction::Direction;

#[derive(RustcEncodable, RustcDecodable)]
pub struct PathTraverse {
    path: GridPath,
    index: usize,
}

impl PathTraverse {
    pub fn new() -> Self {
        PathTraverse {
            path: GridPath::new(),
            index: 0,
        }
    }

    pub fn path_mut(&mut self) -> &mut GridPath {
        &mut self.path
    }

    pub fn reset(&mut self) {
        self.index = 0;
    }

    pub fn is_complete(&self) -> bool {
        self.index == self.path.len()
    }

    pub fn next_direction(&mut self) -> Option<Direction> {
        let direction = self.path.get_node(self.index).map(|node| {
            node.direction_to
        });

        if direction.is_some() {
            self.index += 1;
        }

        direction
    }
}
