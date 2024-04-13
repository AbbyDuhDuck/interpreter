

use std::ops::{Add, Div, Mul, Sub};

use crate::parser::syntax::{AbstractSyntaxTree, TreeNode};

use super::syntax::Lambda;



pub enum Value<T> {
    Value(T),
    None,
}

pub struct VirtualEnv {

}

impl VirtualEnv {
    pub fn new() -> VirtualEnv {
        VirtualEnv { }
    }

    pub fn exec(&self, ast: AbstractSyntaxTree) -> StateNode {
        println!("exec: {ast}");
        println!("lambda: {}", ast.root.lambda);
        // println!("nodes: {:}", ast.root.nodes[0]);
        // println!("lambda: {:#?}", ast.root.nodes[0].lambda);
        StateNode::None
    }

    // pub fn define(&mut self, node_type: &str, lambda: Lambda) {

    // }

    pub fn define(&mut self, lambda_type: &str, 
        cb: fn (frame: EnvFrame) -> StateNode
    ) {

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

pub struct EnvFrame {
    node: TreeNode,
}

impl EnvFrame {
    pub fn new(node: TreeNode) -> EnvFrame {
        EnvFrame { node }
    }
    
    pub fn eval(&self) -> Exec {
        Exec::RuntimeErr("Not Imp[lemsdkjfsdkj".into())
    }
    
    pub fn eval_as<T>(&self) -> StateNode {
        StateNode::RuntimeErr("Not Imp[lemsdkjfsdkj".into())
    }

}