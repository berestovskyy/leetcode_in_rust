//
// Problem 283. Move Zeroes (Easy)
// https://leetcode.com/problems/move-zeroes/
//
// 8ms/2.1MB
// Space complexity: O(1)
// Runtime complexity: O(n)
//
// test problem_0283::tests::move_zeros_1k             ... bench:       1,563 ns/iter (+/- 18)
//

struct Solution {}

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut non_zero = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums.swap(i, non_zero);
                non_zero += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        let mut v = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut v);
        assert_eq!(v, vec![1, 3, 12, 0, 0]);

        let mut v = vec![0];
        Solution::move_zeroes(&mut v);
        assert_eq!(v, vec![0]);
    }

    #[bench]
    fn vec_1k(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        for i in -500..500 {
            v.push(i);
        }
        test::black_box(&v);
        b.iter(|| Solution::move_zeroes(&mut v));
    }
}
