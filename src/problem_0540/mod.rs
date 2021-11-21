//
// Problem 540. Single Element in a Sorted Array (Medium)
// See [on LeetCode](https://leetcode.com/problems/single-element-in-a-sorted-array/).
//
// 0ms (100%)/2.2MB (36%)
// Space complexity: O(1)
// Runtime complexity: O(log n)?
//
// test problem_0540::tests::vec_1k                   ... bench:         110 ns/iter (+/- 5)
//

struct Solution {}

impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, nums.len() - 1);
        while l < r {
            let m = l + (r - l) / 2;
            if (m % 2 == 0) == (nums[m] != nums[m + 1]) {
                r = m;
            } else {
                l = m + 1;
            }
        }
        nums[l]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(
            Solution::single_non_duplicate(vec![1, 1, 2, 3, 3, 4, 4, 8, 8]),
            2
        );
        assert_eq!(
            Solution::single_non_duplicate(vec![3, 3, 7, 7, 10, 11, 11]),
            10
        );
    }

    #[bench]
    fn vec_1k(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        for i in 0..1_000 / 2 {
            v.push(i);
            v.push(i);
        }
        v.push(1_000);
        test::black_box(&v);
        b.iter(|| Solution::single_non_duplicate(v.clone()));
    }
}
