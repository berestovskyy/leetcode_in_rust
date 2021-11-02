//
// Problem 77. Combinations (Medium)
// https://leetcode.com/problems/combinations/
//
// 4ms (100%)/3.2MB (17%)
// Space complexity: O(1)
// Runtime complexity: O(n²)?
//
// test problem_0077::tests::single_4_2      ... bench:      76,076 ns/iter (+/- 6,989)
//

struct Solution {}

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        if k == 0 {
            return vec![vec![]];
        }
        let mut ret = vec![];
        for i in k..=n {
            for mut v in Self::combine(i - 1, k - 1) {
                v.push(i);
                ret.push(v);
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(
            Solution::combine(4, 2),
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![2, 3],
                vec![1, 4],
                vec![2, 4],
                vec![3, 4]
            ]
        );
        assert_eq!(Solution::combine(1, 1), vec![vec![1],]);
    }

    #[bench]
    fn single_4_2(b: &mut Bencher) {
        b.iter(|| {
            let (n, k) = (1000, 1000);
            test::black_box(&n);
            test::black_box(&k);
            let v = Solution::combine(n, k);
        });
    }
}
