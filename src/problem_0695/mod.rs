//
// Problem 695. Max Area of Island (Medium)
// https://leetcode.com/problems/max-area-of-island/
//
// 0ms (100%)/2.2MB
// Space complexity: O(1)
// Runtime complexity: O(n)
//
// test problem_0695::tests::vec_1k          ... bench:      17,618 ns/iter (+/- 1,253)
//

struct Solution {}

impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let (mut max, m, n) = (0, grid.len(), grid[0].len());
        for r in 0..m {
            for c in 0..n {
                match grid[r][c] {
                    0 => {
                        grid[r][c] = -1;
                    }
                    1 => {
                        let mut count = 0;
                        let mut bfs = vec![(r, c)];
                        while let Some((r, c)) = bfs.pop() {
                            if r >= m || c >= n || grid[r][c] != 1 {
                                continue;
                            }
                            grid[r][c] = -1;
                            count += 1;
                            bfs.push((r + 1, c));
                            bfs.push((r.overflowing_sub(1).0, c));
                            bfs.push((r, c + 1));
                            bfs.push((r, c.overflowing_sub(1).0));
                        }
                        max = max.max(count);
                    }
                    _ => {} // visited
                }
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(
            Solution::max_area_of_island(vec![
                vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
                vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0]
            ]),
            6
        );
    }

    #[bench]
    fn vec_1k(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        for i in 0..1_000 / 10 {
            v.push(vec![1, 1, 0, 2, 2]);
            v.push(vec![0, 1, 1, 1, 1]);
        }
        test::black_box(&v);
        b.iter(|| Solution::max_area_of_island(v.clone()));
    }
}
