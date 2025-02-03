use core::fmt::Write;

use dioxide_hal::platform;

pub fn _print(args: core::fmt::Arguments) {
  // platform::console::console().write_fmt(args).unwrap();
  log::info!("{}", args);
}

/// Prints without a newline.
///
/// Carbon copy from <https://doc.rust-lang.org/src/std/macros.rs.html>
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::std::print::_print(format_args!($($arg)*)));
}

/// Prints with a newline.
///
/// Carbon copy from <https://doc.rust-lang.org/src/std/macros.rs.html>
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ({
        $crate::std::print::_print(format_args_nl!($($arg)*));
    })
}

// temporary identical impl to _print
pub fn _eprint(args: core::fmt::Arguments) {
  // platform::console::console().write_fmt(args).unwrap();
  log::error!("{}", args);
}

/// Prints without a newline.
///
/// Carbon copy from <https://doc.rust-lang.org/src/std/macros.rs.html>
#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => ($crate::std::print::_eprint(format_args!($($arg)*)));
}

/// Prints with a newline.
///
/// Carbon copy from <https://doc.rust-lang.org/src/std/macros.rs.html>
#[macro_export]
macro_rules! eprintln {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ({
        $crate::std::print::_eprint(format_args_nl!($($arg)*));
    })
}
