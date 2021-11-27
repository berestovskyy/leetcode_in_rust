//
// Problem 922. Sort Array By Parity II (Easy)
// See [on LeetCode](https://leetcode.com/problems/sort-array-by-parity-ii/).
//
// 8ms (98%)/2.2MB (88%)
// Space complexity: O(1)
// Runtime complexity: O(n)
//
// test problem_0922::tests::vec_1k                   ... bench:         728 ns/iter (+/- 32)
//

struct Solution {}

impl Solution {
    pub fn sort_array_by_parity_ii(mut nums: Vec<i32>) -> Vec<i32> {
        let (mut i_even, mut i_odd) = (0, 1);
        while i_even < nums.len() && i_odd < nums.len() {
            if nums[i_even] & 1 == 0 {
                i_even += 2;
            } else if nums[i_odd] & 1 != 0 {
                i_odd += 2;
            } else {
                nums.swap(i_even, i_odd);
                i_even += 2;
                i_odd += 2;
            }
        }
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(
            Solution::sort_array_by_parity_ii(vec![4, 2, 5, 7]),
            vec![4, 5, 2, 7]
        );
        assert_eq!(Solution::sort_array_by_parity_ii(vec![2, 3]), vec![2, 3]);
    }

    #[bench]
    fn vec_1k(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        for i in 0..1_000 {
            v.push(i + 1);
        }
        test::black_box(&v);
        b.iter(|| Solution::sort_array_by_parity_ii(v.clone()));
    }
}
