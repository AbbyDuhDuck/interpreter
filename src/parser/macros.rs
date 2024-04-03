//! # AST Assertion Macros
//! 
//! This is where the [`assert_ast`] and [`assert_ast_ne`] macros are defined and tested.
//! 


// -=-=-=-=- Testing Macros -=-=-=-=- //

/// Asserts that two [Abstract Syntax Trees](crate::parser::syntax::AbstractSyntaxTree) 
/// are equal to each other (using [`PartialEq`]).
///
/// On panic, this macro will print the values of the expressions with their debug representations.
///
/// Like [`assert`], this macro has a second form, where a custom panic message can be provided
#[macro_export]
macro_rules! assert_ast {
    ( $ast:expr, $expected:expr ) => {
        {
            let ast_str = format!("{}", $ast);
            let expected_str = format!("{}", $expected);
            assert_eq!(ast_str, expected_str, "When trying to match ASTs:");
        }
    };
    ( $ast:expr, $expected:expr, $($arg:tt)+ ) => {
        {
            let ast_str = format!("{}", $ast);
            let expected_str = format!("{}", $expected);
            assert_eq!(ast_str, expected_str, "When trying to match ASTs: {}", format!($($arg)+));
        }
    };
}

/// Asserts that two expressions are equal to each other (using [`PartialEq`]).
///
/// On panic, this macro will print the values of the expressions with their debug representations.
///
/// Like [`assert`], this macro has a second form, where a custom panic message can be provided
#[macro_export]
macro_rules! assert_ast_ne {
    ( $ast:expr, $expected:expr ) => {
        {
            let ast_str = format!("{}", $ast);
            let expected_str = format!("{}", $expected);
            assert_ne!(ast_str, expected_str, "Both are equal when trying to match ASTs:");
        }
    };
    ( $ast:expr, $expected:expr, $($arg:tt)+ ) => {
        {
            let ast_str = format!("{}", $ast);
            let expected_str = format!("{}", $expected);
            let msg = format!($($arg)+);
            assert_ne!(ast_str, expected_str, "Both are equal when trying to match ASTs: {}", msg);
        }
    };
}

// -=-=-=-=- Unit Tests -=-=-=-=- //

/// Test that [`assert_ast`] and [`assert_ast_ne`] are working properly.
#[cfg(test)]
mod tests {
    // lexer
    use crate::lexer::Lexer;
    use crate::lexer::LineReader;
    // parser
    use crate::parser::Parser;
    use crate::parser::syntax::TreeNode;
    use crate::parser::syntax::Expression::*;
    use crate::exec::syntax::Lambda::Eval;
    
    /// assert two [tokens](Token) can be matched with [`assert_ast`].
    #[test]
    fn assert_ast_token() -> Result<(), String> {
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

    /// assert two [tokens](Token) are different with [`assert_ast_ne`].
    #[test]
    fn assert_ast_ne_macro_token() -> Result<(), String> {
        // Setup Lexer and Parser
        let mut lexer = Lexer::new();
        lexer.define("tok:a", "[a-c]+")?;
        let mut parser = Parser::new();
        parser.define("EXPR", Token("tok:a", ""), Eval);
        let mut reader = LineReader::new("abc");
        // Parse an expression
        let ast = parser.parse_tree(&lexer, &mut reader)?;
        // Define the expected AST structure
        let exp = TreeNode::from_token(Token("tok:a", "def").token());
        // Assert that the AST doesn't match the expected structure
        assert_ast_ne!(exp, ast);
        Ok(())
    }

    /// assert an [`ExprOr`] expression.
    #[test]
    fn assert_ast_expr_or() -> Result<(), String> {
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
    
    /// assert an [`ExprOr`] expression that contains a [`SubExpr`].
    #[test]
    fn assert_ast_expr_or_sub_expr() -> Result<(), String> {
        // Setup Lexer
        let mut lexer = Lexer::new();
        lexer.define("tok:a", "[a-c]+")?;
        lexer.define("tok:b", "[d-f]+")?;
        lexer.define("tok:c", "[g-i]+")?;
        // Setup Parser
        let mut parser = Parser::new();
        parser.define("EXPR", ExprOr(&[
            SubExpr(&[Token("tok:a", ""), Token("tok:b", ""),]),
            Token("tok:a", ""),
            Token("tok:c", ""),
        ]), Eval);
        // Setup Reader
        let mut reader = LineReader::new("ghiabcdefabcghi");
        // Parse an expression
        let ast_1 = parser.parse_tree(&lexer, &mut reader)?;
        let ast_2 = parser.parse_tree(&lexer, &mut reader)?;
        let ast_3 = parser.parse_tree(&lexer, &mut reader)?;
        let ast_4 = parser.parse_tree(&lexer, &mut reader)?;
        // Define the expected AST structure
        let exp_1 = TreeNode::from_token(Token("tok:c", "ghi").token());
        let exp_2 = TreeNode::from_expr(&SubExpr(&[
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

    /// assert an [`Expr`] expression.
    #[test]
    fn assert_ast_expr() -> Result<(), String> {
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

    /// assert a [`SubExpr`] expression without recursion.
    #[test]
    fn assert_ast_sub_expr() -> Result<(), String> {
        // Setup Lexer
        let mut lexer = Lexer::new();
        lexer.define("num", "[0-9]+")?;
        lexer.define("op", "\\+|\\(|\\)")?;
        // Expressions
        let mut parser = Parser::new();
        parser.define("EXPR", ExprOr(&[
            SubExpr(&[ Expr("VAL"), Token("op", "+"), Expr("EXPR") ]),
            Expr("VAL"),
        ]), Eval);
        parser.define("VAL", ExprOr(&[
            SubExpr(&[ Token("op", "("), Expr("EXPR"), Token("op", ")") ]),
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

    /// assert a [`SubExpr`] expression with recursion.
    #[test]
    fn assert_ast_sub_expr_sub_expr() -> Result<(), String> {
        // Setup Lexer
        let mut lexer = Lexer::new();
        lexer.define("num", "[0-9]+")?;
        lexer.define("op", "\\+|\\(|\\)")?;
        // Expressions
        let mut parser = Parser::new();
        parser.define("EXPR", ExprOr(&[
            SubExpr(&[ Expr("VAL"), Token("op", "+"), Expr("EXPR") ]),
            Expr("VAL"),
        ]), Eval);
        parser.define("VAL", ExprOr(&[
            SubExpr(&[ Token("op", "("), Expr("EXPR"), Token("op", ")") ]),
            Expr("NUM"),
        ]), Eval);
        parser.define("NUM", Token("num", ""), Eval);        
        // Setup Parser
        let mut reader = LineReader::new("1+(2+3)");
        // Parse an expression
        let ast = parser.parse_tree(&lexer, &mut reader)?;
        // Define the expected AST structure
        let exp = TreeNode::from_expr(&ExprOr(&[
            Token("num", "1"),
            Token("op", "+"),
            SubExpr(&[
                Token("op", "("),
                SubExpr(&[
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