
// -=-=-=-=- Testing Macros -=-=-=-=- //

#[macro_export]
macro_rules! assert_ast {
    ( $ast:expr, $expected:expr ) => {
        {
            let ast_str = format!("{}", $ast);
            let expected_str = format!("{}", $expected);
            assert_eq!(ast_str, expected_str, "Expected AST: {}, Actual AST: {}", expected_str, ast_str);
        }
    };
}
#[macro_export]
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
mod tests {
    use crate::lexer::Lexer;
    use crate::lexer::LineReader;
    use crate::parser::Parser;
    use crate::parser::syntax::TreeNode;
    use crate::parser::syntax::Expression::*;
    
    #[test]
    fn assert_ast_token() -> Result<(), String> {
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
    fn assert_ast_ne_macro_token() -> Result<(), String> {
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
    fn assert_ast_expr_or() -> Result<(), String> {
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
    fn assert_ast_expr_or_sub_expr() -> Result<(), String> {
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
    fn assert_ast_expr() -> Result<(), String> {
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
    fn assert_ast_sub_expr() -> Result<(), String> {
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
            SubExpr(vec![ Token("op", "("), Expr("EXPR"), Token("op", ")") ]),
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

    #[test]
    fn assert_ast_sub_expr_sub_expr() -> Result<(), String> {
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
            SubExpr(vec![ Token("op", "("), Expr("EXPR"), Token("op", ")") ]),
            Expr("NUM"),
        ]));
        parser.define("NUM", Token("num", ""));        
        // Setup Parser
        let mut reader = LineReader::new("1+(2+3)");
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