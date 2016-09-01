#[derive(Clone, Copy, Debug)]
pub struct Style {
    pub bold: bool,
    pub underline: bool,
    pub reverse: bool,
}

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
