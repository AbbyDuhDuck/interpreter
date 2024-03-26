//! # Interpreter Tutorial
//! 
//! Something something, basic interpreter in rust. 

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
    // line reader
    let reader = lexer::LineReader::new(expr_str);

    // lexer (tokenizer)
    let mut _lexer = lexer::Lexer::new();
    _lexer.define(lexer::TokenDef::new("num", "[0-9]+")?);
    _lexer.define(lexer::TokenDef::new("str", "[a-zA-Z_]+")?);

    if let Some(tok) = _lexer.get_next_token("num", &reader) {
        println!("Hey you found the number: {}", tok.value);
    }

    if let Some(tok) = _lexer.get_next_any(&reader) {
        return Ok(format!("Found: {tok:?}"));
    };

    // parser (ast builder)

    // interpreter
    Ok(format!("Ping: {expr_str:?}"))
}