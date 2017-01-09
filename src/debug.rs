use std::io;
use std::str;

static mut TARGET: Option<*mut io::Write> = None;

pub fn init(w: Box<io::Write>) {
    unsafe {
        TARGET = Some(Box::into_raw(w));
    }
}

pub struct NullDebug;

impl io::Write for NullDebug {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub struct PrintDebug;

impl io::Write for PrintDebug {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        print!("{}", str::from_utf8(buf).unwrap());
        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub fn write(buf: &[u8]) -> io::Result<usize> {
    unsafe {
        TARGET.map_or_else(|| Err(io::Error::new(io::ErrorKind::NotFound, "Debug not initialised")),
                           |w| (&mut *w).write(buf))
    }
}

macro_rules! debug_print {
    ($($arg:tt)*) => {
        debug::write(format!($($arg)*).as_bytes()).unwrap()
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
