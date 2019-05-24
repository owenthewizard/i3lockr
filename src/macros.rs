pub static mut DEBUG: bool = false;

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) || unsafe {DEBUG} {
            eprintln!("{f}:{l}:{c} {fmt}", f=file!(), l=line!(), c=column!(), fmt=format!($($arg)*));
        }
    }
}

#[macro_export]
macro_rules! timer_start {
    ($timer:ident) => {
        let $timer = Instant::now();
    };
}

#[macro_export]
macro_rules! timer_time {
    ($s:expr, $timer:ident) => {
        debug!("{} took {:#?}", $s, $timer.elapsed());
    };
}

#[macro_export]
macro_rules! color_panic {
    ($($arg:tt)*) => {
        panic!("{}", Format::Error(format!($($arg)*)));
    }
}
