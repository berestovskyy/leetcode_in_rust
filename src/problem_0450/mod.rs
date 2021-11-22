//
// Problem 450. Delete Node in a BST (Medium)
// See [on LeetCode](https://leetcode.com/problems/delete-node-in-a-bst/).
//
// 4ms (100%)/3.2MB (33%)
// Space complexity: O(1)
// Runtime complexity: O(log n)
//
// TODO: tree tests
//

struct Solution {}

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

// Based on:
// https://leetcode.com/problems/delete-node-in-a-bst/discuss/1193715/Rust-cheapest-and-best

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn delete_node(
        mut root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        root.delete(key);
        root
    }
}

trait Insertable<T> {
    fn insert(&mut self, item: T);
}

impl Insertable<Rc<RefCell<TreeNode>>> for TreeNode {
    fn insert(&mut self, other: Rc<RefCell<TreeNode>>) {
        if other.borrow().val >= self.val {
            let right = self.right.take();
            if let Some(node) = right {
                node.borrow_mut().insert(other);
                self.right = Some(node);
            } else {
                self.right = Some(other);
            }
        } else {
            let left = self.left.take();
            if let Some(node) = left {
                node.borrow_mut().insert(other);
                self.left = Some(node);
            } else {
                self.left = Some(other);
            }
        }
    }
}

trait Deletable<T> {
    fn delete(&mut self, key: T);
}

impl Deletable<i32> for Option<Rc<RefCell<TreeNode>>> {
    fn delete(&mut self, key: i32) {
        if let Some(node) = self.take() {
            let val = node.borrow().val;
            match val.cmp(&key) {
                std::cmp::Ordering::Equal => {
                    let left = node.borrow_mut().left.take();
                    let right = node.borrow_mut().right.take();
                    match (left, right) {
                        (None, None) => {
                            *self = None;
                        }
                        (Some(left), None) => {
                            *self = Some(left);
                        }
                        (None, Some(right)) => {
                            *self = Some(right);
                        }
                        (Some(left), Some(right)) => {
                            right.borrow_mut().insert(left);
                            *self = Some(right);
                        }
                    }
                }
                std::cmp::Ordering::Greater => {
                    node.borrow_mut().left.delete(key);
                    *self = Some(node);
                }
                std::cmp::Ordering::Less => {
                    node.borrow_mut().right.delete(key);
                    *self = Some(node);
                }
            }
        }
    }
}
