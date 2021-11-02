//
// Problem 784. Letter Case Permutation (Medium)
// https://leetcode.com/problems/letter-case-permutation/
//
// 4ms (94%)/2.4MB (27%)
// Space complexity: O(n)
// Runtime complexity: O(n)
//
// test problem_0784::tests::string_50       ... bench:     161,630 ns/iter (+/- 9,823)
//

struct Solution {}

impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        fn dfs(chars: &mut [char], res: &mut Vec<String>, idx: usize) {
            if idx == chars.len() {
                res.push(chars.iter().collect());
            } else {
                dfs(chars, res, idx + 1);
                if chars[idx].is_ascii_alphabetic() {
                    chars[idx] = match chars[idx].is_ascii_uppercase() {
                        true => chars[idx].to_ascii_lowercase(),
                        fasle => chars[idx].to_ascii_uppercase(),
                    };
                    dfs(chars, res, idx + 1);
                    chars[idx] = match chars[idx].is_ascii_uppercase() {
                        true => chars[idx].to_ascii_lowercase(),
                        fasle => chars[idx].to_ascii_uppercase(),
                    };
                }
            }
        }
        let mut chars = s.chars().collect::<Vec<_>>();
        let mut res = vec![];
        dfs(&mut chars, &mut res, 0);
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
            Solution::letter_case_permutation("a1b2".into()),
            vec!["a1b2", "a1B2", "A1b2", "A1B2"]
        );
        assert_eq!(
            Solution::letter_case_permutation("3z4".into()),
            vec!["3z4", "3Z4"]
        );
        assert_eq!(
            Solution::letter_case_permutation("12345".into()),
            vec!["12345"]
        );
        assert_eq!(Solution::letter_case_permutation("0".into()), vec!["0"]);
    }

    #[bench]
    fn string_50(b: &mut Bencher) {
        let mut s = String::with_capacity(1_000);
        for i in 0..50 / 5 {
            s.push('a');
            s.push('0');
            s.push('1');
            s.push('2');
            s.push('3');
        }
        b.iter(|| Solution::letter_case_permutation(s.clone()));
    }
}
