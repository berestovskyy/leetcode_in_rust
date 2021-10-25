//
// Problem 167. Two Sum II - Input array is sorted (Easy)
// https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/
//
// 0ms/2.1MB
// Space complexity: O(1)
// Runtime complexity: O(n)
//
// test problem_0167::tests::two_sum_1k                ... bench:       1,228 ns/iter (+/- 34)
//

struct Solution {}

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut l, mut r) = (0, numbers.len() - 1);
        loop {
            match (numbers[l] + numbers[r]).cmp(&target) {
                std::cmp::Ordering::Greater => r -= 1,
                std::cmp::Ordering::Equal => break,
                std::cmp::Ordering::Less => l += 1,
            }
        }
        vec![l as i32 + 1, r as i32 + 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn all() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![2, 3, 4], 6), vec![1, 3]);
        assert_eq!(Solution::two_sum(vec![-1, 0], -1), vec![1, 2]);
    }

    #[bench]
    fn two_sum_1k(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        for i in -500..500 {
            v.push(i);
        }
        test::black_box(&v);
        b.iter(|| Solution::two_sum(v.clone(), 998));
    }
}
