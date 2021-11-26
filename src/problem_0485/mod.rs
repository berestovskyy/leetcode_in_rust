//
// Problem 485. Max Consecutive Ones (Easy)
// See [on LeetCode](https://leetcode.com/problems/max-consecutive-ones/).
//
// 4ms (94%)/2.1MB (94%)
// Space complexity: O(n)
// Runtime complexity: O(n)
//
// test problem_0485::tests::vec_1k                   ... bench:         819 ns/iter (+/- 80)
//

struct Solution {}

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        nums.split(|&n| n == 0).map(|s| s.len()).max().unwrap() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(
            Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]),
            3
        );
        assert_eq!(
            Solution::find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1]),
            2
        );
    }

    #[bench]
    fn vec_1k(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        for i in 0..1_000 {
            v.push(i % 2);
        }
        test::black_box(&v);
        b.iter(|| Solution::find_max_consecutive_ones(v.clone()));
    }
}
