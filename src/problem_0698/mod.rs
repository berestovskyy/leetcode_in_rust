//
// Problem 698. Partition to K Equal Sum Subsets (Medium)
// See [on LeetCode](https://leetcode.com/problems/partition-to-k-equal-sum-subsets/).
//
// 0ms (100%)/2MB (57%)
// Space complexity: O(1)
// Runtime complexity: O(n)
//
// test problem_0698::tests::vec_1k                   ... bench:  26,103,138 ns/iter (+/- 2,522,960)
//

struct Solution {}

impl Solution {
    pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
        let sum = nums.iter().sum::<i32>();
        if sum % k != 0 {
            return false;
        }
        Self::backtrack(&nums, 0, sum / k, 0, &mut vec![false; nums.len()])
    }
    fn backtrack(nums: &[i32], curr: i32, target: i32, i: usize, used: &mut Vec<bool>) -> bool {
        if used.iter().all(|&b| b) {
            return true;
        }
        for j in i..nums.len() {
            if used[j] || curr + nums[j] > target {
                continue;
            }
            let next = (curr + nums[j]) % target;
            used[j] = true;
            if Self::backtrack(nums, next, target, if next == 0 { 0 } else { j + 1 }, used) {
                return true;
            }
            used[j] = false;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(
            Solution::can_partition_k_subsets(vec![4, 3, 2, 3, 5, 2, 1], 4),
            true
        );
        assert_eq!(
            Solution::can_partition_k_subsets(vec![1, 2, 3, 4], 3),
            false
        );
    }

    #[bench]
    fn vec_1k(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        for i in 0..1_000 {
            v.push(i);
        }
        test::black_box(&v);
        b.iter(|| Solution::can_partition_k_subsets(v.clone(), 5));
    }
}
