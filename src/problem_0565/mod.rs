//
// Problem 565. Array Nesting (Medium)
// https://leetcode.com/problems/array-nesting/
//
// 0ms/2.2MB
// Space complexity: O(1)
// Runtime complexity: O(n)
//
// test problem_0565::tests::string_1k       ... bench:       1,471 ns/iter (+/- 178)
//

struct Solution {}

impl Solution {
    pub fn array_nesting(nums: Vec<i32>) -> i32 {
        let mut visited = vec![false; nums.len()];
        (0..nums.len())
            .map(|i| {
                let (mut j, mut cur_len) = (i, 0);
                while !visited[j] {
                    visited[j] = true;
                    j = nums[j] as usize;
                    cur_len += 1;
                }
                cur_len
            })
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(Solution::array_nesting(vec![5, 4, 0, 3, 1, 6, 2]), 4);
        assert_eq!(Solution::array_nesting(vec![0, 1, 2]), 1);
    }

    #[bench]
    fn string_1k(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        for i in 0..1_000 - 3 {
            v.push(i);
        }
        test::black_box(&v);
        b.iter(|| Solution::array_nesting(v.clone()));
    }
}
