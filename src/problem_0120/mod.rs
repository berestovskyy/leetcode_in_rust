//
// Problem 120. Triangle (Medium)
// https://leetcode.com/problems/triangle/
//
// 0ms (100%)/2.2MB (47%)
// Space complexity: O(1)
// Runtime complexity: O(n)
//
// test problem_0120::tests::vec_1k                   ... bench:     768,087 ns/iter (+/- 106,083)
//

struct Solution {}

impl Solution {
    pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
        for r in 1..triangle.len() {
            triangle[r][0] += triangle[r - 1][0];
            triangle[r][r] += triangle[r - 1][r - 1];
            for c in 1..triangle[r].len() - 1 {
                triangle[r][c] += triangle[r - 1][c - 1].min(triangle[r - 1][c])
            }
        }
        *triangle.last().unwrap().iter().min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(
            Solution::minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]]),
            11
        );
        assert_eq!(Solution::minimum_total(vec![vec![-10]]), -10);
    }

    #[bench]
    fn vec_1k(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        let mut r = vec![];
        for i in 0..1_000 {
            r.push(i % 39);
            v.push(r.clone());
        }
        test::black_box(&v);
        b.iter(|| Solution::minimum_total(v.clone()));
    }
}
