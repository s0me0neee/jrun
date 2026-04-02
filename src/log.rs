#[macro_export]
macro_rules! error {
    ($fmt:expr, $($arg:tt)+) => {
        format!("{} {}", "[Error]".red(), format!($fmt, $($arg)+))
    };
    ($msg:expr) => {
        format!("{} {}", "[Error]".red(), $msg)
    };
}

#[macro_export]
macro_rules! warning {
    ($fmt:expr, $($arg:tt)+) => {
        format!("{} {}", "[Warning]".yellow(), format!($fmt, $($arg)+))
    };
    ($msg:expr) => {
        format!("{} {}", "[Warning]".yellow(), $msg)
    };
}

#[macro_export]
macro_rules! info {
    ($tag:expr, $fmt:expr, $($arg:tt)+) => {
        format!("{} {}",
            format!("[{}]", $tag).blue(),
            format!($fmt, $($arg)+)
        )
    };
    ($tag:expr, $msg:expr) => {
        format!("{} {}", format!("[{}]", $tag).blue(), $msg)
    };
}
