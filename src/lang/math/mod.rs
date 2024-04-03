
pub use math::*;

pub mod math {
    use crate::parser::Parser;
    use crate::lexer::Lexer;
    use crate::exec::VirtualEnv;

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
        use crate::parser::syntax::Expression::*;
        use crate::exec::syntax::Lambda::*;
        let mut parser = Parser::new();
        let _ = parser.define("EXPR", Expr("MATH:EXPR"), Eval);
        let _ = parser.define("MATH:EXPR", ExprOr(&[
            SubExpr(&[ Expr("TERM"), Token("op", "+"), Expr("MATH:EXPR") ]),
            SubExpr(&[ Expr("TERM"), Token("op", "-"), Expr("MATH:EXPR") ]),
            Expr("TERM"),
        ]), LambdaOr(&[
            Lambda("ADD", &[1, 3]),
            Lambda("SUB", &[1, 3]),
            Eval,
        ]));
        let _ = parser.define("TERM", ExprOr(&[
            SubExpr(&[ Expr("FACTOR"), Token("op", "*"), Expr("TERM") ]),
            SubExpr(&[ Expr("FACTOR"), Token("op", "/"), Expr("TERM") ]),
            Expr("FACTOR"),
        ]), LambdaOr(&[
            Lambda("MULT", &[1, 3]),
            Lambda("DIV", &[1, 3]),
            Eval,
        ]));
        let _ = parser.define("FACTOR", ExprOr(&[
            SubExpr(&[ Token("op", "("), Expr("MATH:EXPR"), Token("op", ")")]),
            Expr("NUM"),
            Expr("VAR"),
        ]), LambdaOr(&[
            GetExpr(2, &Eval),
            Eval,
            Eval,
        ]));
        let _ = parser.define("NUM", ExprOr(&[
            Token("float", ""),
            Token("int", ""),
        ]), LambdaOr(&[
            EvalAs("FLOAT"),
            EvalAs("INTEGER"),
        ]));
        let _ = parser.define("VAR", Token("ident", ""), EvalAs("GET_IDENT"));
        parser
    });

    pub const ENV: Lazy<VirtualEnv> = Lazy::new(|| {
        use crate::exec::syntax::Lambda::*;
        use crate::exec::StateNode::*;
        use crate::exec::Exec;
        let mut env = VirtualEnv::new();
       
        env.define("ADD", |frame, | {
            match frame.eval() {
                Exec::BinOp(lhs, rhs) => lhs + rhs,
                _ => RuntimeErr("Something".into()),
            }
        });
        env.define("SUB", |frame, | {
            None
        });
        env.define("MULT", |frame, | {
            None
        });
        env.define("DIV", |frame, | {
            None
        });
        env.define("INTEGER", |frame, | {
            frame.eval_as::<i32>()
        });
        env.define("FLOAT", |frame, | {
            frame.eval_as::<f32>()
        });
        env.define("GET_IDENT", |frame, | {
            RuntimeErr("tthign".into())
        });
        env
    });
}

