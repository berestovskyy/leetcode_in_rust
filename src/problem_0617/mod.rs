//
// Problem 617. Merge Two Binary Trees (Easy)
// https://leetcode.com/problems/merge-two-binary-trees/
//
// 4ms (94%)/2.2MB (84%)
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
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (root1, root2) {
            (Some(root1), Some(root2)) => {
                root1.borrow_mut().val += root2.borrow().val;
                let left1 = root1.borrow_mut().left.take();
                let left2 = root2.borrow_mut().left.take();
                root1.borrow_mut().left = Self::merge_trees(left1, left2);
                let right1 = root1.borrow_mut().right.take();
                let right2 = root2.borrow_mut().right.take();
                root1.borrow_mut().right = Self::merge_trees(right1, right2);
                Some(root1)
            }
            (Some(root1), None) => Some(root1),
            (None, Some(root2)) => Some(root2),
            (_, _) => None,
        }
    }
}
