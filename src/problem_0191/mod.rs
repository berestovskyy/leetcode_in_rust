//
// Problem 191. Number of 1 Bits (Easy)
// https://leetcode.com/problems/number-of-1-bits/
//
// 0ms (100%)/1.9MB (83%)
// Space complexity: O(1)
// Runtime complexity: O(1)
//
// TODO: test problem_0191::tests::single_100                ... bench:           3 ns/iter (+/- 0)
//

struct Solution {}

impl Solution {
    pub fn hamming_weight(mut n: u32) -> i32 {
        let mut sum = 0;
        for i in 0..8 {
            sum += match n & 0xf {
                0 => 0,
                1 => 1,
                2 => 1,
                3 => 2,
                4 => 1,
                5 => 2,
                6 => 2,
                7 => 3,
                8 => 1,
                9 => 2,
                10 => 2,
                11 => 3,
                12 => 2,
                13 => 3,
                14 => 3,
                _ => 4,
            };
            n >>= 4;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(Solution::hamming_weight(0x0B), 3);
        assert_eq!(Solution::hamming_weight(0x80), 1);
        assert_eq!(Solution::hamming_weight(0xFFFFFFFD), 31);
    }

    #[bench]
    fn single_100(b: &mut Bencher) {
        for i in 0..100 {
            b.iter(|| Solution::hamming_weight(0xFFFFFFFD));
        }
    }
}
