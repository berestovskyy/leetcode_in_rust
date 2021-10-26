//
// Problem 344. Reverse String (Easy)
// https://leetcode.com/problems/reverse-string/
//
// 16ms/5.4MB
// Space complexity: O(1)
// Runtime complexity: avg: O(n)
//
// test problem_0344::tests::reverse_string_1k         ... bench:         508 ns/iter (+/- 164)
//

struct Solution {}

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let (mut l, mut r) = (0, s.len() - 1);
        while l < r {
            s.swap(l, r);
            l += 1;
            r -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn all() {
        let mut v = vec!['h', 'e', 'l', 'l', 'o'];
        Solution::reverse_string(&mut v);
        assert_eq!(v, vec!['o','l','l','e','h']);
        let mut v = vec!['H','a','n','n','a','h'];
        Solution::reverse_string(&mut v);
        assert_eq!(v, vec!['h','a','n','n','a','H']);
    }

    #[bench]
    fn reverse_string_1k(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        for i in 0..1_000/4 {
            v.push('a');
            v.push('b');
            v.push('c');
            v.push('d');
        }
        test::black_box(&v);
        b.iter(|| Solution::reverse_string(&mut v));
    }
}
