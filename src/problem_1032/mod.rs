//
// Problem 1032. Stream of Characters (Hard)
// See [on LeetCode](https://leetcode.com/problems/stream-of-characters/).
//
// 96ms (91%)/28MB (42%)
// Space complexity: O(n)
// Runtime complexity: O(n * log n)
//
// test problem_1032::tests::single_example           ... bench:         787 ns/iter (+/- 33)
//

struct Solution {}

// Based on: https://leetcode.com/problems/stream-of-characters/discuss/1610905/Rust-Sorted-array-%2B-binary-search-solution
struct StreamChecker {
    dict: Vec<Vec<char>>,
    query: Vec<char>,
}

impl StreamChecker {
    fn new(words: Vec<String>) -> Self {
        let mut dict = words
            .iter()
            .map(|s| s.chars().rev().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        dict.sort();
        StreamChecker {
            dict,
            query: vec![],
        }
    }

    fn query(&mut self, letter: char) -> bool {
        fn prefix_exists(words: &[Vec<char>], char_i: usize, query: &[char]) -> bool {
            let last_c = query[query.len() - 1 - char_i];
            let l = words.partition_point(|x| x[char_i] < last_c);
            let r = words.partition_point(|x| x[char_i] <= last_c);
            if l == r {
                // No words matching the prefix
                false
            } else if words[l..r].iter().any(|w| w.len() <= char_i + 1) {
                // Trs could be done in sub-linear time, but seems to be good enough
                // Prefix matches some words
                true
            } else if query.len() <= char_i + 1 {
                // Query is too short
                false
            } else {
                // So far so good, check the rest of the prefix
                prefix_exists(&words[l..r], char_i + 1, query)
            }
        }
        self.query.push(letter);
        prefix_exists(&self.dict, 0, &self.query)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        let mut sc = StreamChecker::new(vec!["cd".into(), "f".into(), "kl".into()]);
        assert_eq!(sc.query('a'), false);
        assert_eq!(sc.query('b'), false);
        assert_eq!(sc.query('c'), false);
        assert_eq!(sc.query('d'), true);
        assert_eq!(sc.query('e'), false);
        assert_eq!(sc.query('f'), true);
        assert_eq!(sc.query('g'), false);
        assert_eq!(sc.query('h'), false);
        assert_eq!(sc.query('i'), false);
        assert_eq!(sc.query('j'), false);
        assert_eq!(sc.query('k'), false);
        assert_eq!(sc.query('l'), true);
    }

    #[bench]
    fn single_example(b: &mut Bencher) {
        b.iter(|| example());
    }
}
