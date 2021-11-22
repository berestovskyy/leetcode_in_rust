//
// Problem 1189. Maximum Number of Balloons (Easy)
// See [on LeetCode](https://leetcode.com/problems/maximum-number-of-balloons/).
//
// 2ms (100%)/2.1MB (60%)
// Space complexity: O(1)
// Runtime complexity: O(n)
//
// test problem_1189::tests::single                   ... bench:          82 ns/iter (+/- 12)
//

struct Solution {}

impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let (mut b, mut a, mut l, mut o, mut n) = (0, 0, 0, 0, 0);
        for c in text.chars() {
            match c {
                'b' => b += 1,
                'a' => a += 1,
                'l' => l += 1,
                'o' => o += 1,
                'n' => n += 1,
                _ => {}
            }
        }
        l /= 2;
        o /= 2;
        b.min(a).min(l).min(o).min(n)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(Solution::max_number_of_balloons("nlaebolko".into()), 1);
        assert_eq!(
            Solution::max_number_of_balloons("loonbalxballpoon".into()),
            2
        );
        assert_eq!(Solution::max_number_of_balloons("leetcode".into()), 0);
    }

    #[bench]
    fn single(b: &mut Bencher) {
        let s = "abcdefghijklmnopqrstuvwxyz";
        test::black_box(&s);
        b.iter(|| Solution::max_number_of_balloons(s.to_string()));
    }
}
