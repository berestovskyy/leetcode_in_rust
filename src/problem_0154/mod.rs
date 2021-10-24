//
// Problem 154. Find Minimum in Rotated Sorted Array II (Hard)
// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array-ii/
//
// 0ms/2MB
// Space complexity: O(1)
// Runtime complexity: avg: O(log n)
//                     worst case: O(n), when all numbers are the same
//
// test problem_0154::tests::find_min_1k         ... bench:         159 ns/iter (+/- 4)
// test problem_0154::tests::find_min_1k_rotated ... bench:         159 ns/iter (+/- 6)
// test problem_0154::tests::find_min_1k_same    ... bench:       2,882 ns/iter (+/- 58)
//

struct Solution {}

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, nums.len() - 1);
        while l < r {
            let m = l + (r - l) / 2;
            match nums[m].cmp(&nums[r]) {
                std::cmp::Ordering::Greater => l = m + 1,
                std::cmp::Ordering::Less => r = m,
                std::cmp::Ordering::Equal => r -= 1,
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
        assert_eq!(Solution::find_min(vec![1, 3, 5]), 1);
        assert_eq!(Solution::find_min(vec![2, 2, 2, 0, 1]), 0);
        assert_eq!(Solution::find_min(vec![2, 2, 2, 2, 2]), 2);
        assert_eq!(Solution::find_min(vec![2, 2, 0, 2, 2]), 0);
        assert_eq!(Solution::find_min(vec![2, 2, 3, 1, 2]), 1);
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

    #[bench]
    fn find_min_1k_same(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        for i in 0..1_000 {
            v.push(0);
        }
        test::black_box(&v);
        b.iter(|| Solution::find_min(v.clone()));
    }
}
