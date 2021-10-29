//
// Problem 977. Squares of a Sorted Array (Easy)
// https://leetcode.com/problems/squares-of-a-sorted-array/
//
// 8ms (96%)/2.3MB (40%)
// Space complexity: O(n)
// Runtime complexity: O(n)
//
// test problem_0977::tests::sorted_squares_1k    ... bench:       1,313 ns/iter (+/- 82)
//

struct Solution {}

impl Solution {
    pub fn sorted_squares(mut nums: Vec<i32>) -> Vec<i32> {
        let (mut l, mut r) = (0, nums.len() - 1);
        let mut res = vec![0; nums.len()];
        for i in (0..nums.len()).rev() {
            if nums[l].abs() > nums[r].abs() {
                res[i] = nums[l].pow(2);
                l += 1;
            } else {
                res[i] = nums[r].pow(2);
                r -= 1;
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
            Solution::sorted_squares(vec![-4, -1, 0, 3, 10]),
            vec![0, 1, 9, 16, 100]
        );
        assert_eq!(
            Solution::sorted_squares(vec![-7, -3, 2, 3, 11]),
            vec![4, 9, 9, 49, 121]
        );
    }

    #[bench]
    fn vec_1k(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        for i in -500..500 {
            v.push(i);
        }
        test::black_box(&v);
        b.iter(|| Solution::sorted_squares(v.clone()));
    }
}
