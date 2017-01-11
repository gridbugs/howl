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

#[derive(Clone, Copy)]
pub struct InputSourceRef(*const InputSource);

impl InputSourceRef {
    pub fn new(input_source: *const InputSource) -> Self {
        InputSourceRef(input_source)
    }

    pub fn next_input(&self) -> Option<InputEvent> {
        unsafe { &(*self.0) }.next_input()
    }
}
