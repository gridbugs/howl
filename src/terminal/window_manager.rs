use std::cell::RefCell;
use std::io::Error;
use colour::ansi::AnsiColour;
use terminal::Style;
use terminal::bare_window_manager::{Event, BareWindowManager};
use terminal::window_buffer::WindowBuffer;

pub struct WindowCell<'a> {
    window_coord: (isize, isize),
    window: &'a WindowRef<'a>,
}

#[derive(Clone, Copy)]
pub struct WindowRef<'a> {
    id: u64,
    manager: &'a WindowManager,
}

impl<'a> WindowRef<'a> {
    pub fn get_cell(&'a self, x: isize, y: isize) -> WindowCell<'a> {
        WindowCell {
            window_coord: (x, y),
            window: self,
        }
    }

    pub fn fill(&'a self, ch: char, fg: AnsiColour, bg: AnsiColour) {
        self.manager.0.borrow_mut().fill_window(self.id, ch, fg, bg);
    }

    pub fn flush(&'a self) {
        self.manager.flush();
    }

    pub fn delete(&'a self) {
        self.manager.0.borrow_mut().window_delete(self.id);
    }

    pub fn bring_to_front(&'a self) {
        self.manager.0.borrow_mut().window_bring_to_front(self.id);
    }

    pub fn send_to_back(&'a self) {
        self.manager.0.borrow_mut().window_send_to_back(self.id);
    }

    pub fn get_coord(&'a self) -> (isize, isize) {
        self.manager.0.borrow_mut().get_window(self.id).coord
    }

    pub fn get_size(&'a self) -> (usize, usize) {
        self.manager.0.borrow_mut().get_window(self.id).size
    }

    pub fn width(&'a self) -> usize {
        self.manager.0.borrow_mut().get_window(self.id).size.0
    }

    pub fn height(&'a self) -> usize {
        self.manager.0.borrow_mut().get_window(self.id).size.1
    }
}

impl<'a> WindowCell<'a> {
    pub fn set_ch(&'a self, ch: char) {
        self.window.manager.0.borrow()
            .set_window_ch(self.window.id,
                           self.window_coord.0,
                           self.window_coord.1, ch);
    }

    pub fn set_fg(&'a self, fg: AnsiColour) {
        self.window.manager.0.borrow()
            .set_window_fg(self.window.id,
                           self.window_coord.0,
                           self.window_coord.1, fg);
    }

    pub fn set_bg(&'a self, bg: AnsiColour) {
        self.window.manager.0.borrow()
            .set_window_bg(self.window.id,
                           self.window_coord.0,
                           self.window_coord.1, bg);
    }

    pub fn set(&'a self, ch: char, fg: AnsiColour, bg: AnsiColour, style: Style) {
        let w = self.window.manager.0.borrow();
        w.set_window_ch(self.window.id,
                        self.window_coord.0,
                        self.window_coord.1, ch);
        w.set_window_fg(self.window.id,
                        self.window_coord.0,
                        self.window_coord.1, fg);
        w.set_window_bg(self.window.id,
                        self.window_coord.0,
                        self.window_coord.1, bg);
        w.set_window_style(self.window.id,
                           self.window_coord.0,
                           self.window_coord.1, style);
    }
}

#[derive(Clone, Copy)]
pub struct InputSource<'a> {
    manager: &'a WindowManager,
}

impl<'a> InputSource<'a> {
    pub fn get_event(&'a self) -> Option<Event> {
        self.manager.0.borrow_mut().get_event()
    }
}

pub struct WindowManager(RefCell<BareWindowManager>);

impl WindowManager {
    pub fn new() -> Result<WindowManager, Error> {
        BareWindowManager::new().map(|m| {
            WindowManager(RefCell::new(m))
        })
    }

    pub fn make_window(&self, x: isize, y: isize,
                       width: usize, height: usize) -> WindowRef
    {
        WindowRef {
            manager: self,
            id: self.0.borrow_mut().make_window(x, y, width, height),
        }
    }

    pub fn make_window_buffer(&self, x: isize, y: isize,
                              width: usize, height: usize, border_x: usize, border_y: usize) -> WindowBuffer {
        WindowBuffer::new(self.make_window(x, y, width, height), border_x, border_y)
    }

    pub fn make_input_source(&self) -> InputSource {
        InputSource {
            manager: self,
        }
    }

    pub fn flush(&self) { self.0.borrow_mut().flush(); }
    pub fn get_width(&self) -> usize { self.0.borrow().get_width() }
    pub fn get_height(&self) -> usize { self.0.borrow().get_height() }
}
