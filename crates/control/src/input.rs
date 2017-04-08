use std::time;

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
    Space,
}

#[derive(Clone, Copy)]
pub enum ExternalEvent {
    Input(InputEvent),
    Frame(Frame),
}

#[derive(Clone, Copy)]
pub struct Frame {
    count: usize,
    time: time::Instant,
}

impl Frame {
    pub fn new(count: usize, time: time::Instant) -> Self {
        Frame {
            count: count,
            time: time,
        }
    }

    pub fn now(count: usize) -> Self { Self::new(count, time::Instant::now()) }

    pub fn count(&self) -> usize { self.count }
    pub fn time(&self) -> time::Instant { self.time }
}

#[derive(Clone, Copy)]
pub struct Period(usize);

impl Period {
    pub fn new(count: usize) -> Self { Period(count) }
    pub  fn decrease(&mut self) -> bool {
        if self.0 == 0 {
            return false;
        } else {
            self.0 -= 1;
            return true;
        }
    }
    pub fn is_empty(&self) -> bool { self.0 == 0 }
    pub fn remaining(&self) -> usize { self.0 }
    pub fn clear(&mut self) {
        self.0 = 0;
    }
}

pub trait InputSource {
    fn next_input(&mut self) -> InputEvent;
    fn next_external(&mut self) -> ExternalEvent;
    fn next_frame_periodic(&mut self, period: &mut Period) -> Option<Frame>;
}
