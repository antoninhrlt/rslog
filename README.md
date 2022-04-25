# rslog
Generate beautiful logs for your project. Rust library. Originally designed for 
a compiler/interpreter

## Install
In the "Cargo.toml" file of your project :
```toml
[dependencies]
rslog = { git = "https://github.com/antoninhrlt/rslog" }
```

## Example
```rust
// Token type == &str
// code_line[2] is invalid  !
let code_line: Vec<&str> = vec!["fn", "main", "@", "{", "}"];

let mut logger = Logger::new();

let logs = vec![
    Log::info("Working directory : /Documents/somewhere".to_string()),

    // So, we throw an error
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
```

<details>
<summary>Click to see imports</summary>
    
```rust
use rslog::*;
use rslog::level::LogLevel;
use rslog::log::Log;
use rslog::logger::Logger;
```
</details>
