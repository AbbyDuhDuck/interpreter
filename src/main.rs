//! # Main Entry Point
//! 
//! Where the interpreter is launched from.
//! 
//! 
//! Note: to remove backtracing run `$env:RUST_BACKTRACE=0`
//! 

/// The main entry point for our program
fn main() {
    interpreter::run();
}