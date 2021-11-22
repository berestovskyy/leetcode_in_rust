//
// Problem 54. Spiral Matrix (Medium)
// See [on LeetCode](https://leetcode.com/problems/spiral-matrix/).
//
// 0ms (100%)/2.2MB (36%)
// Space complexity: O(n)
// Runtime complexity: O(n)
//
// test problem_0054::tests::vec_1k                   ... bench:       7,317 ns/iter (+/- 704)
//

struct Solution {}

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let (m, n) = (matrix.len(), matrix[0].len());
        let (mut top, mut left, mut right, mut bottom) = (0, 0, n, m);
        let mut res = Vec::with_capacity(m * n);
        loop {
            if bottom == top {
                break;
            }
            for i in left..right {
                res.push(matrix[top][i]);
            }
            top += 1;
            if right == left {
                break;
            }
            for r in matrix.iter().take(bottom).skip(top) {
                res.push(r[right - 1]);
            }
            right -= 1;
            if bottom == top {
                break;
            }
            for i in (left..right).rev() {
                res.push(matrix[bottom - 1][i]);
            }
            bottom -= 1;
            if right == left {
                break;
            }
            for i in (top..bottom).rev() {
                res.push(matrix[i][left]);
            }
            left += 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(
            Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
        );
        assert_eq!(
            Solution::spiral_order(vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12]
            ]),
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
        );
    }

    #[bench]
    fn vec_1k(b: &mut Bencher) {
        let mut v = Vec::with_capacity(100);
        for i in 0..100 {
            v.push(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        }
        b.iter(|| Solution::spiral_order(v.clone()));
    }
}
