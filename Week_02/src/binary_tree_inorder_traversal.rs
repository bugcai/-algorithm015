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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn helper(node: &Option<Rc<RefCell<TreeNode>>>, trace: &mut Vec<i32>) {
            if let Some(rc_node) = node {
                helper(&rc_node.borrow().left, trace);
                trace.push(rc_node.borrow().val);
                helper(&rc_node.borrow().right, trace);
            }
        }
        let mut trace = vec![];
        helper(&root, &mut trace);
        trace
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case() {}
}