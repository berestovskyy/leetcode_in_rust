//
// Problem 1178. Number of Valid Words for Each Puzzle (Hard)
// https://leetcode.com/problems/number-of-valid-words-for-each-puzzle/
//
// 32ms (100%)/10.4MB (61%)
// Space complexity: O(n)
// Runtime complexity: O(n²)
//
// test problem_1178::tests::vec_1k                   ... bench:     947,636 ns/iter (+/- 135,344)
//

struct Solution {}

impl Solution {
    pub fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
        let mut hm = std::collections::HashMap::new();
        for w in &words {
            *hm.entry(w.bytes().fold(0_u32, |m, b| m | 1 << (b - b'a')))
                .or_insert(0) += 1;
        }
        puzzles
            .iter()
            .map(|p| {
                let first = 1 << (p.as_bytes()[0] - b'a');
                let mask = p.bytes().skip(1).fold(0, |m, b| m | 1 << (b - b'a'));
                let mut cnt = *hm.get(&first).unwrap_or(&0);
                let mut m = mask;
                while m > 0 {
                    cnt += hm.get(&(m | first)).unwrap_or(&0);
                    m = (m - 1) & mask;
                }
                cnt
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(
            Solution::find_num_of_valid_words(
                vec![
                    "aaaa".into(),
                    "asas".into(),
                    "able".into(),
                    "ability".into(),
                    "actt".into(),
                    "actor".into(),
                    "access".into()
                ],
                vec![
                    "aboveyz".into(),
                    "abrodyz".into(),
                    "abslute".into(),
                    "absoryz".into(),
                    "actresz".into(),
                    "gaswxyz".into()
                ]
            ),
            vec![1, 1, 3, 2, 4, 0]
        );
    }

    #[bench]
    fn vec_1k(b: &mut Bencher) {
        let mut w = Vec::with_capacity(1_000);
        for i in 0..1_000 / 5 {
            w.push("abcde".to_string());
            w.push("fghij".to_string());
            w.push("klmno".to_string());
            w.push("pqrst".to_string());
            w.push("uvwxy".to_string());
        }
        let mut p = Vec::with_capacity(1_000);
        for i in 0..1_000 / 5 {
            p.push("abcdefg".to_string());
            p.push("fghijkl".to_string());
            p.push("klmnopq".to_string());
            p.push("pqrstuv".to_string());
            p.push("uvwxyza".to_string());
        }
        test::black_box(&w);
        test::black_box(&p);
        b.iter(|| Solution::find_num_of_valid_words(w.clone(), p.clone()));
    }
}
