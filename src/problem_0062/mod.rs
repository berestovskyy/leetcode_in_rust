//
// Problem 62. Unique Paths (Medium)
// See [on LeetCode](https://leetcode.com/problems/unique-paths/).
//
// 0ms (100%)/2MB (90%)
// Space complexity: O(n)
// Runtime complexity: O(n²)
//
// test problem_0062::tests::single                   ... bench:         741 ns/iter (+/- 93)
//

struct Solution {}

impl Solution {
    pub fn unique_paths(mut m: i32, mut n: i32) -> i32 {
        if m > n {
            std::mem::swap(&mut m, &mut n);
        }
        let mut dp = vec![1_u64; m as usize];
        for j in 1..n as usize {
            for i in 1..m as usize {
                dp[i] += dp[i - 1];
            }
        }
        dp[m as usize - 1] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(Solution::unique_paths(3, 7), 28);
        assert_eq!(Solution::unique_paths(3, 2), 3);
        assert_eq!(Solution::unique_paths(7, 3), 28);
        assert_eq!(Solution::unique_paths(3, 3), 6);
    }

    #[bench]
    fn single(b: &mut Bencher) {
        b.iter(|| Solution::unique_paths(100, 10));
    }
}
