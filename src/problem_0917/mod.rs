//
// Problem 917. Reverse Only Letters (Easy)
// See [on LeetCode](https://leetcode.com/problems/reverse-only-letters/).
//
// 0ms (100%)/2MB (88%)
// Space complexity: O(n)
// Runtime complexity: O(n)
//
// test problem_0917::tests::string_1k                ... bench:       2,980 ns/iter (+/- 277)
//

struct Solution {}

impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        let (mut l, mut r) = (
            s.chars(),
            s.chars().rev().filter(|ch| ch.is_ascii_alphabetic()),
        );
        let mut res = String::with_capacity(s.len());
        for lc in l {
            if !lc.is_ascii_alphabetic() {
                res.push(lc);
            } else {
                res.push(r.next().unwrap());
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
        assert_eq!(Solution::reverse_only_letters("ab-cd".into()), "dc-ba");
        assert_eq!(
            Solution::reverse_only_letters("a-bC-dEf-ghIj".into()),
            "j-Ih-gfE-dCba"
        );
        assert_eq!(
            Solution::reverse_only_letters("Test1ng-Leet=code-Q!".into()),
            "Qedo1ct-eeLg=ntse-T!"
        );
    }

    #[bench]
    fn string_1k(b: &mut Bencher) {
        let mut s = String::with_capacity(1_000);
        for i in 0..1_000 / 5 {
            s.push('a');
            s.push('b');
            s.push('c');
            s.push('d');
            s.push('-');
        }
        test::black_box(&s);
        b.iter(|| Solution::reverse_only_letters(s.clone()));
    }
}
