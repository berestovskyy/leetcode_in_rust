//
// Problem 733. Flood Fill (Easy)
// https://leetcode.com/problems/flood-fill/
//
// 0ms (100%)/2MB
// Space complexity: O(1)
// Runtime complexity: O(n)
//
// test problem_0733::tests::vec_1k          ... bench:      16,687 ns/iter (+/- 2,494)
//

struct Solution {}

impl Solution {
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
        let (m, n) = (image.len(), image[0].len());
        let (sr, sc) = (sr as usize, sc as usize);
        let old_color = image[sr][sc];
        if old_color == new_color {
            return image;
        }
        let mut bfs = vec![(sr, sc)];
        while let Some((r, c)) = bfs.pop() {
            if r >= m || c >= n || image[r][c] != old_color {
                continue;
            }
            image[r][c] = new_color;
            bfs.push((r + 1, c));
            bfs.push((r.overflowing_sub(1).0, c));
            bfs.push((r, c + 1));
            bfs.push((r, c.overflowing_sub(1).0));
        }
        image
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(
            Solution::flood_fill(vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]], 1, 1, 2),
            vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]]
        );
        assert_eq!(
            Solution::flood_fill(vec![vec![0, 0, 0], vec![0, 0, 0]], 0, 0, 2),
            vec![vec![2, 2, 2], vec![2, 2, 2]]
        );
    }

    #[bench]
    fn vec_1k(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        for i in 0..1_000 / 10 {
            v.push(vec![1, 1, 0, 2, 2]);
            v.push(vec![0, 1, 1, 1, 1]);
        }
        test::black_box(&v);
        b.iter(|| Solution::flood_fill(v.clone(), 0, 0, 2));
    }
}
