//
// Problem 782. Transform to Chessboard (Hard)
// See [on LeetCode](https://leetcode.com/problems/transform-to-chessboard/).
//
// 0ms (100%)/2.1MB (27%)
// Space complexity: O(1)
// Runtime complexity: O(n)
//
// test problem_0782::tests::vec_28                   ... bench:       2,489 ns/iter (+/- 162)
//

struct Solution {}

impl Solution {
    pub fn moves_to_chessboard(board: Vec<Vec<i32>>) -> i32 {
        let (n, mut r_sum, mut c_sum, mut r_cnt, mut c_cnt) = (board.len(), 0, 0, 0, 0);
        for i in 0..n {
            for j in 0..n {
                if board[0][0] ^ board[i][0] ^ board[0][j] ^ board[i][j] != 0 {
                    return -1;
                }
            }
        }
        for i in 0..n {
            r_sum += board[0][i];
            c_sum += board[i][0];
            r_cnt += if board[i][0] == i as i32 & 1 { 1 } else { 0 };
            c_cnt += if board[0][i] == i as i32 & 1 { 1 } else { 0 };
        }
        if r_sum != n as i32 / 2 && r_sum != (n as i32 + 1) / 2
            || c_sum != n as i32 / 2 && c_sum != (n as i32 + 1) / 2
        {
            return -1;
        }
        if n % 2 != 0 {
            if c_cnt % 2 != 0 {
                c_cnt = n - c_cnt;
            }
            if r_cnt % 2 != 0 {
                r_cnt = n - r_cnt;
            }
        } else {
            c_cnt = c_cnt.min(n - c_cnt);
            r_cnt = r_cnt.min(n - r_cnt);
        }
        (c_cnt + r_cnt) as i32 / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(
            Solution::moves_to_chessboard(vec![
                vec![0, 1, 1, 0],
                vec![0, 1, 1, 0],
                vec![1, 0, 0, 1],
                vec![1, 0, 0, 1]
            ]),
            2
        );
        assert_eq!(
            Solution::moves_to_chessboard(vec![vec![0, 1], vec![1, 0]]),
            0
        );
        assert_eq!(
            Solution::moves_to_chessboard(vec![vec![1, 0], vec![1, 0]]),
            -1
        );
    }

    #[bench]
    fn vec_28(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        for i in 0..7 {
            v.push(vec![
                0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0,
            ]);
            v.push(vec![
                0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0,
            ]);
            v.push(vec![
                1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1,
            ]);
            v.push(vec![
                1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1,
            ]);
        }
        b.iter(|| Solution::moves_to_chessboard(v.clone()));
    }
}
