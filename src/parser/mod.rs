//! # Parser
//! 
//! Parses a bunch of tokens to into an Abstract Syntax Tree.
//! 

#[macro_use]
mod macros;

pub mod syntax;
mod parser;

pub use parser::*;