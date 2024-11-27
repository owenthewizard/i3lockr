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
            format!(
                "Feature \"{f}\" was not enabled at compile-time. Skipping {f}.",
                f = $s
            )
        );
    };
}

#[macro_export]
macro_rules! time_routine {
    ($operand:ident, $F:ident, $Arg:expr, $feat:literal) => {
        if let Some(arg) = $Arg {
        #[cfg(feature = $feat)]
        {
                let timer = Instant::now();

                $operand.$F(arg);

                debug!("`{}.{}({})` took {:#?}", stringify!($operand), stringify!($F), arg, timer.elapsed());
            }

        #[cfg(not(feature = $feat))]
        {
            eprintln!(
                "{}",
                format!(
                    "Feature {} was not enabled at compile-time. Skipping {}.", stringify!($feat), stringify!($F)
                )
            );
        }
        }
    };

    ($operand:ident, $F:ident, $Arg:expr, $feat:literal, $($more:tt)*) => {
        time_routine!($operand, $F, $Arg, $feat);
        time_routine!($operand, $($more)*);
    };
}
