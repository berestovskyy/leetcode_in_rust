//
// Problem 152. Maximum Product Subarray (Medium)
// See [on LeetCode](https://leetcode.com/problems/maximum-product-subarray/).
//
// 0ms (100%)/2.1MB (85%)
// Space complexity: O(1)
// Runtime complexity: O(n)
//
// test problem_0152::tests::vec_1k                   ... bench:       1,545 ns/iter (+/- 134)
//

struct Solution {}

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let (mut res, mut l, mut r) = (nums[0], 0, 0);
        for i in 0..nums.len() {
            l = nums[i] * if l == 0 { 1 } else { l };
            r = nums[nums.len() - i - 1] * if r == 0 { 1 } else { r };
            res = res.max(l).max(r);
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
        assert_eq!(Solution::max_product(vec![2, 3, -2, 4]), 6);
        assert_eq!(Solution::max_product(vec![-2, 0, -1]), 0);
    }

    #[bench]
    fn vec_1k(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        for i in 0..1_000 {
            v.push(i % 2);
        }
        test::black_box(&v);
        b.iter(|| Solution::max_product(v.clone()));
    }
}
