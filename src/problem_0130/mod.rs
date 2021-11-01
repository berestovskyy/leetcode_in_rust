//
// Problem 130. Surrounded Regions (Medium)
// https://leetcode.com/problems/surrounded-regions/
//
// 8ms (100%)/4.6MB (95%)
// Space complexity: O(1)
// Runtime complexity: O(n)
//
// test problem_0130::tests::vec_1k          ... bench:       5,210 ns/iter (+/- 427)
//

struct Solution {}

impl Solution {
    pub fn solve(mut board: &mut Vec<Vec<char>>) {
        fn dfs(mut board: &mut Vec<Vec<char>>, r: usize, c: usize) {
            let (m, n) = (board.len(), board[0].len());
            if r >= m || c >= n || board[r][c] == 'X' || board[r][c] == 'o' {
                return;
            }
            board[r][c] = 'o'; // visited
            dfs(board, r + 1, c);
            dfs(board, r.overflowing_sub(1).0, c);
            dfs(board, r, c + 1);
            dfs(board, r, c.overflowing_sub(1).0);
        }
        let (m, n) = (board.len(), board[0].len());
        for r in 0..m {
            for c in 0..n {
                if c == 0 || r == 0 || r == m - 1 || c == n - 1 {
                    dfs(board, r, c);
                }
            }
        }
        for r in board {
            for c in r {
                match c {
                    'o' => *c = 'O',
                    'O' => *c = 'X',
                    _ => {}
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        let mut v = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        Solution::solve(&mut v);
        assert_eq!(
            v,
            vec![
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'O', 'X', 'X'],
            ]
        );
        let mut v = vec![vec!['X']];
        Solution::solve(&mut v);
        assert_eq!(v, vec![vec!['X']]);
    }

    #[bench]
    fn vec_1k(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        for i in 0..1_000 / 20 {
            v.push(vec!['X', 'X', 'O', 'X', 'X']);
            v.push(vec!['X', 'X', 'O', 'O', 'X']);
            v.push(vec!['X', 'X', 'X', 'O', 'X']);
            v.push(vec!['X', 'X', 'X', 'O', 'O']);
        }
        test::black_box(&v);
        b.iter(|| Solution::solve(&mut v));
    }
}
