//
// Problem 882. Reachable Nodes In Subdivided Graph (Hard)
// https://leetcode.com/problems/reachable-nodes-in-subdivided-graph/
//
// 40ms (43%)/3.4MB (73%)
// Space complexity: O(n)
// Runtime complexity: O(n * log n)?
//
// test problem_0882::tests::vec_1k                   ... bench:     136,463 ns/iter (+/- 19,429)
//

struct Solution {}

impl Solution {
    pub fn reachable_nodes(edges: Vec<Vec<i32>>, max_moves: i32, n: i32) -> i32 {
        use std::collections::{BinaryHeap, HashMap};

        let mut e = HashMap::<i32, HashMap<i32, i32>>::new();
        for v in &edges {
            e.entry(v[0]).or_default().insert(v[1], v[2]);
            e.entry(v[1]).or_default().insert(v[0], v[2]);
        }
        let mut heap = BinaryHeap::<(i32, i32)>::new();
        heap.push((max_moves, 0));
        let mut visited = HashMap::<i32, i32>::new();
        while !heap.is_empty() {
            let x = heap.pop().unwrap();
            let moves = x.0;
            let i = x.1;
            if !visited.contains_key(&i) {
                visited.insert(i, moves);
                for j in e.get(&i).unwrap_or(&HashMap::<i32, i32>::new()).keys() {
                    let moves2 = moves - e.get(&i).unwrap().get(j).unwrap() - 1;
                    if !visited.contains_key(j) && moves2 >= 0 {
                        heap.push((moves2, *j));
                    }
                }
            }
        }
        let mut ans = visited.len() as i32;
        for v in &edges {
            let a = visited.get(&v[0]).unwrap_or(&0);
            let b = visited.get(&v[1]).unwrap_or(&0);
            ans += std::cmp::min(a + b, v[2]);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(
            Solution::reachable_nodes(vec![vec![0, 1, 10], vec![0, 2, 1], vec![1, 2, 2]], 6, 3),
            13
        );
        assert_eq!(
            Solution::reachable_nodes(
                vec![vec![0, 1, 4], vec![1, 2, 6], vec![0, 2, 8], vec![1, 3, 1]],
                10,
                4
            ),
            23
        );
        assert_eq!(
            Solution::reachable_nodes(
                vec![
                    vec![1, 2, 4],
                    vec![1, 4, 5],
                    vec![1, 3, 1],
                    vec![2, 3, 4],
                    vec![3, 4, 5]
                ],
                17,
                5
            ),
            1
        );
    }

    #[bench]
    fn vec_1k(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        for i in 0..1_000 / 5 {
            v.push(vec![1, 2, 4]);
            v.push(vec![1, 4, 5]);
            v.push(vec![1, 3, 1]);
            v.push(vec![2, 3, 4]);
            v.push(vec![3, 4, 5]);
        }
        test::black_box(&v);
        b.iter(|| Solution::reachable_nodes(v.clone(), 17, 5));
    }
}
