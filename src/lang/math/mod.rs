
pub use math::*;

pub mod math {
    use crate::parser::Parser;
    use crate::lexer::Lexer;
    use crate::exec::{Executor, NodeValue, StateNode, VirtualEnv};

    pub fn exec() -> Executor<'static> {
        Executor::new(self::lexer(), self::parser(), self::env())
    }

    pub fn lexer() -> Lexer {
        let mut lexer = Lexer::new();
        let _ = lexer.define("op", "\\+|\\-|\\*|\\/|\\(|\\)");
        let _ = lexer.define("float", "[0-9]+\\.[0-9]+");
        let _ = lexer.define("int", "[0-9]+");
        let _ = lexer.define("assign", "\\:\\=|\\=");
        let _ = lexer.define("ident", "[a-zA-Z_]+");
        lexer
    }

    pub fn parser() -> Parser<'static> {
        use crate::parser::syntax::Expression::*;
        use crate::exec::syntax::Lambda::*;
        let mut parser = Parser::new();
        let _ = parser.define("EXPR", Expr("MATH:EXPR"), Eval);
        let _ = parser.define("EXPR", ExprOr(&[
            Expr("ASSIGN"),
            Expr("MATH:EXPR"),
        ]), Eval);
        let _ = parser.define("ASSIGN", 
            SubExpr(&[Expr("IDENT"), Token("assign", ""), Expr("MATH:EXPR")]),
            Lambda("SET_IDENT", &[1, 3])
        );
        let _ = parser.define("IDENT", Token("ident", ""), EvalToken);
        
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
            Expr("FACTOR"),
        ]), LambdaOr(&[
            Lambda("MULT", &[1, 3]),
            Eval,
        ]));
        let _ = parser.define("FACTOR", ExprOr(&[
            SubExpr(&[ Expr("VALUE"), Token("op", "/"), Expr("FACTOR") ]),
            Expr("VALUE"),
        ]), LambdaOr(&[
            Lambda("DIV", &[1, 3]),
            Eval,
        ]));
        let _ = parser.define("VALUE", ExprOr(&[
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
        let _ = parser.define("VAR", Expr("IDENT"), Lambda("GET_IDENT", &[1]));
        parser
    }

    pub fn env() -> VirtualEnv {
        use crate::exec::syntax::Lambda::*;
        use crate::exec::StateNode::*;
        use crate::exec::Exec;
        let mut env = VirtualEnv::new();
       
        env.define("ADD", |mut frame, | {
            match frame.eval() {
                Exec::BinExpr(lhs, rhs) => lhs + rhs,
                _ => RuntimeErr("Something add".into()),
            }
        });
        env.define("SUB", |mut frame, | {
            match frame.eval() {
                Exec::BinExpr(lhs, rhs) => lhs - rhs,
                _ => RuntimeErr("Something sub".into()),
            }
        });
        env.define("MULT", |mut frame, | {
            match frame.eval() {
                Exec::BinExpr(lhs, rhs) => lhs * rhs,
                _ => RuntimeErr("Something mult".into()),
            }
        });
        env.define("DIV", |mut frame, | {
            match frame.eval() {
                Exec::BinExpr(lhs, rhs) => lhs / rhs,
                _ => RuntimeErr("Something div".into()),
            }
        });
        env.define("INTEGER", |frame, | {
            frame.eval_as::<i32>()
        });
        env.define("FLOAT", |frame, | {
            frame.eval_as::<f32>()
        });
        env.define("GET_IDENT", |mut frame, | {
            match frame.eval() {
                Exec::UniExpr(ident) => {
                    if let NodeValue::Ident(ident) = ident.as_ident() {
                        return frame.get_ident(&ident);
                    }
                    return RuntimeErr(format!("Could not get Identifier `{ident:?}`"));
                },
                _ => RuntimeErr("Something get ident".into()),
            }
        });
        env.define("SET_IDENT", |mut frame, | {
            match frame.eval() {
                Exec::BinExpr(ident, value) => {
                    if let NodeValue::Ident(ident) = ident.as_ident() {
                        frame.set_ident(&ident, value.as_node_value());
                    } else {
                        return RuntimeErr(format!("Could not set Identifier `{ident:?}`"));
                    }
                    StateNode::None
                },
                _ => RuntimeErr("Something set ident".into()),
            }
        });
        env
    }
}

