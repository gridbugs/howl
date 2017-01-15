use std::cell::UnsafeCell;
use std::io::Error;
use std::collections::HashMap;

use rustty;

use coord::Coord;
use grid::{StaticGrid, Grid, CopyGrid, IterGrid, CoordIterGrid};
use util::LeakyReserver;

use frontends::ansi::{Style, styles, AnsiColour, colours, WindowBuffer};

#[derive(PartialEq, Eq)]
pub enum BufferType {
    Single,
    Double,
}

#[derive(Copy, Clone)]
pub struct WindowCell {
    ch: char,
    fg: AnsiColour,
    bg: AnsiColour,
    style: Style,
}

impl WindowCell {
    fn new(ch: char, fg: AnsiColour, bg: AnsiColour, style: Style) -> Self {
        WindowCell {
            ch: ch,
            fg: fg,
            bg: bg,
            style: style,
        }
    }
}

impl Default for WindowCell {
    fn default() -> Self {
        WindowCell::new(' ', colours::WHITE, colours::BLACK, styles::NONE)
    }
}

struct WindowData {
    coord: (isize, isize),
    grid: StaticGrid<WindowCell>,
}

impl WindowData {
    fn new(x: isize, y: isize, width: usize, height: usize) -> Self {
        WindowData {
            coord: (x, y),
            grid: StaticGrid::new_copy(width, height, WindowCell::default()),
        }
    }

    pub fn get_cell(&mut self, x: isize, y: isize) -> &mut WindowCell {
        self.grid.get_checked_mut(Coord::new(x, y))
    }

    pub fn fill(&mut self, ch: char, fg: AnsiColour, bg: AnsiColour, style: Style) {
        self.grid.set_all(WindowCell::new(ch, fg, bg, style))
    }

    pub fn coord(&self) -> (isize, isize) {
        self.coord
    }

    pub fn size(&self) -> (usize, usize) {
        (self.width(), self.height())
    }

    pub fn width(&self) -> usize {
        self.grid.width()
    }

    pub fn height(&self) -> usize {
        self.grid.height()
    }
}

pub struct Window {
    id: u64,
    manager: *mut WindowManager,
    data: WindowData,
    buffer_type: BufferType,
}

impl Window {
    fn new(manager: *mut WindowManager,
           x: isize,
           y: isize,
           width: usize,
           height: usize,
           buffer_type: BufferType)
           -> Self {
        Window {
            id: unsafe { &mut (*manager) }.allocate_window(x, y, width, height),
            manager: manager,
            data: WindowData::new(x, y, width, height),
            buffer_type: buffer_type,
        }
    }

    pub fn get_cell(&mut self, x: isize, y: isize) -> &mut WindowCell {
        self.data.get_cell(x, y)
    }

    pub fn fill(&mut self, ch: char, fg: AnsiColour, bg: AnsiColour, style: Style) {
        self.data.fill(ch, fg, bg, style)
    }

    pub fn flush(&mut self) {
        unsafe {
            let mut manager = &mut (*self.manager);
            {
                let mut shadow = manager.windows.get_mut(&self.id).unwrap();
                if self.buffer_type == BufferType::Double {
                    self.data.grid.swap(&mut shadow.grid);
                } else {
                    shadow.grid.copy_from(&self.data.grid);
                }
            }
            manager.flush_window(self.id);
        }
    }

    pub fn coord(&self) -> (isize, isize) {
        self.data.coord()
    }

    pub fn size(&self) -> (usize, usize) {
        self.data.size()
    }

    pub fn width(&self) -> usize {
        self.data.width()
    }

    pub fn height(&self) -> usize {
        self.data.height()
    }
}

impl WindowCell {
    pub fn set_ch(&mut self, ch: char) {
        self.ch = ch;
    }

    pub fn set(&mut self, ch: char, fg: AnsiColour, bg: AnsiColour, style: Style) {
        self.ch = ch;
        self.fg = fg;
        self.bg = bg;
        self.style = style;
    }
}

#[derive(Clone, Copy)]
pub struct AnsiInputSource {
    terminal: *mut rustty::Terminal,
}

impl AnsiInputSource {
    pub fn get_event(&self) -> Option<rustty::Event> {
        unsafe { (*self.terminal).get_event(None).unwrap() }
    }
}

pub struct WindowAllocator {
    manager: UnsafeCell<WindowManager>,
}

