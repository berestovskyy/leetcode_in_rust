//
// Problem 368. Largest Divisible Subset (Medium)
// See [on LeetCode](https://leetcode.com/problems/largest-divisible-subset/).
//
// 12ms (89%)/2.1MB (82%)
// Space complexity: O(n)
// Runtime complexity: O(n²)?
//
// test problem_0368::tests::vec_1k                   ... bench:   1,394,493 ns/iter (+/- 188,667)
//

struct Solution {}

impl Solution {
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();
        let mut dp = vec![1; nums.len()];
        let (mut max, mut num) = (0, 0);

        for i in 0..nums.len() {
            for j in 0..i {
                if nums[i] % nums[j] == 0 && dp[i] < dp[j] + 1 {
                    dp[i] = dp[j] + 1;
                }
            }
            if max < dp[i] {
                max = dp[i];
                num = nums[i];
            }
        }
        let mut res = vec![];
        for i in (0..nums.len()).rev() {
            if dp[i] == max && num % nums[i] == 0 {
                res.push(nums[i]);
                max -= 1;
                num = nums[i];
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        let mut v = Solution::largest_divisible_subset(vec![1, 2, 3]);
        v.sort_unstable();
        assert_eq!(v, vec![1, 2]);
        let mut v = Solution::largest_divisible_subset(vec![1, 2, 4, 8]);
        v.sort_unstable();
        assert_eq!(v, vec![1, 2, 4, 8]);
    }

    #[bench]
    fn vec_1k(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        for i in 1..1_001 {
            v.push(i);
        }
        test::black_box(&v);
        b.iter(|| Solution::largest_divisible_subset(v.clone()));
    }
}
