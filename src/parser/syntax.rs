use std::fmt::format;

use crate::lexer::{Lexer, Reader, SizeType, Token};

use super::Parser;



#[derive(Clone, Debug)]
pub enum Expression<'a> {
    ExprOr(Vec<Self>),
    SubExpr(Vec<Self>),
    Expr(&'a str),
    Token(&'a str, &'a str),
}

impl Expression<'_> {
    pub fn get<T>(&self, lexer: &Lexer, parser: &Parser, reader: &mut T) -> Result<TreeNode, String>
    where
        T: Reader,
    {
        let result = match self {
            Expression::ExprOr(expr) => self.get_expr_or(lexer, parser, reader, expr),
            Expression::SubExpr(expr) => self.get_sub_expr(lexer, parser, reader, expr),
            Expression::Expr(expr) => self.get_expr(lexer, parser, reader, expr),
            Expression::Token(token, value) => self.get_token(lexer, reader, token, value),
        };
        if result.is_err() {
            // reader.pop();
        }
        result
    }
    pub fn get_expr_or<T>(&self, lexer: &Lexer, parser: &Parser, reader: &mut T, expr: &Vec<Expression>) -> Result<TreeNode, String>
    where
        T: Reader,
    {
        todo!()
    }

    pub fn get_sub_expr<T>(&self, lexer: &Lexer, parser: &Parser, reader: &mut T, expr: &Vec<Expression>) -> Result<TreeNode, String>
    where
        T: Reader,
    {
        todo!()
    }

    pub fn get_expr<T>(&self, lexer: &Lexer, parser: &Parser, reader: &mut T, expr: &str) -> Result<TreeNode, String>
    where
        T: Reader,
    {
        todo!()
    }

    pub fn get_token<T>(&self, lexer: &Lexer, reader: &mut T, token: &str, value: &str) -> Result<TreeNode, String>
    where
        T: Reader,
    {
        todo!()
    }
}



#[derive(Debug)]
pub struct TreeNode {
    nodes: Vec<Self>,
    leaf: Option<Token>,
}

impl std::fmt::Display for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(leaf) = &self.leaf {
            write!(f, "{}", leaf)?;
        } else {
            write!(f, "( ")?;
            self.nodes.iter().enumerate().try_for_each(|(i, node)| {
                if i > 0 { write!(f, " ")?; }
                write!(f, "{}", node)
            })?;
            write!(f, " )")?;
        }
        Ok(())
    }
}

impl TreeNode {
    pub fn new() -> TreeNode {
        TreeNode { nodes: vec![], leaf: None }
    }

    pub fn add_branch(&mut self, node: TreeNode<>) {
        self.nodes.push(node);
    }

    pub fn set_leaf(&mut self, token: Token) {
        self.leaf = Some(token);
    }
}



#[derive(Debug)]
pub struct AbstractSyntaxTree {
    root: TreeNode,
}

impl std::fmt::Display for AbstractSyntaxTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.root)
    }
}

impl AbstractSyntaxTree {
    pub fn new(root: TreeNode) -> Self {
        AbstractSyntaxTree { root }
    }
}