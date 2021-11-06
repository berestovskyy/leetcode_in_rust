//
// Problem 764. Largest Plus Sign (Medium)
// https://leetcode.com/problems/largest-plus-sign/
//
// 36ms (82%)/2.7MB (93%)
// Space complexity: O(n)
// Runtime complexity: O(n)
//
// test problem_0764::tests::single_10k               ... bench:     106,792 ns/iter (+/- 9,428)
//

struct Solution {}

impl Solution {
    pub fn order_of_largest_plus_sign(n: i32, mines: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![vec![i16::MAX; n as usize]; n as usize];
        for m in &mines {
            dp[m[0] as usize][m[1] as usize] = i16::MIN;
        }
        for r in dp.iter_mut() {
            let mut cnt = 0;
            for c in r.iter_mut() {
                if *c < 0 {
                    cnt = 0;
                } else {
                    cnt += 1;
                    *c = *c.min(&mut cnt);
                }
            }
            let mut cnt = 0;
            for c in r.iter_mut().rev() {
                if *c < 0 {
                    cnt = 0;
                } else {
                    cnt += 1;
                    *c = *c.min(&mut cnt);
                }
            }
        }
        let mut ret = 0;
        for c in 0..n as usize {
            let mut cnt = 0;
            for r in dp.iter_mut() {
                if r[c] < 0 {
                    cnt = 0;
                } else {
                    cnt += 1;
                    r[c] = r[c].min(cnt);
                }
            }
            let mut cnt = 0;
            for r in dp.iter_mut().rev() {
                if r[c] < 0 {
                    cnt = 0;
                } else {
                    cnt += 1;
                    r[c] = r[c].min(cnt);
                }
                ret = r[c].max(ret);
            }
        }
        ret.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(Solution::order_of_largest_plus_sign(5, vec![vec![4, 2]]), 2);
    }

    #[bench]
    fn single_10k(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        for i in 0..1_000 {
            v.push(vec![i % 100, i % 100]);
        }
        test::black_box(&v);
        b.iter(|| Solution::order_of_largest_plus_sign(100, v.clone()));
    }
}
