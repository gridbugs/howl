mod vision;
pub use self::vision::{
    Opacity,
    VisibilityReport,
    VisionSystem,
};

mod default;
pub use self::default::DefaultVisibilityReport;

mod square;
pub use self::square::Square;
