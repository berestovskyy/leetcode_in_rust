//
// Problem 53. Maximum Subarray (Easy)
// See [on LeetCode](https://leetcode.com/problems/maximum-subarray/).
//
// 7ms (96%)/3.4MB (9%)
// Space complexity: O(1)
// Runtime complexity: O(n)
//
// test problem_0053::tests::vec_1k                   ... bench:       1,049 ns/iter (+/- 183)
//

struct Solution {}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let (mut m, mut max) = (nums[0], nums[0]);
        for n in nums.iter().skip(1).cloned() {
            if m < 0 {
                m = n;
            } else {
                m += n;
            }
            max = max.max(m);
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
        assert_eq!(Solution::max_sub_array(vec![1]), 1);
        assert_eq!(Solution::max_sub_array(vec![5, 4, -1, 7, 8]), 23);
    }

    #[bench]
    fn vec_1k(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        for i in -500..500 {
            v.push(i);
        }
        test::black_box(&v);
        b.iter(|| Solution::max_sub_array(v.clone()));
    }
}
