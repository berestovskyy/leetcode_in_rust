//
// Problem 224. Basic Calculator (Hard)
// https://leetcode.com/problems/basic-calculator/
//
// 0ms (100%)/2.5MB (56%)
// Space complexity: O(n)
// Runtime complexity: O(n)
//
// test problem_0224::tests::string_1k                ... bench:       1,992 ns/iter (+/- 121)
//

struct Solution {}

impl Solution {
    pub fn calculate(s: String) -> i32 {
        fn calc(s: &mut std::str::Chars) -> i32 {
            let mut ret = 0;
            let mut sign = 1;
            let mut n = 0;
            while let Some(ch) = s.next() {
                match ch {
                    '+' | '-' => {
                        ret += sign * n;
                        sign = if ch == '+' { 1 } else { -1 };
                        n = 0;
                    }
                    '0'..='9' => n = n * 10 + (ch as u8 - b'0') as i32,
                    '(' => n = calc(s),
                    ')' => break,
                    _ => {}
                }
            }
            ret + sign * n
        }
        calc(&mut s.chars())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(Solution::calculate("1 + 1".into()), 2);
        assert_eq!(Solution::calculate(" 2-1 + 2 ".into()), 3);
        assert_eq!(Solution::calculate("(1+(4+5+2)-3)+(6+8)".into()), 23);
    }

    #[bench]
    fn string_1k(b: &mut Bencher) {
        let mut s = String::with_capacity(1_000);
        for i in 0..1_000 / 2 {
            s.push('1');
            s.push('+');
        }
        s.push('1');
        b.iter(|| Solution::calculate(s.clone()));
    }
}
