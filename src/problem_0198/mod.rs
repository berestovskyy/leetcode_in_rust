//
// Problem 198. House Robber (Medium)
// https://leetcode.com/problems/house-robber/
//
// 0ms (100%)/2.3MB (5%)
// Space complexity: O(1)
// Runtime complexity: O(n)
//
// test problem_0198::tests::vec_1k                   ... bench:         636 ns/iter (+/- 36)
//

struct Solution {}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut prev_1 = 0;
        let mut prev_2 = 0;
        for n in nums {
            let nxt = (n + prev_2).max(prev_1);
            prev_2 = prev_1;
            prev_1 = nxt;
        }
        prev_1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
        assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
    }

    #[bench]
    fn vec_1k(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        for i in 0..1_000 {
            v.push(i);
        }
        test::black_box(&v);
        b.iter(|| Solution::rob(v.clone()));
    }
}
