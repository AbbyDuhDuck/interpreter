use crate::lexer::{Lexer, Reader};



#[derive(Clone, Debug)]
pub enum Expression<'a> {
    SubExpr(Vec<Self>),
    Expr(&'a str),
    Token(&'a str),
}

impl Expression<'_> {
    pub fn get<T>(&self, lexer: &Lexer, reader: &T) -> Result<TreeNode, String>
    where T: Reader {
        match self {
            Expression::SubExpr(expr) => self.get_sub_expr(lexer, reader, expr),
            Expression::Expr(expr) => self.get_expr(lexer, reader, expr),
            Expression::Token(token) => self.get_token(lexer, reader, token),
            // _ => Ok(TreeNode::new())
        }
    }

    pub fn get_sub_expr<T>(&self, lexer: &Lexer, reader: &T, expr: &Vec<Expression>) -> Result<TreeNode, String>
    where T: Reader {
        println!("Trying to match sub-expresion {expr:?}");
        Err("Expression::SubExpr not implemented yet".into())
        // Ok(TreeNode::new())
    }
    pub fn get_expr<T>(&self, lexer: &Lexer, reader: &T, expr: &str) -> Result<TreeNode, String>
    where T: Reader {
        println!("Trying to match expresion {expr:?}");
        Err("Expression::Expr not implemented yet".into())
        // Ok(TreeNode::new())
    }
    pub fn get_token<T>(&self, lexer: &Lexer, reader: &T, token: &str) -> Result<TreeNode, String>
    where T: Reader {
        println!("Trying to match token {token:?}");
        Err("Expression::Token not implemented yet".into())
        // Ok(TreeNode::new())
    }
}


#[derive(Debug)]
pub struct TreeNode {

}

impl TreeNode {
    pub fn new() -> TreeNode {
        TreeNode {}
    }
}

#[derive(Debug)]
pub struct AbstractSyntaxTree {
    root: TreeNode,
}

impl AbstractSyntaxTree {
    pub fn new(root: TreeNode) -> Self {
        AbstractSyntaxTree { root }
    }
}