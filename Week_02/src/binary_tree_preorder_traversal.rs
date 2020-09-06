// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::rc::Rc;
use std::cell::RefCell;

struct Solution;

impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn helper(node: &Option<Rc<RefCell<TreeNode>>>, trace: &mut Vec<i32>) {
            if let Some(tree_node) = node {
                trace.push(tree_node.borrow().val);
                helper(&tree_node.borrow().left, trace);
                helper(&tree_node.borrow().right, trace);
            }
        }
        let mut result = Vec::new();
        helper(&root, &mut result);
        result
    }
}

struct Solution2;

impl Solution2 {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let (mut result, mut stack) = (vec![], vec![root]);
        while let Some(last) = stack.pop() {
            if let Some(n) = last {
                result.push(n.borrow().val);
                stack.push(n.borrow().right.clone());
                stack.push(n.borrow().left.clone());
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case() {}
}