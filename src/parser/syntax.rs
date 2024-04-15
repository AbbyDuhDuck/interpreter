//! # Parser Syntax
//! 
//! Using a tree of [expressions](Expression) you can build a defition to add to a [`Parser`].
//! 

use crate::lexer::{Lexer, ReadPointer, Reader, Token};
use crate::exec::syntax::{Lambda, OwnedLambda};
use super::Parser;

/// Used to define an expression for the [`Parser`] to parse.
#[derive(Clone, Debug)]
pub enum Expression<'a> {
    ExprOr(&'a[Self]),
    SubExpr(&'a[Self]),
    Expr(&'a str),
    Token(&'a str, &'a str),
}

impl Expression<'_> {
    /// Get the resulting [`TreeNode`] from this expression.
    pub fn get<T>(&self, lexer: &Lexer, parser: &Parser, reader: &mut T, lambda: &Lambda) -> Result<TreeNode, String>
    where
        T: Reader,
    {
        let result = match self {
            Expression::ExprOr(expr) => self.get_expr_or(lexer, parser, reader, expr, lambda),
            Expression::SubExpr(expr) => self.get_sub_expr(lexer, parser, reader, expr, lambda),
            Expression::Expr(expr) => self.get_expr(lexer, parser, reader, expr),
            Expression::Token(token, value) => self.get_token(lexer, reader, token, value, lambda),
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
    fn get_expr_or<T>(&self, lexer: &Lexer, parser: &Parser, reader: &mut T, expr: &&[Expression], lambda: &Lambda) -> Result<TreeNode, String>
    where
        T: Reader,
    {
        for (i, subexpr) in expr.iter().enumerate() {
            let sub_lambda = match lambda {
                Lambda::LambdaOr(lambdas) => match lambdas.get(i) {
                    Some(lambda) => lambda,
                    None => return Err(format!("Could not get Lambda for Expression {i} [{}>{}]", expr.len(), lambdas.len()))
                },
                _ => lambda,
            };
            // println!("{lambda}");
            // println!("{sub_lambda}");
            reader.push();
            match subexpr.get(lexer, parser, reader, sub_lambda) {
                Ok(mut node) => {
                    reader.pop();
                    // node.set_lambda(sub_lambda);
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
    fn get_sub_expr<T>(&self, lexer: &Lexer, parser: &Parser, reader: &mut T, expr: &&[Expression], lambda: &Lambda) -> Result<TreeNode, String>
    where
        T: Reader,
    {
        let mut node = TreeNode::from_nodes(
            expr.iter()
            .map(|subexpr| subexpr.get(lexer, parser, reader, &Lambda::Eval))
            .collect::<Result<Vec<TreeNode>, String>>()?
        );
        node.set_lambda(lambda);
        Ok(node)
    }

    /// Get the resulting [TreeNode] for an [`Expr`](Expression::Expr) 
    /// using the passed [`Lexer`], [`Parser`], and [`Reader`].
    fn get_expr<T>(&self, lexer: &Lexer, parser: &Parser, reader: &mut T, expr: &str) -> Result<TreeNode, String>
    where
        T: Reader,
    {
        parser
            .get_expr(expr)?
            .get(lexer, parser, reader)
    }

    /// Get the resulting [TreeNode] for a [`Token`](Expression::Token) 
    /// using the passed [`Lexer`], [`Parser`], and [`Reader`].
    fn get_token<T>(&self, lexer: &Lexer, reader: &mut T, token: &str, value: &str, lambda: &Lambda) -> Result<TreeNode, String>
    where
        T: Reader,
    {
        let tok = lexer.get_next_token(token, reader)
            .ok_or(format!("Could not find token: {token:?}"))?;
        if value != "" && tok.value != value {
            return Err(format!("Could not find token: {token:?} with value {value:?}"));
        };
        reader.next(&tok)?;

        let mut node = TreeNode::from_token(tok);
        node.set_lambda(lambda);
        Ok(node)
    }
}

/// A branch node on an [Abstract Syntax Tree](AbstractSyntaxTree), it can contain other
/// nodes for other brances or an optional [Token] as a leaf.
#[derive(Debug, Clone)]
pub struct TreeNode {
    pub nodes: Vec<Self>,
    pub leaf: Option<Token>,
    pub node_type: String,
    pub lambda: OwnedLambda,
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
        TreeNode { nodes: vec![], leaf: Some(token), node_type: String::new(), lambda: Lambda::EvalToken.into() }
    }
    
    /// Make a branch node from a vector of [TreeNodes](TreeNode).
    pub fn from_nodes(nodes: Vec<TreeNode>) -> TreeNode {
        TreeNode { nodes, leaf: None, node_type: String::new(), lambda: Lambda::Eval.into() }
    }

    /// Make a symbolic [TreeNode] representation of a static [Expression].
    /// 
    /// ---
    /// 
    /// You cannot use [`Expr`](Expression::Expr) in a static representation as
    /// there is no [`Parser`] to reference.
    pub fn from_expr(expr: &Expression) -> TreeNode {
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

    pub fn set_type(&mut self, node_type: String) -> &Self {
        self.node_type = node_type;
        self
    }

    /// Add a [`TreeNode`] branch. 
    pub fn add_branch(&mut self, node: TreeNode<>) {
        self.nodes.push(node);
    }

    /// set the leaf [Token].
    pub fn set_leaf(&mut self, token: Token) {
        self.leaf = Some(token);
    }

    pub fn set_lambda(&mut self, lambda: &Lambda) {
        self.lambda = lambda.into();
    }
}


/// Hold a root [TreeNode] and methods for traversing it.
#[derive(Debug)]
pub struct AbstractSyntaxTree {
    pub root: TreeNode,
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
    use crate::exec::syntax::Lambda::Eval;
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
        parser.define("EXPR", ExprOr(&[
            Token("tok:a", ""),
            Token("tok:b", ""),
            Token("tok:c", ""),
        ]), Eval);
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
        parser.define("EXPR", ExprOr(&[
            SubExpr(&[ Expr("NUM"), Token("op", "+"), Expr("EXPR") ]),
            Expr("NUM"),
        ]), Eval);
        parser.define("NUM", Token("num", ""), Eval);        
        // Setup Parser
        let mut reader = LineReader::new("1+2+3");
        // Parse an expression
        let ast = parser.parse_tree(&lexer, &mut reader)?;
        // Define the expected AST structure
        let exp = TreeNode::from_expr(&ExprOr(&[
            Token("num", "1"),
            Token("op", "+"),
            SubExpr(&[
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
        parser.define("EXPR", Expr("NUM"), Eval);
        parser.define("NUM", Token("tok", ""), Eval);    
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
        parser.define("EXPR", Token("tok:a", ""), Eval);
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
        parser.define("EXPR", ExprOr(&[
            SubExpr(&[ Token("op", "("), Expr("EXPR"), Token("op", ")")]),
            Expr("TOK"),
        ]), Eval);
        parser.define("TOK", Token("tok", ""), Eval);
        
        // let mut reader = LineReader::new("token");
        let mut reader = LineReader::new("((token))");
        
        // Parse an expression
        let ast = parser.parse_tree(&lexer, &mut reader)?;

        // Define the expected AST structure
        let exp = TreeNode::from_expr(&ExprOr(&[
            Token("op", "("),
            SubExpr(&[
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
