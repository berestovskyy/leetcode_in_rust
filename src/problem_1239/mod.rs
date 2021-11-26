//
// Problem 1239. Maximum Length of a Concatenated String with Unique Characters (Medium)
// https://leetcode.com/problems/maximum-length-of-a-concatenated-string-with-unique-characters/
//
// 0ms (100%)/2.2MB (33%)
// Space complexity: O(n)
// Runtime complexity: O(n²)?
//
// test problem_1239::tests::vec_1k                   ... bench:     336,915 ns/iter (+/- 52,332)
//

struct Solution {}

impl Solution {
    pub fn max_length(arr: Vec<String>) -> i32 {
        let mut res = 0;
        let mut dp = Vec::with_capacity(arr.len());
        dp.push(0);
        for s in arr {
            let mut m = 0;
            let mut dup = 0;
            for &b in s.as_bytes() {
                dup |= m & (1 << (b - b'a'));
                m |= 1 << (b - b'a');
            }
            if dup > 0 {
                continue;
            }
            for i in (0..dp.len()).rev() {
                if dp[i] & m > 0 {
                    continue;
                }
                dp.push(dp[i] | m);
                res = res.max((*dp.last().unwrap() as i32).count_ones() as i32);
            }
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
        assert_eq!(
            Solution::max_length(vec!["un".into(), "iq".into(), "ue".into()]),
            4
        );
        assert_eq!(
            Solution::max_length(vec!["cha".into(), "r".into(), "act".into(), "ers".into()]),
            6
        );
        assert_eq!(
            Solution::max_length(vec!["abcdefghijklmnopqrstuvwxyz".into()]),
            26
        );
        assert_eq!(Solution::max_length(vec!["aa".into(), "bb".into()]), 0);
    }

    #[bench]
    fn vec_1k(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        for _ in 0..1_000 {
            v.push("abcdefghijklmnopqrstuvwxyz".into());
        }
        b.iter(|| Solution::max_length(v.clone()));
    }
}
