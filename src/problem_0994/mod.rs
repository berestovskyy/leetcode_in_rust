//
// Problem 994. Rotting Oranges (Medium)
// https://leetcode.com/problems/rotting-oranges/
//
// 0ms/2.1MB
// Space complexity: O(n)
// Runtime complexity: O(n)
//
// test problem_0994::tests::vec_1k          ... bench:      17,669 ns/iter (+/- 1,857)
//

struct Solution {}

impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        fn try_make_rotten(
            grid: &mut Vec<Vec<i32>>,
            rotten: &mut std::collections::VecDeque<(usize, usize)>,
            i: usize,
            j: usize,
            m: usize,
            n: usize,
        ) {
            // Thanks to overflow we don't need to check < 0 here
            if i >= m || j >= n {
                return;
            }
            if grid[i][j] == 1 {
                grid[i][j] = 2;
                rotten.push_back((i, j));
            }
        }
        let (m, n) = (grid.len(), grid[0].len());
        let mut rotten = std::collections::VecDeque::new();
        for (i, r) in grid.iter().enumerate() {
            for (j, c) in r.iter().enumerate() {
                if c == &2 {
                    rotten.push_back((i, j));
                }
            }
        }
        let mut minutes = 0;
        while !rotten.is_empty() {
            let mut minute_limit = rotten.len();
            for _ in 0..minute_limit {
                let (i, j) = rotten.pop_front().unwrap();
                try_make_rotten(&mut grid, &mut rotten, i + 1, j, m, n);
                // Overflow saves us one if (see above)
                try_make_rotten(&mut grid, &mut rotten, i.overflowing_sub(1).0, j, m, n);
                try_make_rotten(&mut grid, &mut rotten, i, j + 1, m, n);
                try_make_rotten(&mut grid, &mut rotten, i, j.overflowing_sub(1).0, m, n);
            }
            minutes += if rotten.is_empty() { 0 } else { 1 };
        }
        for (i, r) in grid.iter().enumerate() {
            for (j, c) in r.iter().enumerate() {
                if c == &1 {
                    return -1;
                }
            }
        }
        minutes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(
            Solution::oranges_rotting(vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]]),
            4
        );
        assert_eq!(
            Solution::oranges_rotting(vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]]),
            -1
        );
        assert_eq!(Solution::oranges_rotting(vec![vec![0, 2]]), 0);
    }

    #[bench]
    fn vec_1k(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        for i in 0..1000 / 10 {
            v.push(vec![0, 1, 1, 0, 2]);
            v.push(vec![2, 1, 0, 1, 1]);
        }
        test::black_box(&v);
        b.iter(|| Solution::oranges_rotting(v.clone()));
    }
}
