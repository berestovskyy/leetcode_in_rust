//
// Problem 557. Reverse Words in a String III (Easy)
// https://leetcode.com/problems/reverse-words-in-a-string-iii/
//
// 0ms (100%)/2.2MB
// Space complexity: O(n)
// Runtime complexity: O(n)
//
// test problem_0557::tests::reverse_words_1k         ... bench:       2,937 ns/iter (+/- 355)
//

struct Solution {}

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut res = String::with_capacity(s.len());
        for s in s.split(' ') {
            for ch in s.chars().rev() {
                res.push(ch);
            }
            res.push(' ');
        }
        res.pop();
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
            Solution::reverse_words("Let's take LeetCode contest".into()),
            "s'teL ekat edoCteeL tsetnoc"
        );
        assert_eq!(Solution::reverse_words("God Ding".into()), "doG gniD");
    }

    #[bench]
    fn string_1k(b: &mut Bencher) {
        let mut s = String::with_capacity(1_000);
        for i in 0..1_000 / 5 {
            s.push('a');
            s.push('b');
            s.push('c');
            s.push('d');
            s.push(' ');
        }
        test::black_box(&s);
        b.iter(|| Solution::reverse_words(s.clone()));
    }
}
