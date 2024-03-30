use crate::lexer::{Lexer, ReadPointer, Reader, Token};

use super::Parser;
// use super::macros;



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
        // println!("TRY - {self:?}");
        let result = match self {
            Expression::ExprOr(expr) => self.get_expr_or(lexer, parser, reader, expr),
            Expression::SubExpr(expr) => self.get_sub_expr(lexer, parser, reader, expr),
            Expression::Expr(expr) => self.get_expr(lexer, parser, reader, expr),
            Expression::Token(token, value) => self.get_token(lexer, reader, token, value),
        };
        // println!("FOUND ({}) - {}", self.get_name(), match &result {
        //     Ok(node) => format!("{node}"),
        //     Err(err) => err.into()
        // });
        result
    }

    fn _get_name(&self) -> &'static str {
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

    pub fn get_sub_expr<T>(&self, lexer: &Lexer, parser: &Parser, reader: &mut T, expr: &Vec<Expression>) -> Result<TreeNode, String>
    where
        T: Reader,
    {
        Ok(TreeNode::from_nodes(
            expr.iter()
            .map(|subexpr| subexpr.get(lexer, parser, reader))
            .collect::<Result<Vec<TreeNode>, String>>()?
        ))
    }

    pub fn get_expr<T>(&self, lexer: &Lexer, parser: &Parser, reader: &mut T, expr: &str) -> Result<TreeNode, String>
    where
        T: Reader,
    {
        parser.get_expr(expr)?.get(lexer, parser, reader)
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



#[cfg(test)]
mod tests {
    use super::*;
    // use super::macros;

    use crate::lexer::LineReader;
    use Expression::*;

    fn setup_lexer_abc<'a>() -> Lexer {
        let mut lexer = Lexer::new();
        let _ = lexer.define("tok:a", "[a-c]+");
        let _ = lexer.define("tok:b", "[d-f]+");
        let _ = lexer.define("tok:c", "[g-i]+");

        lexer
    }

    #[test]
    #[ignore = "Unimplemented"]
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
        let lexer = setup_lexer_abc();
        
        let mut parser = Parser::new();
        parser.define("expr", Token("tok:a", ""));

        let _ast = parser.parse_tree(&lexer, &mut reader)?;


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
