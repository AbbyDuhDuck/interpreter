use super::super::lexer;
use super::syntax;

use lexer::{Lexer, Reader};
use syntax::{AbstractSyntaxTree, TreeNode};


pub enum RuleType<'a> {
    Token(&'a str),
    BinOp(&'a str, &'a str),
    UniOp(&'a str),
}

pub struct ParserRule {
}

pub struct Parser {

}

impl Parser {
    pub fn new() -> Parser {
        Parser { }
    }

    pub fn parse_tree<T>(&self, lexer: &Lexer, reader: &T) -> AbstractSyntaxTree
    where T: Reader {
        AbstractSyntaxTree::new(TreeNode::new())
    }

    pub fn define(&mut self, rule_type: &str, rule: RuleType) {

    }

    pub fn define_expr(&mut self, def: ParserRule) {
        
    }
}