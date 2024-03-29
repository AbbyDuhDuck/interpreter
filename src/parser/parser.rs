use std::collections::HashMap;

use super::{super::lexer, syntax::Expression};
use super::syntax;

use lexer::{Lexer, Reader};
use syntax::{AbstractSyntaxTree, TreeNode};

pub enum RuleType<'a> {
    Token(&'a str),
    BinOp(&'a str, &'a str),
    UniOp(&'a str),
}

pub struct ParserRule {
    expr_type: String,
    // expr_rules: Vec<>
}

pub struct Parser<'a> {
    definitions: HashMap<String, Expression<'a>>
}

impl<'a> Parser<'a> {
    pub fn new() -> Parser<'a> {
        Parser { definitions: HashMap::new() }
    }

    pub fn parse_tree<T>(&self, lexer: &Lexer, reader: &mut T) -> Result<AbstractSyntaxTree, String>
    where T: Reader {
        println!("Parsing an Expression");
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

    pub fn get_expr(&self, expr: &str) -> Result<&Expression, String> {
        self.definitions.get(expr).ok_or(format!("Parser has no expr for `{expr}`"))
    }

    pub fn define(&mut self, expr_type: &str, expr: Expression<'a>) {
        // println!("{expr_type} {expr:#?}");
        self.definitions.insert(expr_type.to_owned(), expr);
    }
}