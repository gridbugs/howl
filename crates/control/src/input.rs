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

pub enum ExternalEvent {
    Input(InputEvent),
    Frame(time::Instant),
}

pub trait InputSource {
    /// Block until a key is pressed, returning the input associated with that key
    fn next_input(&mut self) -> InputEvent;

    /// Block until an external event occurs, returning the event.
    /// This is intended for use when waiting for either a timer tick for animation
    /// or a keypress, allowing for animation while aiting for input.
    fn next_external(&mut self, last_frame: time::Instant) -> ExternalEvent;

    /// Block until the next frame, returning the instant it unblocks, and the
    /// new number of repetitions, or None if called with 0 repetititions.
    /// Intended for use in a loop resembling the following:
    /// while let Some((frame, repeat)) = input.next_frame_repeating(frame, repeat) {
    ///     // draw a frame of animation
    /// }
    ///
    /// This allows an input source to prevent animation by blocking until `repeat`
    /// frames would have passed, then returning None.
    fn next_frame_repeating(&mut self, last_frame: time::Instant, repeat: usize)
        -> Option<(time::Instant, usize)>;
}
