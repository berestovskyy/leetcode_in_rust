//
// Problem 1629. Slowest Key (Easy)
// https://leetcode.com/problems/slowest-key/
//
// 0ms (100%)/2MB
// Space complexity: O(n)
// Runtime complexity: O(n)
//
// test problem_1629::tests::vec_1k          ... bench:         263 ns/iter (+/- 41)
//

struct Solution {}

impl Solution {
    pub fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char {
        let keys_pressed = keys_pressed.as_bytes();
        let mut longest_time = release_times[0];
        let mut longest_key = keys_pressed[0];

        for i in 1..release_times.len() {
            let t = release_times[i] - release_times[i - 1];
            if t > longest_time || (t == longest_time && keys_pressed[i] > longest_key) {
                longest_time = t;
                longest_key = keys_pressed[i];
            }
        }
        longest_key as char
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(
            Solution::slowest_key(vec![9, 29, 49, 50], "cbcd".into()),
            'c'
        );
    }

    #[bench]
    fn vec_1k(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        let mut s = String::with_capacity(1_000);
        for i in 0..1000 / 10 {
            v.push(i);
            s.push('a');
        }
        test::black_box(&v);
        test::black_box(&s);
        b.iter(|| Solution::slowest_key(v.clone(), s.clone()));
    }
}
