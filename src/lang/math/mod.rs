

use crate::lexer;
use crate::parser::{self, syntax};

pub use math::*;

pub mod math {
    use super::parser::Parser;
    use super::lexer::Lexer;
    
    use super::syntax::Expression::*;
    use once_cell::sync::Lazy;

    pub const LEXER: Lazy<Lexer> = Lazy::new(|| {
        let mut lexer = Lexer::new();
        let _ = lexer.define("op", "\\+|\\-|\\*|\\/|\\(|\\)");
        let _ = lexer.define("float", "[0-9]+\\.[0-9]+");
        let _ = lexer.define("int", "[0-9]+");
        let _ = lexer.define("ident", "[a-zA-Z_]+");
        lexer
    });

    pub const PARSER: Lazy<Parser> = Lazy::new(|| {
        let mut parser = Parser::new();
        let _ = parser.define("EXPR", Expr("MATH:EXPR"));
        let _ = parser.define("MATH:EXPR", ExprOr(vec![
            SubExpr(vec![ Expr("TERM"), Token("op", "+"), Expr("MATH:EXPR") ]),
            SubExpr(vec![ Expr("TERM"), Token("op", "-"), Expr("MATH:EXPR") ]),
            Expr("TERM"),
        ]));
        let _ = parser.define("TERM", ExprOr(vec![
            SubExpr(vec![ Expr("FACTOR"), Token("op", "*"), Expr("TERM") ]),
            SubExpr(vec![ Expr("FACTOR"), Token("op", "/"), Expr("TERM") ]),
            Expr("FACTOR"),
        ]));
        let _ = parser.define("FACTOR", ExprOr(vec![
            SubExpr(vec![ Token("op", "("), Expr("VAR"), Token("op", ")")]),
            SubExpr(vec![ Token("op", "("), Expr("MATH:EXPR"), Token("op", ")")]),
            Expr("NUM"),
            Expr("VAR"),
        ]));
        let _ = parser.define("NUM", ExprOr(vec![
            Token("float", ""),
            Token("int", ""),
        ]));
        let _ = parser.define("VAR", Token("ident", ""));
        parser
    });
}

