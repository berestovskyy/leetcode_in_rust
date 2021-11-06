//
// Problem 260. Single Number III (Medium)
// https://leetcode.com/problems/single-number-iii/
//
// 0ms (100%)/2.1MB (100%)
// Space complexity: O(1)
// Runtime complexity: O(n)
//
// test problem_0260::tests::vec_1k                   ... bench:         350 ns/iter (+/- 38)
//

struct Solution {
    pub bad_version: i32,
}

// First we get the two unique numbers xor-ed (n1_xor_n2).
// Any single non-zero bit belongs either to n1 or to n2, so we use it
// as a mask to separate numbers in to two lists.
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut n1_xor_n2 = 0;
        for n in nums.iter() {
            n1_xor_n2 ^= n;
        }
        let mask = n1_xor_n2 & (1 << n1_xor_n2.trailing_zeros());
        let (mut n1, mut n2) = (0, 0);
        for n in nums.iter() {
            if n & mask == 0 {
                n1 ^= n;
            } else {
                n2 ^= n;
            }
        }
        vec![n1, n2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(Solution::single_number(vec![1, 2, 1, 3, 2, 5]), vec![5, 3]);
    }

    #[bench]
    fn vec_1k(b: &mut Bencher) {
        let mut v = vec![];
        for i in 0..1_000 / 2 - 2 {
            v.push(i);
            v.push(i);
        }
        v.push(1000);
        v.push(1001);
        test::black_box(&v);
        b.iter(|| Solution::single_number(v.clone()));
    }
}
