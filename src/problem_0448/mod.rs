//
// Problem 448. Find All Numbers Disappeared in an Array (Easy)
// See [on LeetCode](https://leetcode.com/problems/find-all-numbers-disappeared-in-an-array/).
//
// 12ms (90%)/2.6MB (70%)
// Space complexity: O(1)
// Runtime complexity: O(n)
//
// test problem_0448::tests::vec_1k                   ... bench:       1,630 ns/iter (+/- 511)
//

struct Solution {}

impl Solution {
    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 0..nums.len() {
            let n_idx = nums[i].abs() as usize - 1;
            nums[n_idx] = nums[n_idx].min(-nums[n_idx]);
        }
        let mut res = vec![];
        for (i, n) in nums.iter().enumerate() {
            if *n > 0 {
                res.push(i as i32 + 1);
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(
            Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1]),
            vec![5, 6]
        );
        assert_eq!(Solution::find_disappeared_numbers(vec![1, 1]), vec![2]);
    }

    #[bench]
    fn vec_1k(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        for i in 1..1_000 - 1 {
            v.push(i);
        }
        v.push(1);
        v.push(1);
        test::black_box(&v);
        b.iter(|| Solution::find_disappeared_numbers(v.clone()));
    }
}
