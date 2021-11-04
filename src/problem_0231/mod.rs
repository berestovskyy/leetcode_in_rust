//
// Problem 231. Power of Two (Easy)
// https://leetcode.com/problems/power-of-two/
//
// 0ms (100%)/2MB (89%)
// Space complexity: O(1)
// Runtime complexity: O(1)
//
// TODO: test problem_0231::tests::single_1k                ... bench:           0 ns/iter (+/- 0)
//

struct Solution {}

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        n > 0 && (n & (n - 1)) == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(Solution::is_power_of_two(1), true);
        assert_eq!(Solution::is_power_of_two(16), true);
        assert_eq!(Solution::is_power_of_two(3), false);
        assert_eq!(Solution::is_power_of_two(4), true);
        assert_eq!(Solution::is_power_of_two(5), false);
    }

    #[bench]
    fn single_1k(b: &mut Bencher) {
        for i in 0..10 {
            b.iter(|| Solution::is_power_of_two(i));
        }
    }
}
