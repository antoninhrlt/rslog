// This file is part of "juc"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

use std::fmt;

#[derive(Clone, PartialEq, Eq)]
pub enum LogLevel {
    Error,
    Warning,
    Info, // only printed in Debug mode
    
    /// To define your own log level, first parameter is the error level name
    /// and the second is its color \
    /// Example : 
    /// ```
    /// LogLevel::Other("Important".to_string(), "1;32m".to_string())
    /// ```
    Other(String, String),
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                LogLevel::Error => String::from("\x1b[1;31mError"),
                LogLevel::Warning => String::from("\x1b[1;33mWarning"),
                LogLevel::Info => String::from("\x1b[1;34mInfo"),
                LogLevel::Other(ref name, ref color) => format!(
                    "\x1b[{}{}", 
                    *color, 
                    *name
                )
            }
        )
    }
}
