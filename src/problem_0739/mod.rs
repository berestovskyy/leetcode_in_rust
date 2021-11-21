//
// Problem 739. Daily Temperatures (Medium)
// See [on LeetCode](https://leetcode.com/problems/daily-temperatures/).
//
// 64ms (58%)/3MB (70%)
// Space complexity: O(1)
// Runtime complexity: O(n)
//
// test problem_0739::tests::vec_1k                   ... bench:       3,329 ns/iter (+/- 459)
//

struct Solution {}

impl Solution {
    pub fn daily_temperatures(t: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0; t.len()];
        let mut stack = Vec::new();
        for i in 0..t.len() {
            while stack.last().map_or(false, |&v| t[v] < t[i]) {
                match stack.pop() {
                    None => break,
                    Some(idx) => ans[idx] = (i - idx) as i32,
                }
            }
            stack.push(i);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(
            Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
        assert_eq!(
            Solution::daily_temperatures(vec![30, 40, 50, 60]),
            vec![1, 1, 1, 0]
        );
        assert_eq!(
            Solution::daily_temperatures(vec![30, 60, 90]),
            vec![1, 1, 0]
        );
    }

    #[bench]
    fn vec_1k(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        for i in 0..1_000 {
            v.push(i % 39);
        }
        test::black_box(&v);
        b.iter(|| Solution::daily_temperatures(v.clone()));
    }
}
