//
// Problem 952. Largest Component Size by Common Factor (Hard)
// See [on LeetCode](https://leetcode.com/problems/largest-component-size-by-common-factor/).
//
// 180ms (100%)/3.9MB (100%)
// Space complexity: O(n)
// Runtime complexity: O(n³)?
//
// test problem_0952::tests::vec_1k                   ... bench:     183,811 ns/iter (+/- 73,638)
//

struct Solution {}

impl Solution {
    pub fn largest_component_size(nums: Vec<i32>) -> i32 {
        fn find(x: usize, par: &mut Vec<usize>) -> usize {
            if par[x] == x {
                x
            } else {
                par[x] = find(par[x], par);
                par[x]
            }
        }
        fn union(a: i32, b: i32, par: &mut Vec<usize>, rank: &mut Vec<usize>) -> usize {
            let (a, b) = (find(a as usize, par), find(b as usize, par));
            if a == b {
                return 0;
            }
            if rank[a] >= rank[b] {
                par[b] = a;
                rank[a] += rank[b];
            } else {
                rank[b] += rank[a];
                par[a] = b;
            }
            rank[a].max(rank[b])
        }
        let (mut par, mut rank) = (vec![0; 100001], vec![0; 100001]);
        let mut res = 1;
        // `for (i, p) in par.iter_mut().enumerate().skip(1) {` is much slower
        #[allow(clippy::needless_range_loop)]
        for i in 1..par.len() {
            par[i] = i;
        }
        for i in 0..nums.len() {
            rank[nums[i] as usize] = 1;
        }
        for n in &nums {
            let mut j = 2;
            while j * j <= *n {
                if *n % j == 0 {
                    res = union(j, *n, &mut par, &mut rank).max(res);
                    res = union(j, *n / j, &mut par, &mut rank).max(res);
                }
                j += 1;
            }
        }
        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(Solution::largest_component_size(vec![4, 6, 15, 35]), 4);
        assert_eq!(Solution::largest_component_size(vec![20, 50, 9, 63]), 2);
        assert_eq!(
            Solution::largest_component_size(vec![2, 3, 6, 7, 4, 12, 21, 39]),
            8
        );
    }

    #[bench]
    fn vec_1k(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        for i in 0..1_000 {
            v.push(i);
        }
        test::black_box(&v);
        b.iter(|| Solution::largest_component_size(v.clone()));
    }
}
