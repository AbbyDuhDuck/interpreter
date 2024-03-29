use std::fmt::format;

use crate::lexer::{Lexer, ReadPointer, Reader, SizeType, Token};

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
        println!("TRY - {self:?}");
        let result = match self {
            Expression::ExprOr(expr) => self.get_expr_or(lexer, parser, reader, expr),
            Expression::SubExpr(expr) => self.get_sub_expr(lexer, parser, reader, expr),
            Expression::Expr(expr) => self.get_expr(lexer, parser, reader, expr),
            Expression::Token(token, value) => self.get_token(lexer, reader, token, value),
        };
        println!("FOUND ({}) - {}", self.get_name(), match &result {
            Ok(node) => format!("{node}"),
            Err(err) => err.into()
        });
        result
    }

    fn get_name(&self) -> &'static str {
        match self {
            Expression::ExprOr(_) => "ExprOr",
            Expression::SubExpr(_) => "SubExpr",
            Expression::Expr(_) => "Expr",
            Expression::Token(_, _) => "Token",
        }
    }

    pub fn token(&self) -> Token {
        if let Expression::Token(token_type, value) = self {
            Token::new(
                token_type,
                value,
                ReadPointer::from_pos((0,0,0,0), (0,0))
            )
        } else {
            panic!()
        }
    }

    pub fn get_expr_or<T>(&self, lexer: &Lexer, parser: &Parser, reader: &mut T, expr: &Vec<Expression>) -> Result<TreeNode, String>
    where
        T: Reader,
    {
        for subexpr in expr {
            reader.push();
            match subexpr.get(lexer, parser, reader) {
                Ok(node) => {
                    // reader.commit();
                    return Ok(node);
                }
                Err(_) => {
                    reader.pop();
                    continue
                },
            };
        }
        Err(format!("Could find matching expression for: {self:?}"))
    }

    pub fn get_sub_expr<T>(&self, lexer: &Lexer, parser: &Parser, reader: &mut T, expr: &Vec<Expression>) -> Result<TreeNode, String>
    where
        T: Reader,
    {
        reader.push();

        let mut nodes = vec![];
        for subexpr in expr {
            let node_result = subexpr.get(lexer, parser, reader);
            if let Err(err) = node_result {
                reader.pop();
                return Err(err)
            };
            println!("PTR {}", reader.get_pointer());
            nodes.push(node_result.unwrap());
        };

        // reader.commit();
        Ok(TreeNode::from_nodes(nodes))
    }

    pub fn get_expr<T>(&self, lexer: &Lexer, parser: &Parser, reader: &mut T, expr: &str) -> Result<TreeNode, String>
    where
        T: Reader,
    {
        reader.push();
        let node = parser.get_expr(expr)?.get(lexer, parser, reader);
        if let Err(_) = node {
            reader.pop();
        }
        node
    }

    pub fn get_token<T>(&self, lexer: &Lexer, reader: &mut T, token: &str, value: &str) -> Result<TreeNode, String>
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

    pub fn from_token(token: Token) -> TreeNode {
        TreeNode { nodes: vec![], leaf: Some(token) }
    }
    
    pub fn from_nodes(nodes: Vec<TreeNode>) -> TreeNode {
        TreeNode { nodes, leaf: None }
    }

    pub fn from_expr(expr: Expression<'static>) -> TreeNode {
        match expr {
            Expression::ExprOr(nodes) | Expression::SubExpr(nodes) => {
                let nodes = nodes.into_iter().map(TreeNode::from_expr).collect();
                TreeNode { nodes, leaf: None }
            }
            Expression::Expr(_) => {
                panic!("You can't use a reference when building a symbolic tree.")
            }
            Expression::Token(token, value) => {
                let tok = Expression::Token(token, value).token();
                TreeNode { nodes: vec![], leaf: Some(tok) }
            }
        }
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


// -=-=-=-=- Testing Macros -=-=-=-=- //

macro_rules! assert_ast {
    ( $ast:expr, $expected:expr ) => {
        {
            let ast_str = format!("{}", $ast);
            let expected_str = format!("{}", $expected);
            assert_eq!(ast_str, expected_str, "Expected AST: {}, Actual AST: {}", expected_str, ast_str);
        }
    };
}
macro_rules! assert_ast_ne {
    ( $ast:expr, $expected:expr ) => {
        {
            let ast_str = format!("{}", $ast);
            let expected_str = format!("{}", $expected);
            assert_ne!(ast_str, expected_str, "Expected AST: {}, Actual AST: {}", expected_str, ast_str);
        }
    };
}

// -=-=-=-=- Unit Tests -=-=-=-=- //

#[cfg(test)]
mod macro_tests {
    use super::*;
    use crate::lexer::LineReader;
    use Expression::*;
    
    #[test]
    fn test_expect_ast_macro_token() -> Result<(), String> {
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

    
    #[test]
    fn test_expect_ast_macro_token_ne() -> Result<(), String> {
        // Setup Lexer and Parser
        let mut lexer = Lexer::new();
        lexer.define("tok:a", "[a-c]+")?;
        let mut parser = Parser::new();
        parser.define("EXPR", Token("tok:a", ""));
        let mut reader = LineReader::new("abc");
        // Parse an expression
        let ast = parser.parse_tree(&lexer, &mut reader)?;
        // Define the expected AST structure
        let exp = TreeNode::from_token(Token("tok:a", "def").token());
        // Assert that the AST doesn't match the expected structure
        assert_ast_ne!(exp, ast);
        Ok(())
    }

    #[test]
    fn test_expect_ast_macro_expr_or() -> Result<(), String> {
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
    
    #[test]
    fn test_expect_ast_macro_expr_or_sub_expr() -> Result<(), String> {
        // Setup Lexer
        let mut lexer = Lexer::new();
        lexer.define("tok:a", "[a-c]+")?;
        lexer.define("tok:b", "[d-f]+")?;
        lexer.define("tok:c", "[g-i]+")?;
        // Setup Parser
        let mut parser = Parser::new();
        parser.define("EXPR", ExprOr(vec![
            SubExpr(vec![Token("tok:a", ""), Token("tok:b", ""),]),
            Token("tok:a", ""),
            Token("tok:c", ""),
        ]));
        // Setup Reader
        let mut reader = LineReader::new("ghiabcdefabcghi");
        // Parse an expression
        let ast_1 = parser.parse_tree(&lexer, &mut reader)?;
        let ast_2 = parser.parse_tree(&lexer, &mut reader)?;
        let ast_3 = parser.parse_tree(&lexer, &mut reader)?;
        let ast_4 = parser.parse_tree(&lexer, &mut reader)?;
        // Define the expected AST structure
        let exp_1 = TreeNode::from_token(Token("tok:c", "ghi").token());
        let exp_2 = TreeNode::from_expr(SubExpr(vec![
            Token("tok:a", "abc"),
            Token("tok:b", "def"),
        ]));
        let exp_3 = TreeNode::from_token(Token("tok:a", "abc").token());
        let exp_4 = TreeNode::from_token(Token("tok:c", "ghi").token());
        // Assert that the AST matches the expected structure
        assert_ast!(exp_1, ast_1);
        assert_ast!(exp_2, ast_2);
        assert_ast!(exp_3, ast_3);
        assert_ast!(exp_4, ast_4);
        Ok(())
    }

    #[test]
    fn test_expect_ast_macro_expr() -> Result<(), String> {
        // Setup Lexer
        let mut lexer = Lexer::new();
        lexer.define("tok", "[a-z]+")?;
        lexer.define("op", "\\(|\\)")?;
        // Setup Parser
        let mut parser = Parser::new();
        parser.define("EXPR", ExprOr(vec![
            Expr("NUM"),
        ]));
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

    #[test]
    fn test_expect_ast_macro_sub_expr() -> Result<(), String> {
        // Setup Lexer
        let mut lexer = Lexer::new();
        lexer.define("num", "[0-9]+")?;
        lexer.define("op", "\\+|\\(|\\)")?;
        // Expressions
        let mut parser = Parser::new();
        parser.define("EXPR", ExprOr(vec![
            SubExpr(vec![ Expr("VAL"), Token("op", "+"), Expr("EXPR") ]),
            Expr("VAL"),
        ]));
        parser.define("VAL", ExprOr(vec![
            // SubExpr(vec![ Token("op", "("), Expr("EXPR"), Token("op", ")") ]),
            Expr("NUM"),
        ]));
        parser.define("NUM", Token("num", ""));        
        // Setup Parser
        let mut reader = LineReader::new("1");
        // let mut reader = LineReader::new("1+2");
        // let mut reader = LineReader::new("1+(2+3)");
        // Parse an expression
        let ast = parser.parse_tree(&lexer, &mut reader)?;
        // Define the expected AST structure
        let exp = TreeNode::from_expr(ExprOr(vec![
            Token("num", "1"),
            Token("op", "+"),
            SubExpr(vec![
                Token("op", "("),
                SubExpr(vec![
                    Token("num", "2"),
                    Token("op", "+"),
                    Token("num", "3"),
                ]),
                Token("op", ")"),
            ]),
        ]));
        // Assert that the AST matches the expected structure
        assert_ast!(exp, ast);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::lexer::LineReader;
    use Expression::*;

    fn setup_lexer_ABC<'a>() -> Lexer {
        let mut lexer = Lexer::new();
        let _ = lexer.define("tok:a", "[a-c]+");
        let _ = lexer.define("tok:b", "[d-f]+");
        let _ = lexer.define("tok:c", "[g-i]+");

        lexer
    }

    #[test]
    // #[ignore = "Unimplemented"]
    fn test_get_expr_or() {
        unimplemented!()
    }

    #[test]
    #[ignore = "Unimplemented"]
    fn test_get_sub_expr() {
        unimplemented!()
    }
    
    #[test]
    #[ignore = "Unimplemented"]
    fn test_get_expr() {
        unimplemented!()
    }
    
    #[test]
    #[ignore = "Unimplemented"]
    fn test_get_token() -> Result<(), String> {
        let mut reader = LineReader::new("abc");
        let lexer = setup_lexer_ABC();
        
        let mut parser = Parser::new();
        parser.define("expr", Token("tok:a", ""));

        let ast = parser.parse_tree(&lexer, &mut reader)?;


        Ok(())
    }

    #[test]
    fn test_recursion() -> Result<(), String> {
        // Setup Lexer and Parser
        let mut lexer = Lexer::new();
        lexer.define("tok", "[a-z]+")?;
        lexer.define("op", "\\(|\\)")?;

        let mut parser = Parser::new();
        parser.define("EXPR", ExprOr(vec![
            SubExpr(vec![ Token("op", "("), Expr("EXPR"), Token("op", ")")]),
            Expr("NUM"),
        ]));
        parser.define("NUM", Token("tok", ""));
        
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
