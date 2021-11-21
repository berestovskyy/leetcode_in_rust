//
// Problem 1413. Minimum Value to Get Positive Step by Step Sum (Easy)
// See [on LeetCode](https://leetcode.com/problems/minimum-value-to-get-positive-step-by-step-sum/).
//
// 0ms (100%)/2.2MB
// Space complexity: O(1)
// Runtime complexity: O(n)
//
// test problem_1413::tests::vec_1k                   ... bench:         596 ns/iter (+/- 39)
//

struct Solution {}

impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        let (mut res, mut sum) = (0, 0);
        for n in nums {
            sum += n;
            res = res.min(sum);
        }
        1 - res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(Solution::min_start_value(vec![-3, 2, -3, 4, 2]), 5);
        assert_eq!(Solution::min_start_value(vec![1, 2]), 1);
        assert_eq!(Solution::min_start_value(vec![1, -2, -3]), 5);
    }

    #[bench]
    fn vec_1k(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        for i in 0..1_000 {
            v.push(i % 39);
        }
        test::black_box(&v);
        b.iter(|| Solution::min_start_value(v.clone()));
    }
}
