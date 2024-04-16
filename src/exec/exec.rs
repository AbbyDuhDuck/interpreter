

use std::collections::HashMap;
use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;
use std::{error, u32};

use crate::parser::syntax::{AbstractSyntaxTree, TreeNode};
use crate::exec::syntax::OwnedLambda;

use super::syntax::Lambda;


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
        self.eval_node(&ast.root)
    }

    fn eval_node(&self, node: &TreeNode) -> StateNode {
        let lambda = &node.lambda;
        println!("EVAL: {node} {lambda}");
        println!("{lambda:?}");

        self.eval_lambda(node, lambda)
    }

    fn eval_lambda(&self, node: &TreeNode, lambda: &OwnedLambda) -> StateNode {
        use OwnedLambda::*;
        match lambda {
            Eval => self.eval(node),
            Lambda(name, args) => self.lambda(name, node, args),
            EvalAs(name) => self.lambda(name, node, &[]),
            GetExpr(arg, sublambda) => match node.nodes.get(*arg as usize - 1) {
                Some(subnode) => self.eval_lambda(subnode, sublambda),
                None => StateNode::RuntimeErr(format!("No node found for index {arg} on node `{node}`")),
            }
            _ => StateNode::RuntimeErr(format!("No lambda eval found for `{lambda:?}`")),
        }

        
    }

    fn eval(&self, node: &TreeNode) -> StateNode {
        if let OwnedLambda::Eval = &node.lambda {
            return StateNode::RuntimeErr(format!("Recursion Error: Cannot EVAL on node with EVAL lambda `{node}`"));
        }
        self.eval_node(node)
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
    // Types
    BigFloat(f64),
    Float(f32),
    BigInteger(i128),
    Integer(i32),
    String(String),
    // Errors
    ValueError(String),
}

impl NodeValue {

    pub fn to_string(&self) -> Result<String, String> {
        match self {
            Self::BigFloat(float) => Ok(float.to_string()),
            Self::Float(float) => Ok(float.to_string()),
            Self::BigInteger(int) => Ok(int.to_string()),
            Self::Integer(int) => Ok(int.to_string()),
            Self::String(string) => Ok(string.into()),

            Self::ValueError(err) => Err(err.into()),
        }
    }

    fn parse_value<T>(value: &str) -> Result<Self, String>
    where
        T: FromStr + NodeTypeTrait,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        match T::VARIANT {
            NodeType::Integer => value
                .parse::<i32>()
                .map(NodeValue::Integer)
                .map_err(|_| format!("Failed to parse '{value}' as Integer")),
            NodeType::BigInteger => value
                .parse::<i128>()
                .map(NodeValue::BigInteger)
                .map_err(|_| format!("Failed to parse '{value}' as Integer")),
            NodeType::Float => value
                .parse::<f32>()
                .map(NodeValue::Float)
                .map_err(|_| format!("Failed to parse '{value}' as Float")),
            NodeType::BigFloat => value
                .parse::<f64>()
                .map(NodeValue::BigFloat)
                .map_err(|_| format!("Failed to parse '{value}' as Float")),
            NodeType::String => Ok(NodeValue::String(value.to_string())),
        }
    }

    fn as_type<T>(&self) -> NodeValue
    where
        T: FromStr + NodeTypeTrait,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        match Self::parse_value::<T>(&match self.to_string() {
            Ok(val) => val,
            Err(err) => return Self::ValueError(err),
        }) {
            Ok(val) => val,
            Err(err) => Self::ValueError(err)
        }
    }
}

impl Add for NodeValue {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        println!("{self:?} + {other:?}");

        // TODO: obfuscate out this to multiple functions somehow...
        match (&self, &other) {
            (Self::ValueError(err), _) |
            (_, Self::ValueError(err)) => Self::ValueError(err.into()),

            (Self::BigFloat(f1), Self::BigFloat(f2)) => Self::BigFloat(f1 + f2),
            (Self::Float(f1), Self::Float(f2)) => Self::Float(f1 + f2),
            (Self::BigInteger(i1), Self::BigInteger(i2)) => Self::BigInteger(i1 + i2),
            (Self::Integer(i1), Self::Integer(i2)) => Self::Integer(i1 + i2),

            (Self::BigFloat(_), _) | (_, Self::BigFloat(_)) => self.as_type::<f64>() + other.as_type::<f64>(),
            (Self::Float(_), _) | (_, Self::Float(_)) => self.as_type::<f32>() + other.as_type::<f32>(),
            (Self::BigInteger(_), _) | (_, Self::BigInteger(_)) => self.as_type::<i128>() + other.as_type::<i128>(),
            (Self::Integer(_), _) | (_, Self::Integer(_)) => self.as_type::<i32>() + other.as_type::<i32>(),

            (lhs, rhs) => Self::ValueError(format!("Cannot add {lhs:?} to {rhs:?}."))
        }
    }
}
impl Sub for NodeValue {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        println!("{self:?} - {other:?}");
        
