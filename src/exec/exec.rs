

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
    
    fn lambda(&self, name: &str, node: &TreeNode, args: &[u32]) -> StateNode {
        let lambda = match self.definitions.get(name) {
            Some(lambda) => lambda,
            None => return StateNode::RuntimeErr(format!("No lambda found for `{}`", name)),
        };
        lambda(EnvFrame::build_frame(self, node, args))
    }

    pub fn define(&mut self, lambda_type: &str, cb: fn(EnvFrame) -> StateNode) {
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

#[derive(Debug, Clone)]
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

    pub fn as_value(self) -> StateNode {
        match self {
            Self::Node(node) => {
                Self::RuntimeErr("Cannot convert Node to Value".into())
            },
            _ => self
        }
    }

    pub fn as_node_value(self) -> NodeValue {
        match self {
            Self::RuntimeErr(_) => unreachable!(), // should not be trying to convert an error
            Self::Value(val) => val,
            _ => NodeValue::ValueError(format!("Cannot convert `{self:?}` to NodeValue")),
        }
        // NodeValue::ValueError(format!("Cannot convert `{self:?}` to NodeValue"))
    }

    fn operator(lhs: StateNode, rhs: StateNode, op: fn(a: NodeValue, b: NodeValue) -> NodeValue) -> StateNode {
        println!("OPERATOR\nLHS: {lhs:?}\nRHS: {rhs:?}");
        if let Self::RuntimeErr(_) = lhs { return lhs; }
        if let Self::RuntimeErr(_) = rhs { return rhs; }
        // -=-=- //
        match (lhs.as_node_value(), rhs.as_node_value()) {
            (NodeValue::ValueError(err), _) |(_, NodeValue::ValueError(err)) => StateNode::RuntimeErr(err),
            (lhv, rhv) => match op(lhv, rhv) {
                NodeValue::ValueError(err) => Self::RuntimeErr(err),
                val => Self::Value(val),
            }
            _ => unreachable!(), // Should not happen if as_node_value is implemented correctly
        }
    }
}

impl Add for StateNode {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::operator(self, other, |lhs, rhs| lhs + rhs )
    }
}
impl Sub for StateNode {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        println!("{self:?} - {other:?}");
        Self::operator(self, other, |lhs, rhs| lhs - rhs )
    }
}
impl Mul for StateNode {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self::operator(self, other, |lhs, rhs| lhs * rhs )
    }
}
impl Div for StateNode {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self::operator(self, other, |lhs, rhs| lhs / rhs )
    }
}


#[derive(Debug, Clone)]
pub enum NodeValue {
    BigInteger(i128),
    Integer(i32),
    BigFloat(f64),
    Float(f32),
    String(String),

    ValueError(String),
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

impl Add for NodeValue {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        println!("{self:?} + {other:?}");
        Self::ValueError("Cannot add Node Values - unimplemented".into())
    }
}
impl Sub for NodeValue {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        println!("{self:?} - {other:?}");
        Self::ValueError("Cannot sub Node Values - unimplemented".into())
    }
}
impl Mul for NodeValue {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        println!("{self:?} * {other:?}");
        Self::ValueError("Cannot mul Node Values - unimplemented".into())
    }
}
impl Div for NodeValue {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        println!("{self:?} / {other:?}");
        Self::ValueError("Cannot div Node Values - unimplemented".into())
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
        match self.args.len() {
            1 => Exec::UniOp(self.eval_branch(0)),
            2 => Exec::BinOp(self.eval_branch(0), self.eval_branch(1)),
            // 3 => Exec::TriOp(),
            _ => Exec::Root(StateNode::Node(self.node.clone())),
        }
        // Exec::RuntimeErr("EVAL Not Imp[lemsdkjfsdkj".into())
    }

    fn eval_branch(&self, branch: usize) -> StateNode {
        let node = &self.node.nodes[self.args[branch] as usize - 1];
        self.eval_node(node)
    }

    fn eval_node(&self, node: &TreeNode) -> StateNode {
        self.env.eval(node)
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