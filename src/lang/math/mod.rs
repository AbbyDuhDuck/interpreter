

use crate::lexer;
use crate::parser::{self, syntax};

pub use math::*;

pub mod math {
    use std::collections::HashMap;

    use super::parser::Parser;

    use super::lexer::Lexer;

    use super::syntax::Expression::*;
    use super::syntax::Expression;
    use once_cell::sync::Lazy;

    pub const LEXER: Lazy<Lexer> = Lazy::new(|| {
        let mut lexer = Lexer::new();
        let _ = lexer.define("op", "\\+|\\-|\\*|\\/");
        let _ = lexer.define("num", "[0-9]+");
        let _ = lexer.define("ident", "[a-zA-Z_]+");
        lexer
    });

    pub const PARSER: Lazy<Parser> = Lazy::new(|| {
        let mut parser = Parser::new();
        let _ = parser.define("EXPR", EXPR.to_owned());
        let _ = parser.define("TERM", TERM.to_owned());
        let _ = parser.define("FACTOR", FACTOR.to_owned());
        let _ = parser.define("NUM", NUM.to_owned());
        let _ = parser.define("VAR", VAR.to_owned());
        parser
    });

    // Lazily initialize the hashmap
    pub static EXPRESSIONS: Lazy<HashMap<&'static str, &Lazy<Expression<'static>>>> = Lazy::new(|| {
        let mut expressions = HashMap::new();
        expressions.insert("EXPR", &EXPR);
        expressions.insert("TERM", &TERM);
        expressions.insert("FACTOR", &FACTOR);
        expressions.insert("NUM", &NUM);
        expressions.insert("VAR", &VAR);
        expressions
    });

    pub static EXPR: Lazy<Expression> = Lazy::new(|| SubExpr(vec![
        SubExpr(vec![ Expr("TERM"), Token("op:+"), Expr("EXPR") ]),
        SubExpr(vec![ Expr("TERM"), Token("op:-"), Expr("EXPR") ]),
        Expr("TERM"),
    ]));
    
    pub static TERM: Lazy<Expression> = Lazy::new(|| SubExpr(vec![
        SubExpr(vec![ Expr("FACTOR"), Token("op:*"), Expr("TERM") ]),
        SubExpr(vec![ Expr("FACTOR"), Token("op:/"), Expr("TERM") ]),
        Expr("FACTOR"),
    ]));
    
    pub static FACTOR: Lazy<Expression> = Lazy::new(|| SubExpr(vec![
        SubExpr(vec![ Token("op:("), Expr("EXPR"), Token("op:)")]),
        Expr("NUM"),
        Expr("VAR"),
    ]));
    
    pub static NUM: Lazy<Expression> = Lazy::new(|| SubExpr(vec![
        Token("int:"),
        Token("float:"),
    ]));
    
    pub static VAR: Lazy<Expression> = Lazy::new(|| Token("ident:"));
}

