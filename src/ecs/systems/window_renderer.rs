use ecs::entity_table::EntityTable;
use ecs::system_queue::SystemQueue;
use ecs::message::Message;

use terminal::window_manager::WindowRef;
use colour::ansi;

use std::fmt;

pub struct WindowRenderer<'a> {
    window: WindowRef<'a>,
}

impl<'a> WindowRenderer<'a> {
    pub fn new(window: WindowRef<'a>) -> Self {
        WindowRenderer {
            window: window,
        }
    }

    pub fn process_message(&mut self, _: &mut Message, _: &mut EntityTable, _: &SystemQueue) {
        self.window.fill('a', ansi::BLUE, ansi::GREEN);
    }
}

impl<'a> fmt::Debug for WindowRenderer<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WindowRenderer {{}}")
    }
}
