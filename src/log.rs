// Publicly expose the termion crate from this crate so that
// we can use it in the macros
pub extern crate termion;

/// # Base Logging Macro
/// Prints the colored logging info to stdout. Not meant to be used outside of
/// this crate.
///
/// ## Args
///     - $x1: Termion color to color the level indicator
///         (eg. crate::log::termion::color::Fg(crate::log::termion::color::Green))
///     - $x2: String that defines the logging level. Including any bracing
///         (eg. "[INFO]")
///     - $x3: String to print after the brace
///     - $opt: Any formatting arguments that would be passed to println!
macro_rules! log {
    ($x1:expr, $x2:expr, $x3:expr $(, $opt:expr)*) => {
        {
            // Print the logging level
            print!(
                "{}{}{}",
                $x2,
                $x1,
                crate::log::termion::color::Fg(crate::log::termion::color::Reset)
            );
            // Print the logging info
            println!($x3, $($opt),*);

        }
    };
}

/// # Info Logging Macro
/// Log info to the screen. The info is preceeded by a green [INFO]
/// Can be used just like the println! macro.
macro_rules! info
{
    ($($x1:expr), +) => {
        log!("[INFO] ", crate::log::termion::color::Fg(crate::log::termion::color::Green), $($x1),*);
    }
}

//macro_rules! warn
//{
//($($x1:expr), +) => {
//log!("[WARN] ", crate::log::termion::color::Fg(crate::log::termion::color::Yellow), $($x1),*);
//}
//}

/// # Error Logging Macro
/// Log errors to the screen. The error is preceeded by a red [ERROR].
/// Can be used just like the println! macro.
macro_rules! error
{
    ($($x1:expr), +) => {
        log!("[ERROR] ", crate::log::termion::color::Fg(crate::log::termion::color::Red), $($x1),*);
    }
}

//macro_rules! debug
//{
//($($x1:expr), +) => {
//log!("[DEBUG] ", crate::log::termion::color::Fg(crate::log::termion::color::Blue), $($x1),*);
//}
//}
