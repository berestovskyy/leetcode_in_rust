//
// Problem 1137. N-th Tribonacci Number (Easy)
// See [on LeetCode](https://leetcode.com/problems/n-th-tribonacci-number/).
//
// 0ms (100%)/2.1MB (100%)
// Space complexity: O(n)
// Runtime complexity: O(n²)
//
// test problem_1137::tests::single                   ... bench:           0 ns/iter (+/- 0)
//

struct Solution {}

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let (mut n0, mut n1, mut n2, mut n3) = (0_i32, 1_i32, 1, 2);
        for i in 0..n as usize {
            n0 = n1;
            n1 = n2;
            n2 = n3;
            n3 = n0.overflowing_add(n1).0.overflowing_add(n2).0;
        }
        n0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(Solution::tribonacci(4), 4);
        assert_eq!(Solution::tribonacci(25), 1_389_537);
    }

    #[bench]
    fn single(b: &mut Bencher) {
        b.iter(|| Solution::tribonacci(37));
    }
}
