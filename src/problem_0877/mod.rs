//
// Problem 877. Stone Game (Medium)
// https://leetcode.com/problems/stone-game/
//
// 0ms (100%)/2.1MB
// Space complexity: O(n)
// Runtime complexity: O(n)
//
// test problem_0877::tests::stone_game_1k             ... bench:         835 ns/iter (+/- 22)
//

struct Solution {}

impl Solution {
    pub fn stone_game(piles: Vec<i32>) -> bool {
        fn dfs(piles: &[i32], f: usize, l: usize, alex: i32, lee: i32) -> bool {
            if f > l {
                return alex > lee;
            }
            let first = piles[f];
            let last = piles[l];
            if first > last {
                dfs(piles, f + 1, l - 1, alex + first, lee + last)
            } else {
                dfs(piles, f + 1, l - 1, alex + last, lee + first)
            }
        }

        dfs(&piles, 0, piles.len() - 1, 0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(Solution::stone_game(vec![5, 3, 4, 5]), true);
        assert_eq!(Solution::stone_game(vec![3, 7, 2, 3]), true);
    }

    #[bench]
    fn vec_1k(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        for i in -500..500 {
            v.push(i);
        }
        test::black_box(&v);
        b.iter(|| Solution::stone_game(v.clone()));
    }
}
