//
// Problem 446. Arithmetic Slices II - Subsequence (Hard)
// https://leetcode.com/problems/arithmetic-slices-ii-subsequence/
//
// 68ms (97%)/28MB (16%)
// Space complexity: O(n²)
// Runtime complexity: O(n²)
//
// test problem_0446::tests::vec_100                  ... bench:     213,327 ns/iter (+/- 18,178)
//

struct Solution {}

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut dp = Vec::with_capacity(nums.len());
        for i in 0..nums.len() {
            dp.push(std::collections::HashMap::with_capacity(nums.len()));
            for j in 0..i {
                let diff = nums[i].overflowing_sub(nums[j]);
                let dpj = *dp[j].get(&diff).unwrap_or(&0);
                *dp[i].entry(diff).or_insert(0) += dpj + 1;
                ans += dpj;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(
            Solution::number_of_arithmetic_slices(vec![2, 4, 6, 8, 10]),
            7
        );
        assert_eq!(
            Solution::number_of_arithmetic_slices(vec![7, 7, 7, 7, 7]),
            16
        );
    }

    #[test]
    fn extra() {
        assert_eq!(
            Solution::number_of_arithmetic_slices(vec![0, 2000000000, -294967296]),
            0
        );
    }

    #[bench]
    fn vec_100(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        for i in 0..100 {
            v.push(i);
        }
        test::black_box(&v);
        b.iter(|| Solution::number_of_arithmetic_slices(v.clone()));
    }
}
