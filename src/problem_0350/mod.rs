//
// Problem 350. Intersection of Two Arrays II (Easy)
// See [on LeetCode](https://leetcode.com/problems/intersection-of-two-arrays-ii/).
//
// 0ms (100%)/2.1MB (80%)
// Space complexity: O(n)
// Runtime complexity: O(n)
//
// test problem_0350::tests::vec_1k                   ... bench:      25,882 ns/iter (+/- 2,527)
//

struct Solution {}

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut h = std::collections::HashMap::with_capacity(nums1.len().min(nums2.len()));
        let mut res = Vec::with_capacity(nums1.len().min(nums2.len()));
        let (smaller, bigger) = if nums1.len() < nums2.len() {
            (nums1, nums2)
        } else {
            (nums2, nums1)
        };
        for i in smaller {
            *h.entry(i).or_insert(0) += 1;
        }
        for i in bigger {
            if let Some(cnt) = h.get_mut(&i) {
                if *cnt > 0 {
                    res.push(i);
                    *cnt -= 1;
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
            Solution::intersect(vec![1, 2, 2, 1], vec![2, 2]),
            vec![2, 2]
        );
        let mut v = Solution::intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4]);
        v.sort_unstable();
        assert_eq!(v, vec![4, 9]);
    }

    #[bench]
    fn vec_1k(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        for i in 0..1_000 / 4 {
            v.push(1);
            v.push(2);
            v.push(3);
            v.push(4);
        }
        test::black_box(&v);
        b.iter(|| Solution::intersect(v.clone(), v.clone()));
    }
}
