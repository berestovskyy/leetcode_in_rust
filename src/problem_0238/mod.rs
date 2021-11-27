//
// Problem 238. Product of Array Except Self (Medium)
// See [on LeetCode](https://leetcode.com/problems/product-of-array-except-self/).
//
// 10ms (79%)/3.2MB (32%)
// Space complexity: O(1)
// Runtime complexity: O(n)
//
// test problem_0238::tests::vec_1k                   ... bench:       1,735 ns/iter (+/- 246)
//

struct Solution {}

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let (mut l, mut r, mut pfx, mut sfx) = (0, nums.len(), 1, 1);
        let mut res = vec![1; nums.len()];
        for _ in 0..nums.len() {
            r -= 1;
            res[l] *= pfx;
            res[r] *= sfx;
            pfx *= nums[l];
            sfx *= nums[r];
            l += 1;
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
        assert_eq!(
            Solution::product_except_self(vec![1, 2, 3, 4]),
            vec![24, 12, 8, 6]
        );
        assert_eq!(
            Solution::product_except_self(vec![-1, 1, 0, -3, 3]),
            vec![0, 0, 9, 0, 0]
        );
    }

    #[bench]
    fn vec_1k(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        for i in 0..1_000 {
            v.push(i % 2);
        }
        b.iter(|| Solution::product_except_self(v.clone()));
    }
}
