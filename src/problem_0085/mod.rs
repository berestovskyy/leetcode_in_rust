//
// Problem 85. Maximal Rectangle (Hard)
// See [on LeetCode](https://leetcode.com/problems/maximal-rectangle/).
//
// 8ms (95%)/4.9MB (53%)
// Space complexity: O(n)
// Runtime complexity: O(n²)?
//
// test problem_0085::tests::vec_1k                   ... bench:      27,007 ns/iter (+/- 2,808)
//

struct Solution {}

// Based on: https://leetcode.com/problems/maximal-rectangle/discuss/224579/Rust-20ms-solution
impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() {
            return 0;
        }
        let (n, m) = (matrix.len(), matrix[0].len());
        let mut dp = vec![vec![0; m]; n];
        let mut max_area = 0;
        for r in 0..n {
            for c in 0..m {
                if matrix[r][c] == '0' {
                    continue;
                }
                dp[r][c] += if r > 0 { dp[r - 1][c] + 1 } else { 1 };
                let mut min = dp[r][c];
                for k in (0..=c).rev() {
                    if dp[r][k] == 0 {
                        break;
                    }
                    min = dp[r][k].min(min);
                    max_area = max_area.max(min * (c - k + 1));
                }
            }
        }
        max_area as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(
            Solution::maximal_rectangle(vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '0', '1', '0']
            ]),
            6
        );
        assert_eq!(Solution::maximal_rectangle(vec![]), 0);
        assert_eq!(Solution::maximal_rectangle(vec![vec!['0']]), 0);
        assert_eq!(Solution::maximal_rectangle(vec![vec!['1']]), 1);
        assert_eq!(Solution::maximal_rectangle(vec![vec!['0', '0']]), 0);
    }

    #[bench]
    fn vec_1k(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        for i in 0..1_000 / 5 {
            v.push(vec!['1', '0', '1', '0', '1']);
        }
        b.iter(|| Solution::maximal_rectangle(v.clone()));
    }
}
