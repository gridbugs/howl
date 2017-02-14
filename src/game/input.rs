#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum InputEvent {
    Char(char),
    Up,
    Down,
    Left,
    Right,
    Quit,
    Escape,
    Return,
}

pub trait InputSource {
    fn next_input(&mut self) -> Option<InputEvent>;
}
