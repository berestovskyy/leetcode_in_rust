//
// Problem 95. Unique Binary Search Trees II (Medium)
// https://leetcode.com/problems/unique-binary-search-trees-ii/
//
// 0ms/2.7MB
// Space complexity: O(n)
// Runtime complexity: O(n)
//
// TODO: tree comparison
//
// test problem_0095::tests::single          ... bench:     987,729 ns/iter (+/- 132,958)
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
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        pub fn gen(l: i32, r: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
            if l > r {
                return vec![None];
            }
            let mut res = vec![];
            for i in l..r + 1 {
                let lnodes = gen(l, i - 1);
                let rnodes = gen(i + 1, r);
                for ln in lnodes.iter() {
                    for rn in rnodes.iter() {
                        let node = Some(Rc::new(RefCell::new(TreeNode {
                            val: i,
                            left: ln.clone(),
                            right: rn.clone(),
                        })));
                        res.push(node);
                    }
                }
            }
            res
        }
        gen(1, n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        // TODO: tree comparison
        assert_eq!(Solution::generate_trees(3).len(), 5);
        assert_eq!(Solution::generate_trees(1).len(), 1);
    }

    #[bench]
    fn single(b: &mut Bencher) {
        b.iter(|| Solution::generate_trees(8));
    }
}
