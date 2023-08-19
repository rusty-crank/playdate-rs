use core::fmt::{self, Write};

use alloc::string::String;

use crate::PLAYDATE;

#[doc(hidden)]
#[cold]
pub fn _println(args: fmt::Arguments<'_>) {
    let logger = unsafe { &mut LOGGER };
    logger.write_fmt(args).unwrap();
    logger.flush();
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {{
        $crate::print::_println(format_args!($($arg)*));
    }};
}

static mut LOGGER: Logger = Logger { buf: String::new() };

struct Logger {
    buf: String,
}

impl Logger {
    fn flush(&mut self) {
        PLAYDATE.system.log_to_console(&self.buf);
        self.buf.clear();
    }
}

impl Write for Logger {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        self.buf.push_str(s);
        Ok(())
    }
}
