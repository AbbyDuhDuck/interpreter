
#[derive(Debug)]
pub struct TreeNode {

}

impl TreeNode {
    pub fn new() -> TreeNode {
        TreeNode {}
    }
}

#[derive(Debug)]
pub struct AbstractSyntaxTree {
    root: TreeNode,
}

impl AbstractSyntaxTree {
    pub fn new(root: TreeNode) -> Self {
        AbstractSyntaxTree { root }
    }
}