//
// Problem 226. Invert Binary Tree (Easy)
// https://leetcode.com/problems/invert-binary-tree/
//
// 0ms (100%)/2.2MB
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
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(root) => {
                let l = Self::invert_tree(root.borrow_mut().left.take());
                let r = Self::invert_tree(root.borrow_mut().right.take());
                root.borrow_mut().right = l;
                root.borrow_mut().left = r;
                Some(root)
            }
            None => None,
        }
    }
}
