//
// Problem 96. Unique Binary Search Trees (Medium)
// https://leetcode.com/problems/unique-binary-search-trees/
//
// 0ms (100%)/2.1MB (35%)
// Space complexity: O(n)
// Runtime complexity: O(n²)
//
// test problem_0096::tests::single                   ... bench:         614 ns/iter (+/- 145)
//

struct Solution {}

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let mut dp = vec![1];
        for i in 1..=n as usize {
            dp.push((1..=i).map(|j| dp[j - 1] * dp[i - j]).sum());
        }
        dp[n as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(Solution::num_trees(3), 5);
        assert_eq!(Solution::num_trees(1), 1);
    }

    #[bench]
    fn single(b: &mut Bencher) {
        b.iter(|| Solution::num_trees(19));
    }
}
