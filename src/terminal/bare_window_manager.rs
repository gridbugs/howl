use std::collections::HashMap;
use std::cell::RefCell;
use allocator::allocator::Allocator;
use colour::ansi::AnsiColour;
use colour::ansi;
use terminal::{Style, style};
use grid::{
    Grid,
    DefaultGrid,
    IterGrid,
    StaticGrid,
};
use geometry::Vector2;
use std::io::Error;
use rustty;

pub type Event = rustty::Event;

#[derive(Clone, Copy)]
struct BufferCell {
    ch: char,
    fg: AnsiColour,
    bg: AnsiColour,
    style: Style,
}

impl Default for BufferCell {
    fn default() -> Self {
        BufferCell {
            ch: ' ',
            fg: ansi::WHITE,
            bg: ansi::BLACK,
            style: style::NONE,
        }
    }
}

pub struct Window {
    pub coord: (isize, isize),
    pub size: (usize, usize),
    buffer: RefCell<StaticGrid<BufferCell>>,
}

impl Window {
    fn buffer_ch(&self, x: isize, y: isize, ch: char) {
        self.buffer.borrow_mut().get_mut(Vector2::new(x, y)).map(|buffer_cell| {
            buffer_cell.ch = ch;
        });
    }
    fn buffer_fg(&self, x: isize, y: isize, fg: AnsiColour) {
        self.buffer.borrow_mut().get_mut(Vector2::new(x, y)).map(|buffer_cell| {
            buffer_cell.fg = fg;
        });
    }
    fn buffer_bg(&self, x: isize, y: isize, bg: AnsiColour) {
        self.buffer.borrow_mut().get_mut(Vector2::new(x, y)).map(|buffer_cell| {
            buffer_cell.bg = bg;
        });
    }
    fn buffer_style(&self, x: isize, y: isize, style: Style) {
        self.buffer.borrow_mut().get_mut(Vector2::new(x, y)).map(|buffer_cell| {
            buffer_cell.style = style;
        });
    }
}

fn rustty_style(style: Style) -> rustty::Attr {
    match style {
        style::BOLD => rustty::Attr::Bold,
        style::NONE => rustty::Attr::Default,
        _ => unimplemented!(),
    }
}

pub struct BareWindowManager {
    terminal: RefCell<rustty::Terminal>,
    allocator: Allocator<Window>,
    order: Vec<u64>,
    spatial_hash: HashMap<(isize, isize), u64>,
}

impl BareWindowManager {

    pub fn new() -> Result<BareWindowManager, Error> {
        rustty::Terminal::new().map(|t| {
            BareWindowManager {
                terminal: RefCell::new(t),
                allocator: Allocator::new(),
                order: Vec::new(),
                spatial_hash: HashMap::new(),
            }
        })
    }

    pub fn get_width(&self) -> usize { self.terminal.borrow().cols() }
    pub fn get_height(&self) -> usize { self.terminal.borrow().rows() }

    pub fn make_window(&mut self, x: isize, y: isize,
                       width: usize, height: usize) -> u64
    {
        let window = Window {
            coord: (x, y),
            size: (width, height),
            buffer: RefCell::new(StaticGrid::new_default(width, height)),
        };

        let id = self.allocator.allocate(window);

        self.order.push(id);

        self.spatial_hash_add_window(id);

        id
    }

    fn spatial_hash_update(&mut self) {
        self.spatial_hash.clear();

        let order_copy = self.order.to_vec();

        for id in &order_copy {
            self.spatial_hash_add_window(*id);
        }
    }

    fn spatial_hash_add_window(&mut self, id: u64) {
        let ((x, y), (width, height)) = {
            let window = self.get_window(id);
            (window.coord, window.size)
        };
        for i in 0..(height as isize) {
            for j in 0..(width as isize) {
                let coord = (x + j, y + i);
                self.spatial_hash.insert(coord, id);
            }
        }

    }

    fn order_index_of(&mut self, id: u64) -> usize {
        let mut index = 0;
        for i in &self.order {
            if *i == id {
                return index;
            }
            index += 1;
        }
        panic!("no such window id {}", id);
    }

    pub fn window_delete(&mut self, id: u64) {
        let index = self.order_index_of(id);
        self.order.remove(index);
        self.spatial_hash_update();
        self.redraw_window_buffers();
    }

    pub fn window_bring_to_front(&mut self, id: u64) {
        let index = self.order_index_of(id);
        self.order.remove(index);
        self.order.push(id);
        self.spatial_hash_update();
        self.redraw_window_buffers();
    }

    pub fn window_send_to_back(&mut self, id: u64) {
        let index = self.order_index_of(id);
        self.order.remove(index);
        self.order.insert(0, id);
        self.spatial_hash_update();
        self.redraw_window_buffers();
    }

