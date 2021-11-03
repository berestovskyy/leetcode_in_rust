//
// Problem 129. Sum Root to Leaf Numbers (Medium)
// https://leetcode.com/problems/sum-root-to-leaf-numbers/
//
// 0ms (100%)/2MB (63%)
// Space complexity: O(1)
// Runtime complexity: O(n)
//
// TODO: tree tests
//

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

struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, digits: i32) -> i32 {
            if let Some(node) = node {
                if node.borrow().left == None && node.borrow().right == None {
                    return digits * 10 + node.borrow().val;
                }
                return dfs(&node.borrow().left, digits * 10 + node.borrow().val)
                    + dfs(&node.borrow().right, digits * 10 + node.borrow().val);
            }
            0
        }
        dfs(&root, 0)
    }
}
