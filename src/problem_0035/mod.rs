//
// Problem 35. Search Insert Position (Easy)
// https://leetcode.com/problems/search-insert-position/
//
// 0ms (100%)/2.2MB
// Space complexity: O(1)
// Runtime complexity: O(log n)
//
// test problem_0035::tests::search_insert_1k     ... bench:         162 ns/iter (+/- 1)
//

struct Solution {}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (0, nums.len() as i32 - 1);
        while l <= r {
            let mid = l + (r - l) / 2;
            match nums[mid as usize].cmp(&target) {
                std::cmp::Ordering::Less => l = mid + 1,
                std::cmp::Ordering::Equal => return mid,
                std::cmp::Ordering::Greater => r = mid - 1,
            }
        }
        l
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 0), 0);
        assert_eq!(Solution::search_insert(vec![1], 0), 0);
    }

    #[bench]
    fn vec_1k(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        for i in 0..1_000 {
            v.push(i);
        }
        test::black_box(&v);
        b.iter(|| Solution::search_insert(v.clone(), 500));
    }
}
