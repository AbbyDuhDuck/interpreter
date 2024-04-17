//! # Parser
//! 
//! Contains the definitions for the [`Parser`] and its unit testing.
//! 
//! ---
//! 
//! Note: unit testing is [unimplemented].
//! 

use std::collections::HashMap;
use crate::lexer::{Lexer, Reader};
use crate::exec::syntax::Lambda;
use super::syntax::{AbstractSyntaxTree, Expression, TreeNode};

/// Parser has all the language syntax for a language. It can extract the next Abstract
/// Syntax Tree ([AST](AbstractSyntaxTree)) from a [`Reader`] using a [`Lexer`]. 
pub struct Parser<'a> {
    definitions: HashMap<String, ParserDef<'a>>
}

impl<'a> Parser<'a> {
    pub fn new() -> Parser<'a> {
        Parser { definitions: HashMap::new() }
    }

    /// Use a [`Lexer`] and a [`Reader`] to parse the next [`Expression`] from the Reader's content.
    pub fn parse_tree<T>(&self, lexer: &Lexer, reader: &mut T) -> Result<AbstractSyntaxTree, String>
    where T: Reader {
        // println!("Parsing an Expression");
        let expr = match self.definitions.get("EXPR") {
            Some(expr) => expr,
            None => { 
                return Err("You need to define an Expression for EXPR".into());
            }
        };
        let root = expr.get(lexer, &self, reader)?;
        reader.commit();
        Ok(AbstractSyntaxTree::new(root))
    }

    /// Get a defined [`Expression`] from the parser.
    pub fn get_expr(&self, expr: &str) -> Result<&ParserDef, String> {
        self.definitions.get(expr).ok_or_else(|| format!("Parser has no definition for `{expr}`"))
    }

    /// Define an [`Expression`] that can be matched in [`parse_tree`](Parser::parse_tree).
    pub fn define(&mut self, expr_type: &str, expr: Expression<'a>, lambda: Lambda<'a>) {
        // transform to a sub object with both an expr and a lambda
        self.definitions.insert(expr_type.to_owned(), ParserDef::from(expr, lambda));
    }
}

// -=-=- Parser Definition -=-=- //

pub struct ParserDef<'a> {
    expr: Expression<'a>,
    lambda: Lambda<'a>,
}

impl ParserDef<'_> {
    pub fn from<'a>(expr: Expression<'a>, lambda: Lambda<'a>) -> ParserDef<'a> {
        ParserDef { expr, lambda }
    }

    // -=-=- //

    pub fn get<T>(&self, lexer: &Lexer, parser: &Parser, reader: &mut T) -> Result<TreeNode, String>
    where T: Reader
    {
        self.expr.get(lexer, parser, reader, &self.lambda)
    }

}

// -=-=-=-=- Unit Tests -=-=-=-=- //

// TODO: Make unit tests
