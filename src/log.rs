pub extern crate termion;

macro_rules! log {
    ($x1:expr, $x2:expr, $x3:expr $(, $opt:expr)*) => {
        {
            print!(
                "{}{}{}",
                $x2,
                $x1,
                crate::log::termion::color::Fg(crate::log::termion::color::Reset)
            );
            println!($x3, $($opt),*);

        }
    };
}

macro_rules! info
{
    ($($x1:expr), +) => {
        log!("[INFO] ", crate::log::termion::color::Fg(crate::log::termion::color::Green), $($x1),*);
    }
}

macro_rules! warn
{
    ($($x1:expr), +) => {
        log!("[WARN] ", crate::log::termion::color::Fg(crate::log::termion::color::Yellow), $($x1),*);
    }
}

macro_rules! error
{
    ($($x1:expr), +) => {
        log!("[ERROR] ", crate::log::termion::color::Fg(crate::log::termion::color::Green), $($x1),*);
    }
}