        // TODO: obfuscate out this to multiple functions somehow...
        match (&self, &other) {
            (Self::ValueError(err), _) |
            (_, Self::ValueError(err)) => Self::ValueError(err.into()),

            (Self::BigFloat(f1), Self::BigFloat(f2)) => Self::BigFloat(f1 - f2),
            (Self::Float(f1), Self::Float(f2)) => Self::Float(f1 - f2),
            (Self::BigInteger(i1), Self::BigInteger(i2)) => Self::BigInteger(i1 - i2),
            (Self::Integer(i1), Self::Integer(i2)) => Self::Integer(i1 - i2),

            (Self::BigFloat(_), _) | (_, Self::BigFloat(_)) => self.as_type::<f64>() - other.as_type::<f64>(),
            (Self::Float(_), _) | (_, Self::Float(_)) => self.as_type::<f32>() - other.as_type::<f32>(),
            (Self::BigInteger(_), _) | (_, Self::BigInteger(_)) => self.as_type::<i128>() - other.as_type::<i128>(),
            (Self::Integer(_), _) | (_, Self::Integer(_)) => self.as_type::<i32>() - other.as_type::<i32>(),

            (lhs, rhs) => Self::ValueError(format!("Cannot add {lhs:?} to {rhs:?}."))
        }
    }
}
impl Mul for NodeValue {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        println!("{self:?} * {other:?}");

        // TODO: obfuscate out this to multiple functions somehow...
        match (&self, &other) {
            (Self::ValueError(err), _) |
            (_, Self::ValueError(err)) => Self::ValueError(err.into()),

            (Self::BigFloat(f1), Self::BigFloat(f2)) => Self::BigFloat(f1 * f2),
            (Self::Float(f1), Self::Float(f2)) => Self::Float(f1 * f2),
            (Self::BigInteger(i1), Self::BigInteger(i2)) => Self::BigInteger(i1 * i2),
            (Self::Integer(i1), Self::Integer(i2)) => Self::Integer(i1 * i2),

            (Self::BigFloat(_), _) | (_, Self::BigFloat(_)) => self.as_type::<f64>() * other.as_type::<f64>(),
            (Self::Float(_), _) | (_, Self::Float(_)) => self.as_type::<f32>() * other.as_type::<f32>(),
            (Self::BigInteger(_), _) | (_, Self::BigInteger(_)) => self.as_type::<i128>() * other.as_type::<i128>(),
            (Self::Integer(_), _) | (_, Self::Integer(_)) => self.as_type::<i32>() * other.as_type::<i32>(),

            (lhs, rhs) => Self::ValueError(format!("Cannot add {lhs:?} to {rhs:?}."))
        }
    }
}
impl Div for NodeValue {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        println!("{self:?} / {other:?}");

        if let Some(_) = match other {
            Self::BigFloat(float) if float == 0.0 => Some(()),
            Self::Float(float) if float == 0.0 => Some(()),
            Self::BigInteger(int) if int == 0 => Some(()),
            Self::Integer(int) if int == 0 => Some(()),
            _ => None,
        }{
            return Self::ValueError("Cannot divide by zero".into())
        };
        
        // TODO: obfuscate out this to multiple functions somehow...
        match (&self, &other) {
            // error check
            (Self::ValueError(err), _) |
            (_, Self::ValueError(err)) => Self::ValueError(err.into()),

            // division
            (Self::BigFloat(f1), Self::BigFloat(f2)) => Self::BigFloat(f1 / f2),
            (Self::Float(f1), Self::Float(f2)) => Self::Float(f1 / f2),
            (Self::BigInteger(i1), Self::BigInteger(i2)) => {
                if i1 % i2 != 0 { return Self::BigFloat(*i1 as f64 / *i2 as f64); }
                Self::BigInteger(i1 / i2)
            },
            (Self::Integer(i1), Self::Integer(i2)) => {
                if i1 % i2 != 0 { return Self::Float(*i1 as f32 / *i2 as f32); }
                Self::Integer(i1 / i2)
            }

            // conversion
            (Self::BigFloat(_), _) | (_, Self::BigFloat(_)) => self.as_type::<f64>() / other.as_type::<f64>(),
            (Self::Float(_), _) | (_, Self::Float(_)) => self.as_type::<f32>() / other.as_type::<f32>(),
            (Self::BigInteger(_), _) | (_, Self::BigInteger(_)) => self.as_type::<i128>() / other.as_type::<i128>(),
            (Self::Integer(_), _) | (_, Self::Integer(_)) => self.as_type::<i32>() / other.as_type::<i32>(),

            // value error
            (lhs, rhs) => Self::ValueError(format!("Cannot add {lhs:?} to {rhs:?}."))
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
        self.env.eval_node(node)
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