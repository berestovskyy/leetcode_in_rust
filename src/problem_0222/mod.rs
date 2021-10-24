//
// Problem 222. Count Complete Tree Nodes (Medium)
// https://leetcode.com/problems/count-complete-tree-nodes/
//
// 4ms/4.6MB
// Space complexity: O(1)
// Runtime complexity: O(log n)
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
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                let mut lh = 1;
                let mut cur = node.borrow().left.clone();
                while let Some(n) = cur {
                    lh += 1;
                    cur = n.borrow().left.clone();
                }
                let mut rh = 1;
                let mut cur = node.borrow().right.clone();
                while let Some(n) = cur {
                    rh += 1;
                    cur = n.borrow().right.clone();
                }
                match lh.cmp(&rh) {
                    std::cmp::Ordering::Equal => 2i32.pow(lh) - 1,
                    _ => {
                        1 + Solution::count_nodes(node.borrow().left.clone())
                            + Solution::count_nodes(node.borrow().right.clone())
                    }
                }
            }
            None => 0,
        }
    }
}
