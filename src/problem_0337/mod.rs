//
// Problem 337. House Robber III (Medium)
// See [on LeetCode](https://leetcode.com/problems/house-robber-iii/).
//
// 0ms (100%)/2.9MB (36%)
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
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
            match root {
                None => (0, 0),
                Some(node) => {
                    let l = dfs(node.borrow().left.clone());
                    let r = dfs(node.borrow().right.clone());
                    let max_with_root = node.borrow().val + l.1 + r.1;
                    let max_without_root = l.0.max(l.1) + r.0.max(r.1);
                    (max_with_root, max_without_root)
                }
            }
        }
        let (a, b) = dfs(root);
        a.max(b)
    }
}
