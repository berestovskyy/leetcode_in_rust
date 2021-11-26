//
// Problem 1328. Break a Palindrome (Medium)
// See [on LeetCode](https://leetcode.com/problems/break-a-palindrome/).
//
// 0ms (100%)/2MB (100%)
// Space complexity: O(1)
// Runtime complexity: O(n)
//
// test problem_1328::tests::string_1k                ... bench:         382 ns/iter (+/- 16)
//

struct Solution {}

impl Solution {
    pub fn break_palindrome(palindrome: String) -> String {
        let (len, mut res) = (palindrome.len(), palindrome.into_bytes());
        if len == 1 {
            return "".into();
        }
        for i in 0..len / 2 {
            if res[i] != b'a' {
                res[i] = b'a';
                return String::from_utf8(res).unwrap();
            }
        }
        res[len - 1] = b'b';
        String::from_utf8(res).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(Solution::break_palindrome("abccba".into()), "aaccba");
        assert_eq!(Solution::break_palindrome("a".into()), "");
        assert_eq!(Solution::break_palindrome("aa".into()), "ab");
        assert_eq!(Solution::break_palindrome("aba".into()), "abb");
    }

    #[bench]
    fn string_1k(b: &mut Bencher) {
        let mut s = String::with_capacity(1_000);
        for _ in 0..1_000 {
            s.push('a');
        }
        test::black_box(&s);
        b.iter(|| Solution::break_palindrome(s.clone()));
    }
}
