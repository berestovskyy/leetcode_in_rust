//
// Problem 331. Verify Preorder Serialization of a Binary Tree (Medium)
// https://leetcode.com/problems/verify-preorder-serialization-of-a-binary-tree/
//
// 0ms/2.1MB
// Space complexity: O(1)
// Runtime complexity: avg: O(n)
//
// test problem_0331::tests::is_valid_serialization    ... bench:         110 ns/iter (+/- 5)
//

struct Solution {}

impl Solution {
    pub fn is_valid_serialization(preorder: String) -> bool {
        let mut expected_children = 1;
        for n in preorder.split(',') {
            if expected_children == 0 {
                return false;
            }
            expected_children += match n {
                "#" => -1,
                _ => 1,
            };
        }
        expected_children == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn all() {
        assert_eq!(
            Solution::is_valid_serialization("9,3,4,#,#,1,#,#,2,#,6,#,#".into()),
            true
        );
        assert_eq!(Solution::is_valid_serialization("1,#".into()), false);
        assert_eq!(Solution::is_valid_serialization("9,#,#,1".into()), false);
    }

    #[bench]
    fn is_valid_serialization(b: &mut Bencher) {
        b.iter(|| Solution::is_valid_serialization("9,3,4,#,#,1,#,#,2,#,6,#,#".into()));
    }
}
