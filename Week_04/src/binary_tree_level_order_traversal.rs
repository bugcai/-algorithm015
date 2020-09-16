struct Solution;

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

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut q = vec![];
        if let Some(rc_node) = root {
            q.push((0, rc_node.clone()));
        }
        let mut result = vec![];
        while let Some((level, rc_node)) = q.pop() {
            if level == result.len() {
                result.push(vec![]);
            }
            let node = rc_node.borrow();
            result[level].push(rc_node.borrow().val);
            if let Some(rc_right) = &node.right {
                q.push((level + 1, rc_right.clone()));
            }
            if let Some(rc_left) = &node.left {
                q.push((level + 1, rc_left.clone()));
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {}
}