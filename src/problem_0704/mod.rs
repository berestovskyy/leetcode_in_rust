//
// Problem 704. Binary Search (Easy)
// https://leetcode.com/problems/binary-search/
//
// 0ms/2.3MB
// Space complexity: O(1)
// Runtime complexity: avg: O(log n)
//
// test problem_0704::tests::search_1k           ... bench:         163 ns/iter (+/- 6)
//

struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (0, nums.len() as i32 - 1);
        while l <= r {
            let m = l + (r - l) / 2;
            match nums[m as usize].cmp(&target) {
                std::cmp::Ordering::Less => l = m + 1,
                std::cmp::Ordering::Greater => r = m - 1,
                std::cmp::Ordering::Equal => return m,
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    }

    #[bench]
    fn vec_1k(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        for i in 0..1_000 {
            v.push(i);
        }
        test::black_box(&v);
        b.iter(|| Solution::search(v.clone(), -1));
    }
}
