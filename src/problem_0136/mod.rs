//
// Problem 136. Single Number (Easy)
// https://leetcode.com/problems/single-number/
//
// 0ms (100%)/2.2MB (78%)
// Space complexity: O(1)
// Runtime complexity: O(n)
//
// test problem_0130::tests::vec_1k                   ... bench:       5,225 ns/iter (+/- 246)
//

struct Solution {}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for n in nums {
            res ^= n;
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
        assert_eq!(Solution::single_number(vec![2, 2, 1]), 1);
        assert_eq!(Solution::single_number(vec![4, 1, 2, 1, 2]), 4);
        assert_eq!(Solution::single_number(vec![1]), 1);
    }

    #[bench]
    fn vec_1k(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        for i in 0..1_000 / 2 {
            v.push(i);
        }
        v.push(2000);
        b.iter(|| {
            Solution::single_number(v.clone());
        });
    }
}
