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
macro_rules! warn_disabled {
    ($s:expr) => {
        eprintln!(
            "{}",
            Format::Warning(format!(
                "Feature \"{f}\" was not enabled at compile-time. Skipping {f}",
                f = $s
            ))
        );
    };
}
