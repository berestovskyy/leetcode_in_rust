//
// Problem 980. Unique Paths III (Hard)
// https://leetcode.com/problems/unique-paths-iii/
//
// 0ms (100%)/2.1MB (23%)
// Space complexity: O(1)
// Runtime complexity: O(n²)?
//
// test problem_0980::tests::vec_1k          ... bench:      61,380 ns/iter (+/- 4,311)
//

struct Solution {}

impl Solution {
    pub fn unique_paths_iii(mut grid: Vec<Vec<i32>>) -> i32 {
        fn dfs(mut grid: &mut Vec<Vec<i32>>, p: (usize, usize), vis: usize, target: usize) -> i32 {
            let (m, n) = (grid.len(), grid[0].len());
            if p.0 >= m || p.1 >= n {
                return 0;
            }
            match grid[p.0][p.1] {
                2 => {
                    if vis == target {
                        1
                    } else {
                        0
                    }
                }
                1 | -1 | 3 => 0,
                _ => {
                    // empty cell
                    let mut res = 0;
                    grid[p.0][p.1] = 3; // vis
                    res += dfs(grid, (p.0 + 1, p.1), vis + 1, target)
                        + dfs(grid, (p.0.overflowing_sub(1).0, p.1), vis + 1, target)
                        + dfs(grid, (p.0, p.1 + 1), vis + 1, target)
                        + dfs(grid, (p.0, p.1.overflowing_sub(1).0), vis + 1, target);
                    grid[p.0][p.1] = 0; // back to empty
                    res
                }
            }
        }
        let (m, n) = (grid.len(), grid[0].len());
        let mut start = (0, 0);
        let mut empty = 0;
        for (r, row) in grid.iter().enumerate() {
            for (c, cell) in row.iter().enumerate() {
                match cell {
                    1 => {
                        start.0 = r;
                        start.1 = c;
                    }
                    0 => empty += 1,
                    _ => {}
                }
            }
        }
        dfs(&mut grid, (start.0 + 1, start.1), 0, empty)
            + dfs(&mut grid, (start.0.overflowing_sub(1).0, start.1), 0, empty)
            + dfs(&mut grid, (start.0, start.1 + 1), 0, empty)
            + dfs(&mut grid, (start.0, start.1.overflowing_sub(1).0), 0, empty)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(
            Solution::unique_paths_iii(vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 2, -1]]),
            2
        );
        assert_eq!(
            Solution::unique_paths_iii(vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 2]]),
            4
        );
        assert_eq!(Solution::unique_paths_iii(vec![vec![0, 1], vec![2, 0]]), 0);
    }

    #[bench]
    fn vec_1k(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        v.push(vec![1, 0, 0, 0, 0]);
        v.push(vec![0, 0, 0, 0, 0]);
        for i in 0..1_000 / 10 - 10 {
            v.push(vec![-1, -1, -1, -1, 0]);
            v.push(vec![-1, -1, -1, -1, 0]);
        }
        v.push(vec![0, 0, 0, 0, 0]);
        v.push(vec![0, 0, 0, 0, 2]);
        test::black_box(&v);
        b.iter(|| Solution::unique_paths_iii(v.clone()));
    }
}
