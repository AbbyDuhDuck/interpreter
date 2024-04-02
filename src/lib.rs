//! # Interpreter Tutorial
//! 
//! Something something, basic interpreter in rust. 

mod macros;
pub mod lexer;
pub mod parser;
pub mod exec;
pub mod lang;

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
    use lang::math;

    // -=- line reader -=- //
    let mut reader = lexer::LineReader::new(expr_str);

    // -=- lexer (tokenizer) -=- //
    let _lexer = math::LEXER;

    // let mut _lexer = lexer::Lexer::new(); // typically you would make this only once.
    // _lexer.define("num", "[0-9]+")?;
    // _lexer.define("str", "[a-zA-Z_]+")?;

    // match by token type
    if let Some(tok) = _lexer.get_next_token("num", &reader) {
        println!("Hey you found the number: {}", tok.value);
    }

    // match any
    if let Some(tok) = _lexer.get_next_any(&reader) {
        println!("Found: {tok:?}");
    };

    // -=- parser (ast builder) -=- //
    let parser = math::PARSER;

    // let mut parser = parser::Parser::new(); // typically you would make this only once.
    // parser.define("expr", parser::RuleType::Token("num")); // TODO

    let ast = parser.parse_tree(&_lexer, &mut reader)?;
    println!("AST:\n{ast:}");

    // -=- interpreter -=- //
    let env = math::ENV;
    let result = env.exec(ast);
    println!("Result: {result:?}");

    // just ping back the input for now
    Ok(format!("Ping: {expr_str:?}"))
}