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
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(preorder: &Vec<i32>, pre_left: usize, pre_right: usize, in_map: &HashMap<&i32, usize>, in_left: usize, in_right: usize) -> Option<Rc<RefCell<TreeNode>>> {
            if pre_left > pre_right || in_left > in_right {
                return None;
            }
            let root_value = preorder[pre_left];
            let mut root_node = TreeNode::new(root_value);
            let &pvot = in_map.get(&root_value).unwrap();
            if pvot > 0 {
                root_node.left = dfs(preorder, pre_left + 1, pvot - in_left + pre_left, in_map, in_left, pvot - 1);
            }
            root_node.right = dfs(preorder, pvot - in_left + pre_left + 1, pre_right, in_map, pvot + 1, in_right);
            Some(Rc::new(RefCell::new(root_node)))
        }
        if preorder.len() == 0 {
            return None;
        }
        let in_map: HashMap<&i32, usize> = inorder.iter().zip(0..inorder.len()).collect();
        dfs(&preorder, 0, preorder.len() - 1, &in_map, 0, inorder.len() - 1)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        Solution::build_tree(vec![1, 2, 3], vec![3, 2, 1]);
    }

    #[test]
    fn case2() {
        println!("{:?}", Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]));
    }
}