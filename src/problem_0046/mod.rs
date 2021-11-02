//
// Problem 46. Permutations (Medium)
// https://leetcode.com/problems/permutations/
//
// 0ms (100%)/2.2MB (44%)
// Space complexity: O(n)
// Runtime complexity: O(n²)?
//
// test problem_0036::tests::is_valid_sudoku        ... bench:         659 ns/iter (+/- 36)
//

struct Solution {}

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.is_empty() {
            return vec![Vec::with_capacity(6)];
        }
        let mut ret = Vec::new();
        for i in 0..nums.len() {
            let mut nums_copy = nums.clone();
            let n = nums_copy.remove(i);
            for mut v in Self::permute(nums_copy) {
                v.push(n);
                ret.push(v);
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(
            Solution::permute(vec![1, 2, 3]),
            vec![
                vec![3, 2, 1],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![1, 2, 3]
            ]
        );
        assert_eq!(Solution::permute(vec![0, 1]), vec![vec![1, 0], vec![0, 1]]);
        assert_eq!(Solution::permute(vec![1]), vec![vec![1]]);
    }

    #[bench]
    fn single(b: &mut Bencher) {
        b.iter(|| Solution::permute(vec![1, 2, 3]));
    }
}
