pub struct LoggableError {
    pub label: String,
    pub message: String,
}

impl LoggableError {
    pub fn new<S: Into<String>, T: Into<String>>(label: S, message: T) -> Self {
        Self {
            label: label.into(),
            message: message.into(),
        }
    }
}

pub mod config;
pub mod peers;
