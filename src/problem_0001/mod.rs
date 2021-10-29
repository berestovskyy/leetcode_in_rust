//
// Problem 1. Two Sum (Easy)
// https://leetcode.com/problems/two-sum/
//
// 0ms/2MB
// Space complexity: O(n)
// Runtime complexity: O(n)
//
// test problem_0001::tests::vec_1k          ... bench:      15,941 ns/iter (+/- 1,297)
//

struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hm = std::collections::HashMap::with_capacity(nums.len());
        for (i, n) in nums.iter().enumerate() {
            if let Some(&j) = hm.get(&(target - n)) {
                return vec![j, i as i32];
            }
            hm.insert(n, i as i32);
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    }

    #[bench]
    fn vec_1k(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        for i in -500..500 {
            v.push(i);
        }
        test::black_box(&v);
        b.iter(|| Solution::two_sum(v.clone(), 99));
    }
}
