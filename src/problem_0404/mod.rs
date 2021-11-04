//
// Problem 404. Sum of Left Leaves (Easy)
// https://leetcode.com/problems/sum-of-left-leaves/
//
// 0ms (100%)/2.2MB (45%)
// Space complexity: O(1)
// Runtime complexity: O(n)
//
// TODO: tree tests
//

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

struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn left_leaves(node: &Option<Rc<RefCell<TreeNode>>>, is_left: bool) -> i32 {
            let mut sum = 0;
            if let Some(node) = node {
                let node = node.borrow();
                sum += left_leaves(&node.left, true);
                sum += left_leaves(&node.right, false);
                if node.left.is_none() && node.right.is_none() && is_left {
                    sum += node.val;
                }
            }
            sum
        }
        left_leaves(&root, false)
    }
}
