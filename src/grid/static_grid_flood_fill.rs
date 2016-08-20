use grid::{
    Grid,
    IterGrid,
    StaticGrid,
    Coord,
};

use geometry::direction;

impl<T> StaticGrid<T> {
    pub fn flood_fill_region_coord_all<'a, P>(&'a self, predicate: P)
                                      -> Vec<Vec<Coord>>
        where P: FnMut(&T) -> bool
    {
        self.flood_fill_region_coord(&direction::DIRECTIONS, predicate)
    }

    pub fn flood_fill_region_coord_cardinal<'a, P>(&'a self, predicate: P)
                                      -> Vec<Vec<Coord>>
        where P: FnMut(&T) -> bool
    {
        self.flood_fill_region_coord(&direction::CARDINAL_DIRECTIONS, predicate)
    }

    pub fn flood_fill_region_coord_ordinal<'a, P>(&'a self, predicate: P)
                                      -> Vec<Vec<Coord>>
        where P: FnMut(&T) -> bool
    {
        self.flood_fill_region_coord(&direction::ORDINAL_DIRECTIONS, predicate)
    }

    pub fn flood_fill_region_coord<P>(&self, directions: &[direction::Direction], mut predicate: P)
                             -> Vec<Vec<Coord>>
        where P: FnMut(&T) -> bool
    {
        let mut regions = Vec::<Vec<Coord>>::new();

        let mut visited = StaticGrid::<bool>::new_copy(self.width as usize, self.height as usize, false);
        let mut to_visit = Vec::<Coord>::new();

        for (coord, data) in izip!(
            self.coord_iter(),
            self.iter())
        {
            if !visited[coord] && predicate(data) {
                regions.push(self.flood_fill_helper(|d| {predicate(d)}, coord, &mut visited,
                                               &mut to_visit, &directions));
            }
        }

        regions
    }

    fn flood_fill_helper<P>(&self, mut predicate: P,
                               start_coord: Coord,
                               visited: &mut StaticGrid<bool>,
                               to_visit: &mut Vec<Coord>,
                               directions: &[direction::Direction]) -> Vec<Coord>
        where P: FnMut(&T) -> bool
    {
        let mut region = Vec::<Coord>::new();

        assert!(to_visit.is_empty());
        assert!(!visited[start_coord]);
        assert!(predicate(&self[start_coord]));
        to_visit.push(start_coord);
        visited[start_coord] = true;

        while !to_visit.is_empty() {
            let current_coord = to_visit.pop().unwrap();
            region.push(current_coord);

            for direction in directions {
                let next_coord = current_coord + direction.vector().convert::<isize>();
                if self.is_valid_coord(next_coord) &&
                    !visited[next_coord] &&
                    predicate(&self[next_coord])
                {
                    visited[next_coord] = true;
                    to_visit.push(next_coord);
                }
            }
        }

        region
    }
}
