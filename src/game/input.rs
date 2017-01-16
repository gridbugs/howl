#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum InputEvent {
    Char(char),
    Up,
    Down,
    Left,
    Right,
    Quit,
    Escape,
}

pub trait InputSource {
    fn next_input(&self) -> Option<InputEvent>;
}
