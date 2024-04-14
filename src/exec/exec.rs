

use std::collections::HashMap;
use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;
use std::u32;

use crate::parser::syntax::{AbstractSyntaxTree, TreeNode};
use crate::exec::syntax::OwnedLambda;

use super::syntax::Lambda;



pub enum Value<T> {
    Value(T),
    None,
}

pub struct VirtualEnv {
    definitions: HashMap<String, fn(EnvFrame) -> StateNode>,
}

impl VirtualEnv {
    pub fn new() -> VirtualEnv {
        VirtualEnv { definitions: HashMap::new() }
    }

    pub fn exec(&self, ast: AbstractSyntaxTree) -> StateNode {
        println!("exec: {ast}");
        // println!("lambda: {}", ast.root.lambda);
        // println!("nodes: {:}", ast.root.nodes[0]);
        // println!("lambda: {:#?}", ast.root.nodes[0].lambda);
        self.eval(&ast.root)
    }

    fn eval(&self, node: &TreeNode) -> StateNode {
        let lambda = &node.lambda;
        println!("EVAL: {node} {lambda}");
        println!("{lambda:?}");

        use OwnedLambda::*;
        match lambda {
            Lambda(name, args) => self.lambda(name, node, args),
            EvalAs(name) => self.lambda(name, node, &[]),
            _ => StateNode::RuntimeErr(format!("No lambda eval found for `{lambda:?}`")),
        }

        
    }
    
    fn lambda<'a>(&self, name: &str, node: &'a TreeNode, args: &'a [u32]) -> StateNode {
        let lambda = match self.definitions.get(name) {
            Some(lambda) => lambda,
            None => return StateNode::RuntimeErr(format!("No lambda found for `{}`", name)),
        };
        lambda(EnvFrame::build_frame(self, node, args))
    }

    pub fn define<'a>(&mut self, lambda_type: &str, cb: for<'b> fn(EnvFrame<'b>) -> StateNode) {
        self.definitions.insert(lambda_type.into(), cb);
    }
}


pub enum Exec {
    NoOp(),
    UniOp( StateNode ),
    BinOp( StateNode, StateNode ),
    TriOp( StateNode, StateNode, StateNode ),
    Root( StateNode ),
    RuntimeErr(String)
}

impl Exec {
    pub fn new(node: TreeNode) -> Exec {
        Self::Root( StateNode::new(node) )
    }
}

#[derive(Debug)]
pub enum StateNode {
    None,
    Node(TreeNode),
    Value(NodeValue),
    RuntimeErr(String)
}

impl StateNode {
    pub fn new(node: TreeNode) -> StateNode {
        Self::Node(node)
    }
}

impl Add for StateNode {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}
impl Sub for StateNode {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}
impl Mul for StateNode {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}
impl Div for StateNode {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        todo!()
    }
}


#[derive(Debug)]
pub enum NodeValue {
    BigInteger(i128),
    Integer(i32),
    BigFloat(f64),
    Float(f32),
    String(String),
}

impl NodeValue {
    fn parse_value<T>(value: &str) -> Result<Self, String>
    where
        T: FromStr + NodeTypeTrait,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        match T::VARIANT {
            NodeType::Integer => value
                .parse::<i32>()
                .map(NodeValue::Integer)
                .map_err(|e| format!("Failed to parse '{}' as Integer: {:?}", value, e)),
            NodeType::BigInteger => value
                .parse::<i128>()
                .map(NodeValue::BigInteger)
                .map_err(|e| format!("Failed to parse '{}' as Integer: {:?}", value, e)),
            NodeType::Float => value
                .parse::<f32>()
                .map(NodeValue::Float)
                .map_err(|e| format!("Failed to parse '{}' as Float: {:?}", value, e)),
            NodeType::BigFloat => value
                .parse::<f64>()
                .map(NodeValue::BigFloat)
                .map_err(|e| format!("Failed to parse '{}' as Float: {:?}", value, e)),
            NodeType::String => Ok(NodeValue::String(value.to_string())),
        }
    }
}

#[derive(Debug)]
pub enum NodeType {
    BigInteger,
    Integer,
    BigFloat,
    Float,
    String,
}

pub trait NodeTypeTrait {
    const VARIANT: NodeType;
}
impl NodeTypeTrait for i32 {
    const VARIANT: NodeType = NodeType::Integer;
}
impl NodeTypeTrait for i128 {
    const VARIANT: NodeType = NodeType::BigInteger;
}
impl NodeTypeTrait for f32 {
    const VARIANT: NodeType = NodeType::Float;
}
impl NodeTypeTrait for f64 {
    const VARIANT: NodeType = NodeType::BigFloat;
}
impl NodeTypeTrait for String {
    const VARIANT: NodeType = NodeType::String;
}
impl NodeTypeTrait for str {
    const VARIANT: NodeType = NodeType::String;
}

pub struct EnvFrame<'a> {
    env: &'a VirtualEnv,
    node: &'a TreeNode,
    args: &'a [u32],
}

impl EnvFrame<'_> {
    pub fn build_frame<'a>(env: &'a VirtualEnv, node: &'a TreeNode, args: &'a [u32]) -> EnvFrame<'a> {
        EnvFrame { env, node, args }
    }
    
    pub fn eval(&self) -> Exec {
        Exec::RuntimeErr("EVAL Not Imp[lemsdkjfsdkj".into())
    }
    
    pub fn eval_as<T>(&self) -> StateNode
    where
        T: FromStr + NodeTypeTrait,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        let value = match &self.node.leaf {
            Some(token) => &token.value,
            None => return StateNode::RuntimeErr("EVAL_AS called on branch node, leaf node expected.".into()),
        };

        match NodeValue::parse_value::<T>(value) {
            Ok(parsed_value) => StateNode::Value(parsed_value),
            Err(err) => StateNode::RuntimeErr(err),
        }
    }

}