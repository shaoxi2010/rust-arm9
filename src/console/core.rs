///! Printing.
use core::fmt;

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------

#[doc(hidden)]
pub(crate) fn _print(args: fmt::Arguments) {
    use fmt::Write;

    crate::console::pl011::console().write_fmt(args).unwrap();
}

/// Prints without a newline.
///
/// Carbon copy from <https://doc.rust-lang.org/src/std/macros.rs.html>
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::console::core::_print(format_args!($($arg)*)));
}

/// Prints with a newline.
///
/// Carbon copy from <https://doc.rust-lang.org/src/std/macros.rs.html>
#[macro_export]

macro_rules! println {
    () => ($crate::print!("\r\n"));
    ($($arg:tt)*) => ({
        $crate::console::core::_print(format_args!($($arg)*));
        println!();
    })
}