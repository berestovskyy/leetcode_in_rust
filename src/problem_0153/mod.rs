//
// Problem 153. Find Minimum in Rotated Sorted Array (Medium)
// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/
//
// 0ms/2MB
// Space complexity: O(1)
// Runtime complexity: avg: O(log n)
//
// test problem_0153::tests::find_min_1k               ... bench:         103 ns/iter (+/- 16)
// test problem_0153::tests::find_min_1k_rotated       ... bench:         102 ns/iter (+/- 7)
//

struct Solution {}

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, nums.len() - 1);
        while l < r {
            let m = l + (r - l) / 2;
            if nums[m] > nums[r] {
                l = m + 1;
            } else {
                r = m;
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
    fn all() {
        assert_eq!(Solution::find_min(vec![3, 4, 5, 1, 2]), 1);
        assert_eq!(Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
        assert_eq!(Solution::find_min(vec![11, 13, 15, 17]), 11);
    }

    #[bench]
    fn find_min_1k(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        for i in 0..1_000 {
            v.push(i);
        }
        test::black_box(&v);
        b.iter(|| Solution::find_min(v.clone()));
    }

    #[bench]
    fn find_min_1k_rotated(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        for i in 500..1_000 {
            v.push(i);
        }
        for i in 0..500 {
            v.push(i);
        }
        test::black_box(&v);
        b.iter(|| Solution::find_min(v.clone()));
    }
}
