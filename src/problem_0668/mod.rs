//
// Problem 668. Kth Smallest Number in Multiplication Table (Hard)
// See [on LeetCode](https://leetcode.com/problems/kth-smallest-number-in-multiplication-table/).
//
// 8ms (98%)/2.1MB (62%)
// Space complexity: O(1)
// Runtime complexity: O(n * log n)?
//
// test problem_0668::tests::single_1k                ... bench:      39,428 ns/iter (+/- 3,513)
//

struct Solution {}

impl Solution {
    pub fn find_kth_number(m: i32, n: i32, k: i32) -> i32 {
        let (m, n) = (m.min(n), m.max(n));
        let (mut l, mut r) = (1, n * m);
        while l < r {
            let mid = (l + r) / 2;
            let mut rank = 0;
            for i in 1..m + 1 {
                rank += n.min(mid / i);
            }
            if rank < k {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        l
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(Solution::find_kth_number(3, 3, 5), 3);
        assert_eq!(Solution::find_kth_number(2, 3, 6), 6);
    }

    #[bench]
    fn single_1k(b: &mut Bencher) {
        b.iter(|| Solution::find_kth_number(1_000, 1_000, 1_000));
    }
}
