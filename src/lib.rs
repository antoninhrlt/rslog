// This file is part of "rslog"
// Under the MIT License
// Copyright (c) Antonin Hérault

pub mod level;
pub mod log;
pub mod logger;

/// Transform a line of tokens to a printable string for a log \
/// `token_i` is token to highlight iterator
pub fn line_to_string<Token>(line: &Vec<Token>, token_i: usize) -> String 
where Token: ToString
{
    let mut result = String::from("\t");
    
    let mut i = 0;
    for token in line {
        if token_i > 0 && i == token_i - 1 {
            result += &"\x1b[31m";
        }
        result += &format!("{}\x1b[0m ", token.to_string());
        i += 1;
    }
    result += "\n\n";
    result
}

pub fn source_to_string(source: String, line_i: usize, token_i: usize) -> String {
    format!("in '{}' at ({}, {})", source, line_i + 1, token_i + 1)
}

/// NOTE you should run the test with parameters: "-- --nocapture" to see the
/// outputs of the logs
#[test]
fn test() {
    use crate::level::LogLevel;
    use crate::log::Log;
    use crate::logger::Logger;

    let mut logger = Logger::new();

    let logs: Vec<Log> = vec![
        Log::info("This is an info log".to_string()),
        Log::new(
            LogLevel::Error,
            "This is an error log".to_string(),
            "This message is for this error log".to_string(),
        ),
        Log::new(
            LogLevel::Warning,
            "This is a warning log".to_string(),
            "This message is for this warning log".to_string(),
        ),
        Log::new(
            LogLevel::Error,
            "This is an error log with an hint".to_string(),
            "This message is for this error log".to_string(),
        )
        .add_hint("This hint is for this error log".to_string()),
        Log::new(
            LogLevel::Error,
            "This is an error log with a cause and an hint".to_string(),
            "This message is for this error log".to_string(),
        )
        .add_cause(&"This cause is for this error log".to_string())
        .add_hint("This hint is for this error log".to_string()),
        Log::info("This is another info log".to_string()),
    ];

    for log in logs {
        logger.add_log(log);
    }

    logger.interpret();
}

#[test]
fn for_example() {
    use crate::*;
    use crate::level::LogLevel;
    use crate::log::Log;
    use crate::logger::Logger;

    // code_line[2] is invalid  !
    let code_line: Vec<&str> = vec!["fn", "main", "@", "{", "}"];

    let mut logger = Logger::new();

    let logs = vec![
        Log::info("Working directory : /Documents/somewhere".to_string()),

        // so, we throw an error
        Log::new(
            LogLevel::Error,
            "Unexpected token".to_string(),
            format!(
                "{}Token '{}' found but not expected", 
                line_to_string::<&str>(&code_line, 3), // not index, but position
                code_line[2]
            ),
        )
        .add_cause(&source_to_string("foo.rs".to_string(), 0, 2)) // 1st line, 3rd token
        .add_hint(format!("Add a parameters list after '{}'", code_line[1]))
    ];

    for log in logs {
        logger.add_log(log);
    }

    // Print all logs and kill the program process if at least one log is an error
    logger.interpret();
}
