//
// Problem 3. Longest Substring Without Repeating Characters (Medium)
// https://leetcode.com/problems/longest-substring-without-repeating-characters/
//
// 0ms/2MB
// Space complexity: O(n)
// Runtime complexity: O(n)
//
// test problem_0003::tests::vec_1k          ... bench:      14,108 ns/iter (+/- 1,804)
//

struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut hm = std::collections::HashMap::with_capacity(128);
        let (mut max, mut start) = (0, 0);
        for (i, ch) in s.char_indices() {
            hm.entry(ch)
                .and_modify(|old| {
                    if *old >= start {
                        max = max.max(i - start);
                        start = *old + 1;
                    }
                    *old = i;
                })
                .or_insert(i);
        }
        max.max(s.len() - start) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(Solution::length_of_longest_substring("abcabcbb".into()), 3);
        assert_eq!(Solution::length_of_longest_substring("bbbbb".into()), 1);
        assert_eq!(Solution::length_of_longest_substring("pwwkew".into()), 3);
        assert_eq!(Solution::length_of_longest_substring("".into()), 0);
    }

    #[bench]
    fn vec_1k(b: &mut Bencher) {
        let mut s = String::with_capacity(1_000);
        for i in 0..1_000 / 5 {
            s.push('a');
            s.push('b');
            s.push('c');
            s.push('d');
            s.push('e');
        }
        test::black_box(&s);
        b.iter(|| Solution::length_of_longest_substring(s.clone()));
    }
}
