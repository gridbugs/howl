const MS_PER_S: f64 = 1000.0;

#[derive(Clone, Copy, Debug)]
pub struct Speed {
    cells_per_ms: f64,
}

impl Speed {
    pub fn from_cells_per_sec(cells_per_sec: f64) -> Self {
        Speed {
            cells_per_ms: cells_per_sec / MS_PER_S,
        }
    }

    pub fn from_cells_per_ms(cells_per_ms: f64) -> Self {
        Speed {
            cells_per_ms: cells_per_ms,
        }
    }

    pub fn ms_per_cell(self) -> u64 {
        (1.0 / self.cells_per_ms) as u64
    }
}
