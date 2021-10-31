//
// Problem 542. 01 Matrix (Medium)
// https://leetcode.com/problems/01-matrix/
//
// 24ms (100%)/3.3MB (35%)
// Space complexity: O(1)
// Runtime complexity: O(n²)?
//
// test problem_0542::tests::vec_1k          ... bench:      36,124 ns/iter (+/- 2,737)
//

struct Solution {}

impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (r, c) = (mat.len(), mat[0].len());
        let mut answer = vec![vec![i32::MAX; c]; r];
        let mut vd = std::collections::VecDeque::new();
        for (i, row) in mat.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if *col == 0 {
                    answer[i][j] = 0;
                    vd.push_back((i, j));
                }
            }
        }
        while let Some((i, j)) = vd.pop_front() {
            for d in [0, 1, 0, !0, 0].windows(2) {
                let di = i.wrapping_add(d[0]);
                let dj = j.wrapping_add(d[1]);
                if di < r && dj < c && answer[di][dj] > answer[i][j] {
                    answer[di][dj] = answer[i][j] + 1;
                    vd.push_back((di, dj));
                }
            }
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(
            Solution::update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]),
            vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]
        );
    }

    #[bench]
    fn vec_1k(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        for i in 0..1_000 / 10 {
            v.push(vec![0, 0, 0, 0, 0]);
            v.push(vec![0, 0, 1, 0, 0]);
        }
        test::black_box(&v);
        b.iter(|| Solution::update_matrix(v.clone()));
    }
}
