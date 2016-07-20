use std::io;
use std::mem;

static mut TARGET: Option<*mut io::Write> = None;

pub fn init(w: &mut io::Write) {
    let ptr = w as *mut io::Write;
    unsafe {
        TARGET = mem::transmute(Some(ptr));
    }
}

pub fn write(buf: &[u8]) -> io::Result<usize> {
    unsafe {
        TARGET.map_or_else(|| {
            Err(io::Error::new(io::ErrorKind::NotFound,
                               "Debug not initialised"))
        }, |w| {
            (&mut *w).write(buf)
        })
    }
}

macro_rules! debug_print {
    ($($arg:tt)*) => {
        debug::debug::write(format!($($arg)*).as_bytes()).unwrap()
    };
}

macro_rules! debug_println {
    ($fmt:expr) => {
        debug_print!(concat!($fmt, "\n"))
    };
    ($fmt:expr, $($arg:tt)*) => {
        debug_print!(concat!($fmt, "\n"), $($arg)*)
    };
}
