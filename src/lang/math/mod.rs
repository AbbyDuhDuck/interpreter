

use crate::lexer;
use crate::parser::{self, syntax};

pub use math::*;

pub mod math {
    use super::parser::Parser;
    use super::lexer::Lexer;

    use super::syntax::Expression;
    use super::syntax::Expression::*;
    use once_cell::sync::Lazy;

    pub const LEXER: Lazy<Lexer> = Lazy::new(|| {
        let mut lexer = Lexer::new();
        let _ = lexer.define("op", "\\+|\\-|\\*|\\/|\\(|\\)");
        // let _ = lexer.define("float", "[0-9]+\\.[0-9]+");
        let _ = lexer.define("int", "[0-9]+");
        let _ = lexer.define("ident", "[a-zA-Z_]+");
        lexer
    });

    pub const PARSER: Lazy<Parser> = Lazy::new(|| {
        let mut parser = Parser::new();
        let _ = parser.define("EXPR", EXPR.to_owned());
        let _ = parser.define("MATH:EXPR", MATH_EXPR.to_owned());
        let _ = parser.define("TERM", TERM.to_owned());
        let _ = parser.define("FACTOR", FACTOR.to_owned());
        let _ = parser.define("NUM", NUM.to_owned());
        let _ = parser.define("VAR", VAR.to_owned());
        parser
    });
    
    pub static EXPR: Expression = Expr("MATH:EXPR");

    pub static MATH_EXPR: Lazy<Expression> = Lazy::new(|| ExprOr(vec![
        SubExpr(vec![ Expr("TERM"), Token("op", "+"), Expr("MATH:EXPR") ]),
        SubExpr(vec![ Expr("TERM"), Token("op", "-"), Expr("MATH:EXPR") ]),
        Expr("TERM"),
    ]));
    
    pub static TERM: Lazy<Expression> = Lazy::new(|| ExprOr(vec![
        SubExpr(vec![ Expr("FACTOR"), Token("op", "*"), Expr("TERM") ]),
        SubExpr(vec![ Expr("FACTOR"), Token("op", "/"), Expr("TERM") ]),
        Expr("FACTOR"),
    ]));
    
    pub static FACTOR: Lazy<Expression> = Lazy::new(|| ExprOr(vec![
        SubExpr(vec![ Token("op", "("), Expr("VAR"), Token("op", ")")]),
        SubExpr(vec![ Token("op", "("), Expr("MATH:EXPR"), Token("op", ")")]),
        Expr("NUM"),
        Expr("VAR"),
    ]));
    
    pub static NUM: Lazy<Expression> = Lazy::new(|| ExprOr(vec![
        // Token("float", ""),
        Token("int", ""),
    ]));
    
    pub static VAR: Expression = Token("ident", "");
}