impl WindowAllocator {
    pub fn new() -> Result<Self, Error> {
        WindowManager::new().map(|manager| WindowAllocator { manager: UnsafeCell::new(manager) })
    }

    pub fn make_window(&self,
                       x: isize,
                       y: isize,
                       width: usize,
                       height: usize,
                       buffer_type: BufferType)
                       -> Window {
        Window::new(self.manager.get(), x, y, width, height, buffer_type)
    }

    pub fn make_window_buffer(&self,
                              x: isize,
                              y: isize,
                              width: usize,
                              height: usize,
                              border_x: usize,
                              border_y: usize)
                              -> WindowBuffer {
        WindowBuffer::new(self.make_window(x, y, width, height, BufferType::Single),
                          border_x,
                          border_y)
    }

    pub fn make_input_source(&self) -> AnsiInputSource {
        let terminal = unsafe {
            &mut (*self.manager.get()).terminal
        };
        AnsiInputSource { terminal: terminal }
    }

    pub fn width(&self) -> usize {
        unsafe {
            (*self.manager.get()).width()
        }
    }

    pub fn height(&self) -> usize {
        unsafe {
            (*self.manager.get()).height()
        }
    }
}

fn rustty_style(style: Style) -> rustty::Attr {
    match style {
        styles::BOLD => rustty::Attr::Bold,
        styles::NONE => rustty::Attr::Default,
        _ => unimplemented!(),
    }
}

struct WindowManager {
    id_reserver: LeakyReserver<u64>,
    terminal: rustty::Terminal,
    windows: HashMap<u64, WindowData>,
    order: Vec<u64>,
    top_window_map: StaticGrid<Option<u64>>,
}

impl WindowManager {
    pub fn new() -> Result<Self, Error> {
        rustty::Terminal::new().map(|t| {
            WindowManager {
                top_window_map: StaticGrid::new_copy(t.cols(), t.rows(), None),
                id_reserver: LeakyReserver::new(),
                terminal: t,
                windows: HashMap::new(),
                order: Vec::new(),
            }
        })
    }

    fn allocate_window(&mut self, x: isize, y: isize, width: usize, height: usize) -> u64 {
        let id = self.id_reserver.reserve();

        // make back buffer for window
        self.windows.insert(id, WindowData::new(x, y, width, height));

        // add to order
        self.order.push(id);

        // new windows go on top of old windows
        for j in 0..(height as isize) {
            for i in 0..(width as isize) {
                *self.top_window_map.get_checked_mut(Coord::new(i + x, j + y)) = Some(id);
            }
        }

        id
    }

    fn is_top_window(&self, id: u64) -> bool {
        *self.order.last().unwrap() == id
    }

    pub fn width(&self) -> usize {
        self.terminal.cols()
    }

    pub fn height(&self) -> usize {
        self.terminal.rows()
    }

    fn flush_top_window(&mut self, id: u64) {
        let window = self.windows.get(&id).unwrap();
        for (coord, cell) in izip![window.grid.coord_iter(), window.grid.iter()] {
            let x = coord.x + window.coord.0;
            let y = coord.y + window.coord.1;
            let termcell = &mut self.terminal[(x as usize, y as usize)];
            termcell.set_ch(cell.ch);
            termcell.set_fg(rustty::Color::Byte(u8::from(cell.fg)));
            termcell.set_bg(rustty::Color::Byte(u8::from(cell.bg)));
            termcell.set_attrs(rustty_style(cell.style));
        }
    }

    fn flush_non_top_window(&mut self, id: u64) {
        let window = self.windows.get(&id).unwrap();
        for (coord, cell) in izip![window.grid.coord_iter(), window.grid.iter()] {
            let x = coord.x + window.coord.0;
            let y = coord.y + window.coord.1;
            if *self.top_window_map.get_checked_mut(Coord::new(x, y)) == Some(id) {
                let termcell = &mut self.terminal[(x as usize, y as usize)];
                termcell.set_ch(cell.ch);
                termcell.set_fg(rustty::Color::Byte(u8::from(cell.fg)));
                termcell.set_bg(rustty::Color::Byte(u8::from(cell.bg)));
                termcell.set_attrs(rustty_style(cell.style));
            }
        }
    }

    fn flush_window(&mut self, id: u64) {
        if self.is_top_window(id) {
            self.flush_top_window(id);
        } else {
            self.flush_non_top_window(id);
        }
        self.flush();
    }

    fn flush(&mut self) {
        self.terminal.swap_buffers().unwrap();
    }
}
