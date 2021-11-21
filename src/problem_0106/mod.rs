//
// Problem 106. Construct Binary Tree from Inorder and Postorder Traversal (Medium)
// See [on LeetCode](https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/).
//
// 0ms (100%)/2.5MB (73%)
// Space complexity: O(n)
// Runtime complexity: O(n²)
//
// test problem_0106::tests::vec_1k                   ... bench:     236,483 ns/iter (+/- 47,068)
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

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn helper(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(&last) = postorder.last() {
                let pos = inorder.iter().position(|&e| e == last).unwrap();
                Some(Rc::new(RefCell::new(TreeNode {
                    val: last,
                    left: helper(&inorder[0..pos], &postorder[0..pos]),
                    right: helper(&inorder[pos + 1..], &postorder[pos..postorder.len() - 1]),
                })))
            } else {
                None
            }
        }
        helper(&inorder, &postorder)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(
            Solution::build_tree(vec![9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3]).is_some(),
            true
        );
        assert_eq!(Solution::build_tree(vec![-1], vec![-1]).is_some(), true);
    }

    #[bench]
    fn vec_1k(b: &mut Bencher) {
        let mut v1 = Vec::with_capacity(1_000);
        let mut v2 = Vec::with_capacity(1_000);
        for i in 0..1_000 {
            v1.push(i);
            v2.push(i);
        }
        test::black_box(&v1);
        test::black_box(&v2);
        b.iter(|| Solution::build_tree(v1.clone(), v2.clone()));
    }
}
