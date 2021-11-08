//
// Problem 43. Multiply Strings (Medium)
// https://leetcode.com/problems/multiply-strings/
//
// 0ms (100%)/2.1MB (56%)
// Space complexity: O(n)
// Runtime complexity: O(n²)
//
// test problem_0043::tests::single                   ... bench:      13,171 ns/iter (+/- 1,700)
//

struct Solution {}

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let mut res = vec![0; num1.len() + num2.len()];
        for (i1, ch1) in num1.chars().rev().enumerate() {
            for (i2, ch2) in num2.chars().rev().enumerate() {
                let n1 = ch1 as u8 - b'0';
                let n2 = ch2 as u8 - b'0';
                let mul = n1 * n2 + res[i1 + i2];
                res[i1 + i2] = mul % 10;
                res[i1 + i2 + 1] += mul / 10;
            }
        }
        while res.len() > 1 && res.last() == Some(&0) {
            res.pop();
        }
        res.into_iter()
            .rev()
            .map(|d| (d + b'0') as char)
            .collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(Solution::multiply("2".into(), "3".into()), "6");
        assert_eq!(Solution::multiply("123".into(), "456".into()), "56088");
    }

    #[bench]
    fn single(b: &mut Bencher) {
        b.iter(|| {
            Solution::multiply(
                "1234567890123456789012345678901234567890123456789012345678901234567890".into(),
                "1234567890123456789012345678901234567890123456789012345678901234567890".into(),
            )
        });
    }
}
