//
// Problem 1275. Find Winner on a Tic Tac Toe Game (Easy)
// See [on LeetCode](https://leetcode.com/problems/find-winner-on-a-tic-tac-toe-game/).
//
// 0ms (100%)/2.1MB (%)
// Space complexity: O(1)
// Runtime complexity: O(n)
//
// test problem_1275::tests::string_26                ... bench:         761 ns/iter (+/- 224)
//

struct Solution {}

impl Solution {
    pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
        let mut turn = 1; // A
        let (mut cols, mut rows) = (vec![0_i32; 3], vec![0_i32; 3]);
        let (mut d1, mut d2) = (0, 0);
        for m in &moves {
            let (r, c) = (m[0] as usize, m[1] as usize);
            rows[r] += turn;
            cols[c] += turn;
            if r == c {
                d1 += turn;
            }
            if r + c == 2 {
                d2 += turn;
            }
            if d1
                .max(d2)
                .max(*cols.iter().max().unwrap())
                .max(*rows.iter().max().unwrap())
                == 3
            {
                return "A".into();
            } else if d1
                .min(d2)
                .min(*cols.iter().min().unwrap())
                .min(*rows.iter().min().unwrap())
                == -3
            {
                return "B".into();
            }
            turn = -turn; // A->B
        }
        match moves.len() {
            9 => "Draw".into(),
            _ => "Pending".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(
            Solution::tictactoe(vec![
                vec![0, 0],
                vec![2, 0],
                vec![1, 1],
                vec![2, 1],
                vec![2, 2]
            ]),
            "A"
        );
        assert_eq!(
            Solution::tictactoe(vec![
                vec![0, 0],
                vec![1, 1],
                vec![0, 1],
                vec![0, 2],
                vec![1, 0],
                vec![2, 0]
            ]),
            "B"
        );
        assert_eq!(
            Solution::tictactoe(vec![
                vec![0, 0],
                vec![1, 1],
                vec![2, 0],
                vec![1, 0],
                vec![1, 2],
                vec![2, 1],
                vec![0, 1],
                vec![0, 2],
                vec![2, 2]
            ]),
            "Draw"
        );
        assert_eq!(Solution::tictactoe(vec![vec![0, 0], vec![1, 1]]), "Pending");
    }

    #[bench]
    fn string_26(b: &mut Bencher) {
        b.iter(|| {
            Solution::tictactoe(vec![
                vec![0, 0],
                vec![1, 1],
                vec![2, 0],
                vec![1, 0],
                vec![1, 2],
                vec![2, 1],
                vec![0, 1],
                vec![0, 2],
                vec![2, 2],
            ])
        });
    }
}
