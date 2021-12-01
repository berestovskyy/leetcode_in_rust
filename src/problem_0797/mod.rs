//
// Problem 797. All Paths From Source to Target (Medium)
// See [on LeetCode](https://leetcode.com/problems/all-paths-from-source-to-target/).
//
// 8ms (100%)/3MB (13%)
// Space complexity: O(n)
// Runtime complexity: O(2^n)
//
// test problem_0797::tests::vec_15                   ... bench:       5,742 ns/iter (+/- 488)
//

struct Solution {}

impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn dfs(g: &[Vec<i32>], p: Vec<i32>, r: &mut Vec<Vec<i32>>, i: i32, t: i32) {
            match i == t {
                true => r.push(p),
                false => {
                    for e in &g[i as usize] {
                        let mut tmp = p.clone();
                        tmp.push(*e);
                        dfs(g, tmp, r, *e, t);
                    }
                }
            }
        }
        let mut res = vec![];
        dfs(&graph, vec![0], &mut res, 0, graph.len() as i32 - 1);
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
            Solution::all_paths_source_target(vec![vec![1, 2], vec![3], vec![3], vec![]]),
            vec![vec![0, 1, 3], vec![0, 2, 3]]
        );
        assert_eq!(
            Solution::all_paths_source_target(vec![
                vec![4, 3, 1],
                vec![3, 2, 4],
                vec![3],
                vec![4],
                vec![]
            ]),
            vec![
                vec![0, 4],
                vec![0, 3, 4],
                vec![0, 1, 3, 4],
                vec![0, 1, 2, 3, 4],
                vec![0, 1, 4]
            ]
        );
        assert_eq!(
            Solution::all_paths_source_target(vec![vec![1], vec![]]),
            vec![vec![0, 1]]
        );
        assert_eq!(
            Solution::all_paths_source_target(vec![vec![1, 2, 3], vec![2], vec![3], vec![]]),
            vec![vec![0, 1, 2, 3], vec![0, 2, 3], vec![0, 3]]
        );
        assert_eq!(
            Solution::all_paths_source_target(vec![vec![1, 3], vec![2], vec![3], vec![]]),
            vec![vec![0, 1, 2, 3], vec![0, 3]]
        );
    }

    #[bench]
    fn vec_15(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        v.push(vec![1, 3]);
        v.push(vec![2]);
        v.push(vec![3]);
        v.push(vec![4]);
        v.push(vec![5]);
        v.push(vec![6]);
        v.push(vec![7]);
        v.push(vec![8]);
        v.push(vec![9, 11]);
        v.push(vec![10]);
        v.push(vec![11]);
        v.push(vec![12]);
        v.push(vec![13]);
        v.push(vec![14]);
        v.push(vec![0]);
        b.iter(|| Solution::all_paths_source_target(v.clone()));
    }
}
