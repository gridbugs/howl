#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Style {
    pub bold: bool,
    pub underline: bool,
    pub reverse: bool,
}

pub mod styles {

    use frontends::ansi::Style;

    pub const NONE: Style = Style {
        bold: false,
        underline: false,
        reverse: false,
    };

    pub const BOLD: Style = Style {
        bold: true,
        underline: false,
        reverse: false,
    };
}
