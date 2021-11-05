//
// Problem 441. Arranging Coins (Easy)
// https://leetcode.com/problems/arranging-coins/
//
// 0ms (100%)/2.1MB (50%)
// Space complexity: O()
// Runtime complexity: O()
//
// test problem_0441::tests::single_1k                ... bench:           0 ns/iter (+/- 0)
// test problem_0441::tests::single_1k_2              ... bench:      16,054 ns/iter (+/- 1,054)
//

struct Solution {}

impl Solution {
    pub fn arrange_coins(mut n: i32) -> i32 {
        // n = (a1 + ak)/2*k
        // n = (1 + k)/2*k
        // n = k^2/2 + k/2
        // >>> k^2 + k - 2n = 0 <<<
        // d = b^2 - 4ac
        // d = 1 + 4*1*2n = 8n + 1
        // k = (-b +- sqrt(d))/2a
        // k = (-1 +- sqrt(8n + 1))/2, k > 0
        // k = sqrt(8n + 1)/2 - 1/2
        ((8.0 * n as f64 + 1.0).sqrt() / 2.0 - 1.0 / 2.0) as i32
    }
}

impl Solution {
    pub fn arrange_coins_2(mut n: i32) -> i32 {
        let mut k = 1;
        while n - k >= k + 1 {
            n -= k;
            k += 1;
        }
        k
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(Solution::arrange_coins(5), 2);
        assert_eq!(Solution::arrange_coins(8), 3);
    }

    #[test]
    fn extra() {
        assert_eq!(Solution::arrange_coins(1), 1);
        assert_eq!(Solution::arrange_coins(i32::MAX), 65535);
    }

    #[bench]
    fn single_1k(b: &mut Bencher) {
        b.iter(|| {
            for _ in 0..1_000 {
                Solution::arrange_coins(i32::MAX);
            }
        });
    }

    #[bench]
    fn single_1k_2(b: &mut Bencher) {
        b.iter(|| {
            for _ in 0..1_000 {
                Solution::arrange_coins_2(i32::MAX);
            }
        });
    }
}
