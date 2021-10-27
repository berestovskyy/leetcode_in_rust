//
// Problem 75. Sort Colors (Medium)
// https://leetcode.com/problems/sort-colors/
//
// 0ms/2MB
// Space complexity: O(1)
// Runtime complexity: O(n)
//
// test problem_0075::tests::bench_1k                  ... bench:         746 ns/iter (+/- 150)
//

struct Solution {}

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut r, mut w, mut b) = (0, 0, nums.len());
        while w < b {
            match nums[w] {
                0 => {
                    nums.swap(w, r);
                    r += 1;
                    w += 1;
                }
                2 => {
                    b -= 1;
                    nums.swap(w, b);
                }
                _ => {
                    w += 1;
                }
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
        let mut v = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut v);
        assert_eq!(v, [0, 0, 1, 1, 2, 2]);

        let mut v = vec![2, 0, 1];
        Solution::sort_colors(&mut v);
        assert_eq!(v, [0, 1, 2]);

        let mut v = vec![0];
        Solution::sort_colors(&mut v);
        assert_eq!(v, [0]);

        let mut v = vec![1];
        Solution::sort_colors(&mut v);
        assert_eq!(v, [1]);
    }

    #[test]
    fn extra() {
        let mut v = vec![2];
        Solution::sort_colors(&mut v);
        assert_eq!(v, [2]);
    }

    #[bench]
    fn vec_1k(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        for i in 0..1_000 / 4 {
            v.push(0);
            v.push(1);
            v.push(2);
            v.push(1);
        }
        test::black_box(&v);
        b.iter(|| Solution::sort_colors(&mut v));
    }
}
