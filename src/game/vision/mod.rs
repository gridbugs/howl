mod vision;
pub use self::vision::{
    Opacity,
    VisibilityReport,
    VisionSystem,
};

mod default;
pub use self::default::{
    DefaultOpacity,
    DefaultVisibilityReport,
};

mod square;
pub use self::square::square;
