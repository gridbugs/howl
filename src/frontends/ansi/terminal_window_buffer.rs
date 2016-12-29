use std::io;

use frontends::ansi::{colours, styles, Window};

pub struct WindowBuffer {
    window: Window,
    lines: Vec<String>,
    current_line: usize,
    cursor_pos: (isize, isize),
    border_x: usize,
    border_y: usize,
    line_width: usize,
    num_lines: usize,
    width: usize,
    height: usize,
}

impl WindowBuffer {
    pub fn new(mut window: Window, border_x: usize, border_y: usize) -> Self {
        let (width, height) = window.size();

        window.fill(' ', colours::WHITE, colours::BLACK, styles::NONE);

        WindowBuffer {
            window: window,
            lines: {
                let mut vec = Vec::with_capacity(height - border_y * 2);
                for _ in 0..(height - border_y * 2) {
                    vec.push(String::new());
                }
                vec
            },
            current_line: 0,
            cursor_pos: (0, 0),
            border_x: border_x,
            border_y: border_y,
            line_width: width - border_x * 2,
            num_lines: height - border_y * 2,
            width: width,
            height: height,
        }
    }

    pub fn draw_borders(&mut self) {
        self.window.get_cell(0, 0).set_ch('+');
        self.window.get_cell(self.width as isize - 1, 0).set_ch('+');
        self.window.get_cell(0, self.height as isize - 1).set_ch('+');
        self.window.get_cell(self.width as isize - 1, self.height as isize - 1).set_ch('+');
        for i in 1..self.width - 1 {
            self.window.get_cell(i as isize, 0).set_ch('-');
            self.window.get_cell(i as isize, self.height as isize - 1).set_ch('-');
        }
        for i in 1..self.height - 1 {
            self.window.get_cell(0, i as isize).set_ch('|');
            self.window.get_cell(self.width as isize - 1, i as isize).set_ch('|');
        }

        self.window.flush();
    }

    fn clear(&mut self) {
        for i in self.border_y..(self.height - self.border_y) {
            for j in self.border_x..(self.width - self.border_x) {
                self.window.get_cell(j as isize, i as isize).set_ch(' ');
            }
        }
    }

    fn scroll(&mut self) {
        self.cursor_pos.1 -= 1;
        self.clear();

        for i in 0..(self.num_lines - 1) {
            let line_idx = (self.current_line + 1 + i) % self.num_lines;
            let mut x = self.border_x;
            let y = self.border_y + i;
            for ch in self.lines[line_idx].chars() {
                let cell = self.window.get_cell(x as isize, y as isize);
                cell.set_ch(ch);
                x += 1;
            }
        }
    }

    fn get_current_string(&mut self) -> &mut String {
        &mut self.lines[self.current_line]
    }

    fn newline(&mut self) {
        self.cursor_pos.0 = 0;
        self.cursor_pos.1 += 1;
        self.current_line = (self.current_line + 1) % self.num_lines;
        self.get_current_string().clear();
    }
}

impl io::Write for WindowBuffer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut count = 0;
        for ch in buf {
            if *ch as char == '\n' {
                self.newline();
            } else {
                self.window
                    .get_cell(self.cursor_pos.0 + self.border_x as isize,
                              self.cursor_pos.1 + self.border_y as isize)
                    .set_ch(*ch as char);
                self.get_current_string().push(*ch as char);

                self.cursor_pos.0 += 1;
                if self.cursor_pos.0 == self.line_width as isize {
                    self.newline();
                }
            }

            if self.cursor_pos.1 == self.num_lines as isize {
                self.scroll();
            }
            count += 1;
        }
        self.window.flush();
        Ok(count)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.window.flush();
        Ok(())
    }
}
