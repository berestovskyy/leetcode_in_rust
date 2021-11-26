//
// Problem 115. Distinct Subsequences (Hard)
// See [on LeetCode](https://leetcode.com/problems/distinct-subsequences/).
//
// 0ms (100%)/2.1MB (100%)
// Space complexity: O(n)
// Runtime complexity: O(n²)
//
// test problem_0115::tests::string_1k                ... bench:       3,890 ns/iter (+/- 428)
//

struct Solution {}

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let mut d = vec![0_i64; t.len() + 1];
        d[0] = 1;
        for si in s.bytes() {
            for (j, tj) in t.bytes().enumerate().rev() {
                if si == tj {
                    d[j + 1] += d[j];
                }
            }
        }
        d[t.len()] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(Solution::num_distinct("rabbbit".into(), "rabbit".into()), 3);
        assert_eq!(Solution::num_distinct("babgbag".into(), "bag".into()), 5);
    }

    #[bench]
    fn string_1k(b: &mut Bencher) {
        let mut s = String::with_capacity(1_000);
        for i in 0..1_000 / 5 {
            s.push_str("rabit");
        }
        b.iter(|| Solution::num_distinct(s.clone(), "rabbit".into()));
    }
}
