//
// Problem 978. Longest Turbulent Subarray (Medium)
// See [on LeetCode](https://leetcode.com/problems/longest-turbulent-subarray/).
//
// 8ms (50%)/2.6MB (50%)
// Space complexity: O(1)
// Runtime complexity: O(n)
//
// test problem_0978::tests::vec_1k                   ... bench:       1,199 ns/iter (+/- 91)
//

struct Solution {}

impl Solution {
    pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
        let (mut cur_sz, mut max_sz, mut prev_sig) = (0, 0, 0);
        for e in arr.windows(2) {
            let cur_sig = (e[0] - e[1]).signum();
            if cur_sig == 0 {
                cur_sz = 0;
            } else if cur_sig == prev_sig {
                cur_sz = 1;
            } else {
                cur_sz += 1;
            }
            max_sz = max_sz.max(cur_sz);
            prev_sig = cur_sig;
        }
        max_sz.max(cur_sz) + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(
            Solution::max_turbulence_size(vec![9, 4, 2, 10, 7, 8, 8, 1, 9]),
            5
        );
        assert_eq!(Solution::max_turbulence_size(vec![4, 8, 12, 16]), 2);
        assert_eq!(Solution::max_turbulence_size(vec![100]), 1);
    }

    #[bench]
    fn vec_1k(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        for i in 0..1_000 {
            v.push(i);
        }
        test::black_box(&v);
        b.iter(|| Solution::max_turbulence_size(v.clone()));
    }
}
