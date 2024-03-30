//! # Parser Syntax
//! 
//! Using a tree of [expressions](Expression) you can build a defition to add to a [`Parser`].
//! 

use crate::lexer::{Lexer, ReadPointer, Reader, Token};
use super::Parser;

/// Used to define an expression for the [`Parser`] to parse.
#[derive(Clone, Debug)]
pub enum Expression<'a> {
    ExprOr(Vec<Self>),
    SubExpr(Vec<Self>),
    Expr(&'a str),
    Token(&'a str, &'a str),
}

impl Expression<'_> {
    /// Get the resulting [`TreeNode`] from this expression.
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
        result
    }

    /// Get the expression as a token - if the expression is not a [`Token`], it
    /// will [panic] with an error message.
    pub fn token(&self) -> Token {
        match self {
            Expression::Token(token_type, value) => Token::new(
                token_type,
                value,
                ReadPointer::from_pos((0,0,0,0), (0,0))
            ),
            _ => panic!("`token` can only be used on an `Expression::Token`!")
        }
    }

    /// Get the resulting [TreeNode] for an [`ExprOr`](Expression::ExprOr) 
    /// using the passed [`Lexer`], [`Parser`], and [`Reader`].
    fn get_expr_or<T>(&self, lexer: &Lexer, parser: &Parser, reader: &mut T, expr: &Vec<Expression>) -> Result<TreeNode, String>
    where
        T: Reader,
    {
        for subexpr in expr {
            reader.push();
            match subexpr.get(lexer, parser, reader) {
                Ok(node) => {
                    reader.pop();
                    return Ok(node);
                }
                Err(_) => {
                    reader.back();
                    continue
                },
            };
        }
        Err(format!("Could find matching expression for: {self:?}"))
    }

    /// Get the resulting [TreeNode] for a [`SubExpr`](Expression::SubExpr) 
    /// using the passed [`Lexer`], [`Parser`], and [`Reader`].
    fn get_sub_expr<T>(&self, lexer: &Lexer, parser: &Parser, reader: &mut T, expr: &Vec<Expression>) -> Result<TreeNode, String>
    where
        T: Reader,
    {
        Ok(TreeNode::from_nodes(
            expr.iter()
            .map(|subexpr| subexpr.get(lexer, parser, reader))
            .collect::<Result<Vec<TreeNode>, String>>()?
        ))
    }

    /// Get the resulting [TreeNode] for an [`Expr`](Expression::Expr) 
    /// using the passed [`Lexer`], [`Parser`], and [`Reader`].
    fn get_expr<T>(&self, lexer: &Lexer, parser: &Parser, reader: &mut T, expr: &str) -> Result<TreeNode, String>
    where
        T: Reader,
    {
        parser.get_expr(expr)?.get(lexer, parser, reader)
    }

    /// Get the resulting [TreeNode] for a [`Token`](Expression::Token) 
    /// using the passed [`Lexer`], [`Parser`], and [`Reader`].
    fn get_token<T>(&self, lexer: &Lexer, reader: &mut T, token: &str, value: &str) -> Result<TreeNode, String>
    where
        T: Reader,
    {
        let tok = lexer.get_next_token(token, reader)
            .ok_or(format!("Could not find token: {token:?}"))?;
        if value != "" && tok.value != value {
            return Err(format!("Could not find token: {token:?} with value {value:?}"));
        };
        reader.next(&tok)?;

        Ok(TreeNode::from_token(tok))
    }
}

/// A branch node on an [Abstract Syntax Tree](AbstractSyntaxTree), it can contain other
/// nodes for other brances or an optional [Token] as a leaf.
#[derive(Debug)]
pub struct TreeNode {
    nodes: Vec<Self>,
    leaf: Option<Token>,
}

/// Implement display so the [`TreeNode`] can be displayed nicely.
/// 
/// ---
/// 
/// Note: this will effect the outcome of [`assert_ast`] and [`assert_ast_ne`]
/// if changed.
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
    // We might not need a public `new` constructor
    // fn new() -> TreeNode {
    //     TreeNode { nodes: vec![], leaf: None }
    // }

    /// Make a leaf node from a [`Token`]
    pub fn from_token(token: Token) -> TreeNode {
        TreeNode { nodes: vec![], leaf: Some(token) }
    }
    
    /// Make a branch node from a vector of [TreeNodes](TreeNode).
    pub fn from_nodes(nodes: Vec<TreeNode>) -> TreeNode {
        TreeNode { nodes, leaf: None }
    }

    /// Make a symbolic [TreeNode] representation of a static [Expression].
    /// 
    /// ---
    /// 
    /// You cannot use [`Expr`](Expression::Expr) in a static representation as
    /// there is no [`Parser`] to reference.
    pub fn from_expr<'a>(expr: Expression<'a>) -> TreeNode {
        match expr {
            Expression::ExprOr(nodes) | Expression::SubExpr(nodes) => {
                let nodes = nodes.into_iter().map(TreeNode::from_expr).collect();
                TreeNode::from_nodes(nodes)
            }
            Expression::Expr(_) => {
                panic!("You can't use a reference when building a symbolic tree.")
            }
            Expression::Token(token, value) => {
                let tok = Expression::Token(token, value).token();
                TreeNode::from_token(tok)
            }
        }
    }

    /// Add a [`TreeNode`] branch. 
    pub fn add_branch(&mut self, node: TreeNode<>) {
        self.nodes.push(node);
    }

    /// set the leaf [Token].
    pub fn set_leaf(&mut self, token: Token) {
        self.leaf = Some(token);
    }
}


/// Hold a root [TreeNode] and methods for traversing it.
#[derive(Debug)]
pub struct AbstractSyntaxTree {
    root: TreeNode,
}

