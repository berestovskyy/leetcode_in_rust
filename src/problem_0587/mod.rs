//
// Problem 587. Erect the Fence (Hard)
// https://leetcode.com/problems/erect-the-fence/
//
// 4ms/2.3MB
// Space complexity: O(n)
// Runtime complexity: O(n²)?
//
// test problem_0587::tests::vec_1k          ... bench:     123,807 ns/iter (+/- 11,083)
//

struct Solution {}

impl Solution {
    pub fn outer_trees(trees: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn check_convex(trees: &[Vec<i32>], i1: usize, i2: usize, i3: usize) -> i32 {
            let (p1, p2, p3) = (&trees[i1], &trees[i2], &trees[i3]);
            (p2[1] - p1[1]) * (p3[0] - p2[0]) - (p2[0] - p1[0]) * (p3[1] - p2[1])
        }
        let mut trees = trees;
        trees.sort_by(|p1, p2| p1[0].cmp(&p2[0]).then(p1[1].cmp(&p2[1])));

        let mut hull = Vec::with_capacity(trees.len());
        for i in (0..trees.len()) {
            while hull.len() >= 2
                && check_convex(&trees, hull[hull.len() - 2], hull[hull.len() - 1], i) > 0
            {
                hull.pop();
            }
            hull.push(i);
        }
        for i in (0..trees.len()).rev() {
            while hull.len() >= 2
                && check_convex(&trees, hull[hull.len() - 2], hull[hull.len() - 1], i) > 0
            {
                hull.pop();
            }
            hull.push(i);
        }

        use std::collections::HashSet;
        let mut hull_set = HashSet::with_capacity(trees.len());
        let mut ret = Vec::with_capacity(trees.len());
        for i in hull {
            if (hull_set.insert(i)) {
                ret.push(trees[i].clone());
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        let mut v = Solution::outer_trees(vec![
            vec![1, 1],
            vec![2, 2],
            vec![2, 0],
            vec![2, 4],
            vec![3, 3],
            vec![4, 2],
        ]);
        v.sort_unstable();
        assert_eq!(
            v,
            vec![vec![1, 1], vec![2, 0], vec![2, 4], vec![3, 3], vec![4, 2]]
        );
    }

    #[bench]
    fn vec_1k(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        for i in 0..1_000 / 5 {
            v.push(vec![i, 1]);
            v.push(vec![i, 2 + i]);
            v.push(vec![i, 3 + i]);
            v.push(vec![i, 4]);
            v.push(vec![i, 5 + i]);
        }
        test::black_box(&v);
        b.iter(|| Solution::outer_trees(v.clone()));
    }
}
