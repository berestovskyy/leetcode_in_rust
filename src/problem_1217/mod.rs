//
// Problem 1217. Minimum Cost to Move Chips to The Same Position (Easy)
// https://leetcode.com/problems/minimum-cost-to-move-chips-to-the-same-position/
//
// 0ms (100%)/2MB (100%)
// Space complexity: O(1)
// Runtime complexity: O(n)
//
// test problem_1217::tests::vec_1k                   ... bench:         997 ns/iter (+/- 78)
//

struct Solution {}

impl Solution {
    pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
        position
            .iter()
            .fold([0, 0], |[even, odd], n| {
                if n & 1 == 0 {
                    [even + 1, odd]
                } else {
                    [even, odd + 1]
                }
            })
            .iter()
            .cloned()
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(Solution::min_cost_to_move_chips(vec![1, 2, 3]), 1);
        assert_eq!(Solution::min_cost_to_move_chips(vec![2, 2, 2, 3, 3]), 2);
        assert_eq!(Solution::min_cost_to_move_chips(vec![1, 1000000000]), 1);
    }

    #[bench]
    fn vec_1k(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        for i in 0..1_000 {
            v.push(i);
        }
        b.iter(|| Solution::min_cost_to_move_chips(v.clone()));
    }
}
