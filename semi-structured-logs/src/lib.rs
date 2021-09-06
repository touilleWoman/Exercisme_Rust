/// various log levels
#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    match level {
        LogLevel::Info => info(message),
        LogLevel::Warning => warn(message),
        LogLevel::Error => error(message),
    }
}
pub fn info(message: &str) -> String {
    let mut info = String::from("[INFO]: ");
    info.push_str(message);
    info
}
pub fn warn(message: &str) -> String {
    let mut warn = String::from("[WARNING]: ");
    warn.push_str(message);
    warn
}
pub fn error(message: &str) -> String {
    let mut error = String::from("[ERROR]: ");
    error.push_str(message);
    error
}
