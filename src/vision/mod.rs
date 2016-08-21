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

mod recursive_shadowcast;
pub use self::recursive_shadowcast::RecursiveShadowcast;

#[cfg(test)]
mod tests;
