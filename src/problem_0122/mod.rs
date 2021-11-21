//
// Problem 122. Best Time to Buy and Sell Stock II (Medium)
// See [on LeetCode](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/).
//
// 0ms (100%)/2.2MB
// Space complexity: O(1)
// Runtime complexity: O(n)
//
// test problem_0122::tests::vec_1k                   ... bench:         882 ns/iter (+/- 182)
//

struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        (1..prices.len())
            .map(|i| 0.max(prices[i] - prices[i - 1]))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }

    #[bench]
    fn vec_1k(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        for i in 0..1_000 {
            v.push(i % 39);
        }
        test::black_box(&v);
        b.iter(|| Solution::max_profit(v.clone()));
    }
}
