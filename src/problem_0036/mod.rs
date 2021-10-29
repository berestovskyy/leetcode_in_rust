//
// Problem 36. Valid Sudoku (Medium)
// https://leetcode.com/problems/valid-sudoku/
//
// 4ms (86%)/2.1MB (56%)
// Space complexity: O(1)
// Runtime complexity: O(log n)
//
// test problem_0036::tests::is_valid_sudoku        ... bench:         659 ns/iter (+/- 36)
//

struct Solution {}

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows: [u32; 9] = [0; 9];
        let mut cols: [u32; 9] = [0; 9];
        let mut boxes: [u32; 9] = [0; 9];

        for (cur_row, row) in board.iter().enumerate() {
            for (cur_col, c) in row.iter().enumerate() {
                if let '0'..='9' = *c {
                    let d = *c as u32 - '0' as u32;
                    let mask: u32 = 1 << d;
                    let r: &mut u32;
                    let c: &mut u32;
                    let b: &mut u32;
                    match (cur_row, cur_col) {
                        (row @ 0..=2, col @ 0..=2) => {
                            r = &mut rows[row];
                            c = &mut cols[col];
                            b = &mut boxes[0];
                        }
                        (row @ 0..=2, col @ 3..=5) => {
                            r = &mut rows[row];
                            c = &mut cols[col];
                            b = &mut boxes[1];
                        }
                        (row @ 0..=2, col @ 6..=8) => {
                            r = &mut rows[row];
                            c = &mut cols[col];
                            b = &mut boxes[2];
                        }
                        (row @ 3..=5, col @ 0..=2) => {
                            r = &mut rows[row];
                            c = &mut cols[col];
                            b = &mut boxes[3];
                        }
                        (row @ 3..=5, col @ 3..=5) => {
                            r = &mut rows[row];
                            c = &mut cols[col];
                            b = &mut boxes[4];
                        }
                        (row @ 3..=5, col @ 6..=8) => {
                            r = &mut rows[row];
                            c = &mut cols[col];
                            b = &mut boxes[5];
                        }
                        (row @ 6..=8, col @ 0..=2) => {
                            r = &mut rows[row];
                            c = &mut cols[col];
                            b = &mut boxes[6];
                        }
                        (row @ 6..=8, col @ 3..=5) => {
                            r = &mut rows[row];
                            c = &mut cols[col];
                            b = &mut boxes[7];
                        }
                        (row @ 6..=8, col @ 6..=8) => {
                            r = &mut rows[row];
                            c = &mut cols[col];
                            b = &mut boxes[8];
                        }
                        (_, _) => {
                            r = &mut rows[0];
                            c = &mut cols[0];
                            b = &mut boxes[0];
                        }
                    };
                    if *r & mask != 0 || *c & mask != 0 || *b & mask != 0 {
                        return false;
                    };

                    *r |= mask;
                    *c |= mask;
                    *b |= mask;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(
            Solution::is_valid_sudoku(vec![
                vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
            ]),
            true
        );
        assert_eq!(
            Solution::is_valid_sudoku(vec![
                vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
            ]),
            false
        );
    }

    #[bench]
    fn single(b: &mut Bencher) {
        b.iter(|| {
            Solution::is_valid_sudoku(vec![
                vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
            ])
        });
    }
}
