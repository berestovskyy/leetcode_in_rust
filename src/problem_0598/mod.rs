//
// Problem 598. Range Addition II (Easy)
// https://leetcode.com/problems/range-addition-ii/
//
// 0ms/2.5MB
// Space complexity: O(1)
// Runtime complexity: avg: O(n)
//
// test problem_0598::tests::max_count_1k              ... bench:      58,495 ns/iter (+/- 5,314)
//

struct Solution {}

impl Solution {
    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        let (mut min_m, mut min_n) = (m, n);
        for op in &ops {
            if let [op_m, op_n] = op.as_slice() {
                min_m = min_m.min(*op_m);
                min_n = min_n.min(*op_n);
            }
        }
        min_m * min_n
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(Solution::max_count(3, 3, vec![vec![2, 2], vec![3, 3]]), 4);
        assert_eq!(
            Solution::max_count(
                3,
                3,
                vec![
                    vec![2, 2],
                    vec![3, 3],
                    vec![3, 3],
                    vec![3, 3],
                    vec![2, 2],
                    vec![3, 3],
                    vec![3, 3],
                    vec![3, 3],
                    vec![2, 2],
                    vec![3, 3],
                    vec![3, 3],
                    vec![3, 3]
                ]
            ),
            4
        );
    }

    #[bench]
    fn vec_1k(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        for i in 0..1_000 / 10 {
            v.push(vec![2, 2]);
            v.push(vec![3, 3]);
            v.push(vec![3, 3]);
            v.push(vec![3, 3]);
            v.push(vec![2, 2]);
            v.push(vec![3, 3]);
            v.push(vec![3, 3]);
            v.push(vec![3, 3]);
            v.push(vec![2, 2]);
            v.push(vec![3, 3]);
        }
        test::black_box(&v);
        b.iter(|| Solution::max_count(3, 3, v.clone()));
    }
}
