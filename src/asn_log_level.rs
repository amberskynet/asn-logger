use log::LevelFilter;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[repr(usize)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Serialize, Deserialize)]
pub enum AsnLogLevel {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl Display for AsnLogLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AsnLogLevel::Off => write!(f, "OFF"),
            AsnLogLevel::Error => write!(f, "ERROR"),
            AsnLogLevel::Warn => write!(f, "WARN"),
            AsnLogLevel::Info => write!(f, "INFO"),
            AsnLogLevel::Debug => write!(f, "DEBUG"),
            AsnLogLevel::Trace => write!(f, "TRACE"),
        }
    }
}

impl AsnLogLevel {
    pub fn from_string(s: String) -> Result<AsnLogLevel, String> {
        match s.to_lowercase().as_str() {
            "off" => Ok(AsnLogLevel::Off),
            "error" => Ok(AsnLogLevel::Error),
            "warn" => Ok(AsnLogLevel::Warn),
            "info" => Ok(AsnLogLevel::Info),
            "debug" => Ok(AsnLogLevel::Debug),
            "trace" => Ok(AsnLogLevel::Trace),
            _ => Err(String::from(&format!("Unknown log level: {}", s))),
        }
    }
}

impl From<AsnLogLevel> for LevelFilter {
    fn from(level: AsnLogLevel) -> LevelFilter {
        match level {
            AsnLogLevel::Off => LevelFilter::Off,
            AsnLogLevel::Error => LevelFilter::Error,
            AsnLogLevel::Warn => LevelFilter::Warn,
            AsnLogLevel::Info => LevelFilter::Info,
            AsnLogLevel::Debug => LevelFilter::Debug,
            AsnLogLevel::Trace => LevelFilter::Trace,
        }
    }
}
