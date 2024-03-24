//! # Interpreter Tutorial
//! 
//! Something something, basic interpreter in rust. 

use regex::Regex;

mod macros;
pub mod lexer;
pub mod parser;

/// run a basic input loop where the user will be prompted with `@>` or `#>` to enter
/// code to be executed.
/// 
/// ---
/// 
/// it can be started with `interpreter::run()` or by running the interpreter executable.
pub fn run() {
    use macros::io::*;
    loop {
        // prompt the user for input
        let raw = prompt!("@> ");
        let input = raw.trim();
        if input == "exit" {
            break;
        }
        // exec the input
        let result = match exec(input) {
            Ok(val) => val,
            Err(err) => {
                // this is where you can check for ErrorEOF
                println!("Encountered Error: {err}");
                continue;
            }
        };
        // display the result
        println!("{result}");
        println!("---");
    }
}

/// Executes a line of our custom programming language using a typical, yet complex process.
/// 
/// ---
/// 
/// Uses a LineReader and Lexer to make a list of Tokens - those are fed to a Parser to 
/// produce an Abstract Syntax Tree - finally the Interpreter will traverse the AST and
/// run your code - producing a Result.
pub fn exec(expr_str: &str) -> Result<String, String>{
    use lexer::Reader;
    // line reader
    let reader = lexer::LineReader::new(expr_str);

    // short test
    let re_num = Regex::new("^[0-9]+").unwrap();
    let re_str = Regex::new("^[a-zA-Z_]+").unwrap();
    if let Some(val) = reader.read_regex(re_num) {
        return Ok(format!("Found Number: {val}"))
    }
    if let Some(val) = reader.read_regex(re_str) {
        return Ok(format!("Found String: {val:?}"))
    }

    // lexer (tokenizer)

    // parser (ast builder)

    // interpreter
    Ok(format!("Ping: {expr_str:?}"))
}