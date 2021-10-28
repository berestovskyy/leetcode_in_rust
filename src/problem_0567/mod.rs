//
// Problem 567. Permutation in String (Medium)
// https://leetcode.com/problems/permutation-in-string/
//
// 0ms/2.2MB
// Space complexity: O(1)
// Runtime complexity: O(n)
//
// test problem_0567::tests::string_1k       ... bench:       8,462 ns/iter (+/- 1,139)
//

struct Solution {}

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let (mut hm1, mut hm2) = ([0u16; 26], [0u16; 26]);
        for ch in s1.chars() {
            hm1[(ch as u8 - b'a') as usize] += 1;
        }
        let s2 = s2.as_bytes();
        for (i, ch) in s2.iter().enumerate() {
            hm2[(ch - b'a') as usize] += 1;
            if i >= s1.len() {
                hm2[(s2[i - s1.len()] - b'a') as usize] -= 1;
            }
            if hm1 == hm2 {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(
            Solution::check_inclusion("ab".into(), "eidbaooo".into()),
            true
        );
        assert_eq!(
            Solution::check_inclusion("ab".into(), "eidboaoo".into()),
            false
        );
    }

    #[test]
    fn extra() {
        assert_eq!(Solution::check_inclusion("a".into(), "ab".into()), true);
    }

    #[bench]
    fn string_1k(b: &mut Bencher) {
        let mut s = String::with_capacity(1_000);
        for i in 0..1_000 - 3 {
            s.push('a');
        }
        s.push('b');
        s.push('c');
        s.push('d');
        test::black_box(&s);
        b.iter(|| Solution::check_inclusion("abcd".into(), s.clone()));
    }
}