    fn redraw_window_buffer(&self, id: u64) {
        let window = self.get_window(id);
        let buffer = window.buffer.borrow();

        for (buffer_cell, coord) in izip!(
            buffer.iter(),
            buffer.coord_iter())
        {
            self.with_cell(window.coord.0 + coord.x, window.coord.1 + coord.y, |cell| {
                cell.set_ch(buffer_cell.ch);
                cell.set_fg(rustty::Color::Byte(buffer_cell.fg.code()));
                cell.set_bg(rustty::Color::Byte(buffer_cell.bg.code()));
                cell.set_attrs(rustty_style(buffer_cell.style));
            });
        }
    }

    fn redraw_window_buffers(&self) {
        self.terminal.borrow_mut().clear().unwrap();

        for id in &self.order {
            self.redraw_window_buffer(*id);
        }
    }

    pub fn flush(&mut self) {
        self.terminal.borrow_mut().swap_buffers().unwrap();
    }

    fn get_top_window_id(&self, x: isize, y: isize) -> Option<u64> {
        self.spatial_hash.get(&(x, y)).map(|id| {*id})
    }

    fn is_top_window(&self, x: isize, y: isize, id: u64) -> bool {
        if let Some(top) = self.get_top_window_id(x, y) {
            top == id
        } else {
            false
        }
    }

    pub fn set_window_ch(&self, id: u64, x: isize, y: isize, ch: char) {
        let (coord, size) = {
            let window = self.get_window(id);
            window.buffer_ch(x, y, ch);
            (window.coord, window.size)
        };

        let global_coord = (x + coord.0, y + coord.1);

        if self.is_top_window(global_coord.0, global_coord.1, id) &&
            x >= 0 && y >= 0 && (x as usize) < size.0 && (y as usize) < size.1 {
                self.with_cell(global_coord.0, global_coord.1, |cell| {
                    cell.set_ch(ch);
                });
        }
    }

    pub fn set_window_fg(&self, id: u64, x: isize, y: isize, fg: AnsiColour) {
        let (coord, size) = {
            let window = self.get_window(id);
            window.buffer_fg(x, y, fg);
            (window.coord, window.size)
        };

        let global_coord = (x + coord.0, y + coord.1);

        if self.is_top_window(global_coord.0, global_coord.1, id) &&
            x >= 0 && y >= 0 && (x as usize) < size.0 && (y as usize) < size.1 {
                self.with_cell(x + coord.0, y + coord.1, |cell| {
                    cell.set_fg(rustty::Color::Byte(fg.code()));
                });
            }
    }

    pub fn set_window_bg(&self, id: u64, x: isize, y: isize, bg: AnsiColour) {
        let (coord, size) = {
            let window = self.get_window(id);
            window.buffer_bg(x, y, bg);
            (window.coord, window.size)
        };

        let global_coord = (x + coord.0, y + coord.1);

        if self.is_top_window(global_coord.0, global_coord.1, id) &&
            x >= 0 && y >= 0 && (x as usize) < size.0 && (y as usize) < size.1 {
                self.with_cell(x + coord.0, y + coord.1, |cell| {
                    cell.set_bg(rustty::Color::Byte(bg.code()));
                });
            }
    }

    pub fn set_window_style(&self, id: u64, x: isize, y: isize, style: Style) {
        let (coord, size) = {
            let window = self.get_window(id);
            window.buffer_style(x, y, style);
            (window.coord, window.size)
        };

        let global_coord = (x + coord.0, y + coord.1);

        if self.is_top_window(global_coord.0, global_coord.1, id) &&
            x >= 0 && y >= 0 && (x as usize) < size.0 && (y as usize) < size.1 {
                self.with_cell(x + coord.0, y + coord.1, |cell| {
                    cell.set_attrs(rustty_style(style));
                });
            }
    }



    pub fn fill_window(&mut self, id: u64, ch: char, fg: AnsiColour, bg: AnsiColour) {
        let (width, height) = self.get_window(id).size;
        for i in 0..(height as isize) {
            for j in 0..(width as isize) {
                self.set_window_ch(id, j, i, ch);
                self.set_window_fg(id, j, i, fg);
                self.set_window_bg(id, j, i, bg);
            }
        }
    }

    pub fn get_event(&self) -> Option<Event> {
        self.terminal.borrow_mut().get_event(None).unwrap()
    }

    pub fn get_window(&self, id: u64) -> &Window {
        self.allocator.get(id).unwrap()
    }

    fn get_mut_window(&mut self, id: u64) -> &mut Window {
        self.allocator.get_mut(id).unwrap()
    }

    fn with_cell<F>(&self, x: isize, y: isize, f: F)
        where F: Fn(&mut rustty::Cell) {
        let mut term = self.terminal.borrow_mut();
        let cell = &mut term[(x as usize, y as usize)];
        f(cell);
    }
}