/// Implement the Display for the [AbstractSyntaxTree].
/// 
/// ---
/// 
/// Note: this will effect the outcome of [`assert_ast`] and [`assert_ast_ne`]
/// if changed.
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

// -=-=-=-=- Unit Tests -=-=-=-=- //

/// Tests to ensure the [AbstractSyntaxTree] and Expression's [`get`](Expression::get)
/// method is working.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::LineReader;
    use Expression::*;

    /// assert an [`ExprOr`] expression.
    #[test]
    fn test_get_expr_or() -> Result<(), String> {
        // Setup Lexer
        let mut lexer = Lexer::new();
        lexer.define("tok:a", "[a-c]+")?;
        lexer.define("tok:b", "[d-f]+")?;
        lexer.define("tok:c", "[g-i]+")?;
        // Setup Parser
        let mut parser = Parser::new();
        parser.define("EXPR", ExprOr(vec![
            Token("tok:a", ""),
            Token("tok:b", ""),
            Token("tok:c", ""),
        ]));
        // Setup Reader
        let mut reader = LineReader::new("abcdefghi");
        // Parse an expression
        let ast_1 = parser.parse_tree(&lexer, &mut reader)?;
        let ast_2 = parser.parse_tree(&lexer, &mut reader)?;
        let ast_3 = parser.parse_tree(&lexer, &mut reader)?;
        // Define the expected AST structure
        let exp_1 = TreeNode::from_token(Token("tok:a", "abc").token());
        let exp_2 = TreeNode::from_token(Token("tok:b", "def").token());
        let exp_3 = TreeNode::from_token(Token("tok:c", "ghi").token());
        // Assert that the AST matches the expected structure
        assert_ast!(exp_1, ast_1);
        assert_ast!(exp_2, ast_2);
        assert_ast!(exp_3, ast_3);
        Ok(())
    }

    /// assert a [`SubExpr`] expression.
    #[test]
    fn test_get_sub_expr() -> Result<(), String> {
        // Setup Lexer
        let mut lexer = Lexer::new();
        lexer.define("num", "[0-9]+")?;
        lexer.define("op", "\\+|\\(|\\)")?;
        // Expressions
        let mut parser = Parser::new();
        parser.define("EXPR", ExprOr(vec![
            SubExpr(vec![ Expr("NUM"), Token("op", "+"), Expr("EXPR") ]),
            Expr("NUM"),
        ]));
        parser.define("NUM", Token("num", ""));        
        // Setup Parser
        let mut reader = LineReader::new("1+2+3");
        // Parse an expression
        let ast = parser.parse_tree(&lexer, &mut reader)?;
        // Define the expected AST structure
        let exp = TreeNode::from_expr(ExprOr(vec![
            Token("num", "1"),
            Token("op", "+"),
            SubExpr(vec![
                Token("num", "2"),
                Token("op", "+"),
                Token("num", "3"),
            ]),
        ]));
        // Assert that the AST matches the expected structure
        assert_ast!(exp, ast);
        Ok(())
    }
    
    /// assert an [`Expr`] expression.
    #[test]
    fn test_get_expr() -> Result<(), String> {
        // Setup Lexer
        let mut lexer = Lexer::new();
        lexer.define("tok", "[a-z]+")?;
        lexer.define("op", "\\(|\\)")?;
        // Setup Parser
        let mut parser = Parser::new();
        parser.define("EXPR", Expr("NUM"));
        parser.define("NUM", Token("tok", ""));    
        // Setup Parser
        let mut reader = LineReader::new("token");
        // Parse an expression
        let ast = parser.parse_tree(&lexer, &mut reader)?;
        // Define the expected AST structure
        let exp = TreeNode::from_token(Token("tok", "token").token());
        // Assert that the AST matches the expected structure
        assert_ast!(exp, ast);
        Ok(())
    }
    
    /// assert a [`Token`](Expression::Token) expression.
    #[test]
    fn test_get_token() -> Result<(), String> {
        // Setup Lexer and Parser
        let mut lexer = Lexer::new();
        lexer.define("tok:a", "[a-c]+")?;
        let mut parser = Parser::new();
        parser.define("EXPR", Token("tok:a", ""));
        let mut reader = LineReader::new("abc");
        // Parse an expression
        let ast = parser.parse_tree(&lexer, &mut reader)?;
        // Define the expected AST structure
        let exp = TreeNode::from_token(Token("tok:a", "abc").token());
        // Assert that the AST matches the expected structure
        assert_ast!(exp, ast);
        Ok(())
    }

    /// Make sure recursion works
    #[test]
    fn test_recursion() -> Result<(), String> {
        // Setup Lexer and Parser
        let mut lexer = Lexer::new();
        lexer.define("tok", "[a-z]+")?;
        lexer.define("op", "\\(|\\)")?;

        let mut parser = Parser::new();
        parser.define("EXPR", ExprOr(vec![
            SubExpr(vec![ Token("op", "("), Expr("EXPR"), Token("op", ")")]),
            Expr("TOK"),
        ]));
        parser.define("TOK", Token("tok", ""));
        
        // let mut reader = LineReader::new("token");
        let mut reader = LineReader::new("((token))");
        
        // Parse an expression
        let ast = parser.parse_tree(&lexer, &mut reader)?;

        // Define the expected AST structure
        let exp = TreeNode::from_expr(ExprOr(vec![
            Token("op", "("),
            SubExpr(vec![
                Token("op", "("),
                Token("tok", "token"),
                Token("op", ")"),
            ]),
            Token("op", ")"),
        ]));

        // Assert that the AST matches the expected structure
        assert_ast!(exp, ast);
        Ok(())
    }
}
