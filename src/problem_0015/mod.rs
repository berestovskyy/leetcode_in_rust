//
// Problem 15. 3Sum (Medium)
// https://leetcode.com/problems/3sum/
//
// 20ms/3.5MB
// Space complexity: O(1)
// Runtime complexity: O(n * log n)
//
// test problem_0015::tests::vec_1k          ... bench:   8,223,859 ns/iter (+/- 2,018,213)
//

struct Solution {}

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return vec![];
        }
        let mut res = Vec::with_capacity(32768);
        nums.sort_unstable();
        let mut prev_k = i32::MIN;
        for k in (2..nums.len()).rev() {
            if nums[k] == prev_k {
                continue;
            }
            prev_k = nums[k];
            let (mut i, mut j) = (0, k - 1);
            while i < j {
                if nums[i] > 0 || nums[k] < 0 {
                    break;
                }
                let third = 0 - (nums[i] + nums[j]);
                match nums[k].cmp(&third) {
                    std::cmp::Ordering::Equal => {
                        res.push(vec![nums[i], nums[j], nums[k]]);
                        i += 1;
                        j -= 1;
                        while i < j && nums[i] == nums[i - 1] {
                            i += 1;
                        }
                        while i < j && nums[j] == nums[j + 1] {
                            j -= 1;
                        }
                    }
                    std::cmp::Ordering::Less => i += 1,
                    std::cmp::Ordering::Greater => j -= 1,
                }
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
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            [[-1, -1, 2], [-1, 0, 1]]
        );
        assert_eq!(Solution::three_sum(vec![]), Vec::<Vec<i32>>::new());
        assert_eq!(Solution::three_sum(vec![0]), Vec::<Vec<i32>>::new());
    }

    #[bench]
    fn vec_1k(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        for i in -500..500 {
            v.push(i);
        }
        test::black_box(&v);
        b.iter(|| Solution::three_sum(v.clone()));
    }
}
