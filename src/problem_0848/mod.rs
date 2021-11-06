//
// Problem 848. Shifting Letters (Medium)
// https://leetcode.com/problems/shifting-letters/
//
// 12ms (93%)/3.2MB (62%)
// Space complexity: O(n)
// Runtime complexity: O(n)
//
// test problem_0848::tests::string_1k                ... bench:       6,095 ns/iter (+/- 710)
//

struct Solution {}

impl Solution {
    pub fn shifting_letters(s: String, mut shifts: Vec<i32>) -> String {
        shifts.push(0);
        for i in (0..shifts.len() - 1).rev() {
            shifts[i] = (shifts[i] + shifts[i + 1]) % 26;
        }
        let mut res = String::with_capacity(s.len());
        for (c, sh) in s.chars().zip(shifts) {
            res.push(((c as u8 - b'a' + sh as u8) % 26 + b'a').into());
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
            Solution::shifting_letters("abc".into(), vec![3, 5, 9]),
            "rpl"
        );
        assert_eq!(
            Solution::shifting_letters("aaa".into(), vec![1, 2, 3]),
            "gfd"
        );
    }

    #[bench]
    fn string_1k(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        let mut s = String::with_capacity(1_000);
        for i in 0..1_000 {
            v.push(i);
            s.push('a');
        }
        test::black_box(&v);
        test::black_box(&s);
        b.iter(|| Solution::shifting_letters(s.clone(), v.clone()));
    }
}
