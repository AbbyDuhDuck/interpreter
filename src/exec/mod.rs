
mod exec;
pub mod syntax;


use std::ops::Deref;

pub use exec::*;
use once_cell::sync::Lazy;

use crate::{lexer::{Lexer, Reader}, parser:: Parser};

pub struct Executor<'a> {
    lexer: Lexer,
    parser: Parser<'a>,
    env: VirtualEnv,
}

impl Executor<'_> {
    pub fn new(lexer: Lexer, parser: Parser, env: VirtualEnv) -> Executor {
        Executor { lexer, parser, env }
    }

    pub fn math() -> Executor<'static> {
        crate::lang::math::exec()
    }

    pub fn exec<T>(&mut self, reader: &mut T) -> Result<String, String> where T: Reader{
        let ast = self.parser.parse_tree(&self.lexer, reader)?;
        // println!("AST:\n{ast:}");

        // -=- interpreter -=- //
        self.env.set_ident("thing", exec::NodeValue::Integer(-1));
        let result = self.env.exec(ast);

        match result {
            StateNode::None => Ok("None".into()),
            StateNode::Value(val) => Ok(val.to_string().unwrap_or_default()),
            
            StateNode::RuntimeErr(err) => Err(err),
            StateNode::Node(node) => Err(format!("Node Result: {node}")),
        }
    }
}