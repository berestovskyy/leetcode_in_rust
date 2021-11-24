//
// Problem 986. Interval List Intersections (Medium)
// See [on LeetCode](https://leetcode.com/problems/interval-list-intersections/).
//
// 8ms (42%)/2.3MB (80%)
// Space complexity: O(1)
// Runtime complexity: O(n)
//
// test problem_0986::tests::vec_1k                   ... bench:     227,329 ns/iter (+/- 28,213)
//

struct Solution {}

impl Solution {
    pub fn interval_intersection(
        first_list: Vec<Vec<i32>>,
        second_list: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let (mut i1, mut i2) = (0, 0);
        let mut res = vec![];
        while i1 < first_list.len() && i2 < second_list.len() {
            let start = first_list[i1][0].max(second_list[i2][0]);
            let end = first_list[i1][1].min(second_list[i2][1]);

            if start <= end {
                res.push(vec![start, end]);
            }
            if first_list[i1][1] <= second_list[i2][1] {
                i1 += 1;
            } else {
                i2 += 1;
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
            Solution::interval_intersection(
                vec![vec![0, 2], vec![5, 10], vec![13, 23], vec![24, 25]],
                vec![vec![1, 5], vec![8, 12], vec![15, 24], vec![25, 26]]
            ),
            vec![
                vec![1, 2],
                vec![5, 5],
                vec![8, 10],
                vec![15, 23],
                vec![24, 24],
                vec![25, 25]
            ]
        );
        assert_eq!(
            Solution::interval_intersection(vec![vec![1, 3], vec![5, 9]], vec![]),
            Vec::<Vec<i32>>::new()
        );
        assert_eq!(
            Solution::interval_intersection(vec![], vec![vec![4, 8], vec![10, 12]]),
            Vec::<Vec<i32>>::new()
        );
        assert_eq!(
            Solution::interval_intersection(vec![vec![1, 7]], vec![vec![3, 10]]),
            vec![vec![3, 7]]
        );
    }

    #[bench]
    fn vec_1k(b: &mut Bencher) {
        let mut v1 = Vec::with_capacity(1_000);
        let mut v2 = Vec::with_capacity(1_000);
        for i in 0..1_000 {
            v1.push(vec![i, i + 5]);
            v2.push(vec![i + 2, i + 7]);
        }
        test::black_box(&v1);
        test::black_box(&v2);
        b.iter(|| Solution::interval_intersection(v1.clone(), v2.clone()));
    }
}
