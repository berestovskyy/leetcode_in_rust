//
// Problem 1293. Shortest Path in a Grid with Obstacles Elimination (Hard)
// https://leetcode.com/problems/shortest-path-in-a-grid-with-obstacles-elimination/
//
// 4ms (100%)/2.1MB (100%)
// Space complexity: O(1)
// Runtime complexity: O(n²)?
//
// test problem_1293::tests::single                   ... bench:       1,173 ns/iter (+/- 39)
//

struct Solution {}

impl Solution {
    pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut q = std::collections::VecDeque::with_capacity(m + n);
        let mut vis = vec![vec![-1; n]; m];
        q.push_back((0, 0, k, 0));
        while !q.is_empty() {
            let (x, y, k, p) = q.pop_front().unwrap();
            if x >= m || y >= n || k <= vis[x][y] {
                continue; // Out of grid, or visited, or can't eliminate an obsticle
            }
            if x == m - 1 && y == n - 1 {
                return p; // Reached the target
            }
            vis[x][y] = if grid[x][y] == 1 { k - 1 } else { k };
            q.push_back((x.overflowing_sub(1).0, y, vis[x][y], p + 1));
            q.push_back((x + 1, y, vis[x][y], p + 1));
            q.push_back((x, y.overflowing_sub(1).0, vis[x][y], p + 1));
            q.push_back((x, y + 1, vis[x][y], p + 1));
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(
            Solution::shortest_path(
                vec![
                    vec![0, 0, 0],
                    vec![1, 1, 0],
                    vec![0, 0, 0],
                    vec![0, 1, 1],
                    vec![0, 0, 0]
                ],
                1
            ),
            6
        );
        assert_eq!(
            Solution::shortest_path(vec![vec![0, 1, 1], vec![1, 1, 1], vec![1, 0, 0]], 1),
            -1
        );
    }

    #[bench]
    fn single(b: &mut Bencher) {
        b.iter(|| {
            Solution::shortest_path(
                vec![
                    vec![0, 0, 0],
                    vec![1, 1, 0],
                    vec![0, 0, 0],
                    vec![0, 1, 1],
                    vec![0, 0, 0],
                ],
                1,
            )
        });
    }
}
