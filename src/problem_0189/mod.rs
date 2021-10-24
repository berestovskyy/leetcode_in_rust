//
// Problem 189. Rotate Array (Medium)
// https://leetcode.com/problems/rotate-array/
//
// 12ms/3.6MB
// Space complexity: O(n)
// Runtime complexity: O(n)
//
// test problem_0189::tests::rotate_1k            ... bench:       6,832 ns/iter (+/- 1,267)
//

struct Solution {}

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k = k as usize % nums.len();
        let mut changes = 0;
        for round in 0..k {
            // it should take no more than k rounds
            let mut tmp = nums[round];
            let mut dst = round;
            loop {
                dst = (dst + k) % nums.len();
                let t = nums[dst];
                nums[dst] = tmp;
                tmp = t;
                changes += 1;
                if dst == round {
                    break;
                }
            }
            if changes == nums.len() {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn all() {
        let mut v = vec![1, 2, 3, 4, 5, 6, 7];
        Solution::rotate(&mut v, 3);
        assert_eq!(v, vec![5, 6, 7, 1, 2, 3, 4]);
        let mut v = vec![-1, -100, 3, 99];
        Solution::rotate(&mut v, 2);
        assert_eq!(v, vec![3, 99, -1, -100]);
    }

    #[bench]
    fn rotate_1k(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        for i in 0..1_000 {
            v.push(i);
        }
        test::black_box(&v);
        b.iter(|| Solution::rotate(&mut v.clone(), 500));
    }
}
