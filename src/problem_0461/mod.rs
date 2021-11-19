//
// Problem 461. Hamming Distance (Easy)
// See [on LeetCode](https://leetcode.com/problems/hamming-distance/).
//
// 0ms (100%)/1.9MB (88%)
// Space complexity: O(1)
// Runtime complexity: O(1)
//
// test problem_0461::tests::vec_1k                   ... bench:         587 ns/iter (+/- 214)
//

struct Solution {}

impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        (x ^ y).count_ones() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(Solution::hamming_distance(1, 4), 2);
        assert_eq!(Solution::hamming_distance(3, 1), 1);
    }

    #[bench]
    fn vec_1k(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        for i in 0..1_000 {
            v.push(i);
        }
        test::black_box(&v);
        b.iter(|| {
            for i in 1..v.len() {
                Solution::hamming_distance(v[i - 1], v[i]);
            }
        });
    }
}
