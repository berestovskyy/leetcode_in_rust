//
// Problem 282. Expression Add Operators (Hard)
// See [on LeetCode](https://leetcode.com/problems/expression-add-operators/).
//
// 120ms (80%)/2.6MB (20%)
// Space complexity: O(3^n)
// Runtime complexity: O(3^n)
//
// test problem_0282::tests::single                   ... bench:      44,909 ns/iter (+/- 5,348)
//

struct Solution {}

impl Solution {
    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        fn dfs(
            num: &str,
            start: usize,
            target: i64,
            path: String,
            prev: i64,
            cur: i64,
            mut res: Vec<String>,
        ) -> Vec<String> {
            if start == num.len() && prev + cur == target {
                res.push(path.clone());
            }
            let mut i = 1;
            while start + i <= num.len() {
                let s = &num[start..start + i];
                let n = s.parse::<i64>();
                if let Result::Ok(n) = n {
                    if n.to_string().len() != s.len() {
                        return res;
                    }
                    if start == 0 {
                        res = dfs(num, i, target, s.to_string(), 0, n, res);
                    } else {
                        res = dfs(
                            num,
                            start + i,
                            target,
                            path.clone() + "+" + s,
                            prev + cur,
                            n,
                            res,
                        );
                        res = dfs(
                            num,
                            start + i,
                            target,
                            path.clone() + "-" + s,
                            prev + cur,
                            -n,
                            res,
                        );
                        res = dfs(
                            num,
                            start + i,
                            target,
                            path.clone() + "*" + s,
                            prev,
                            cur * n,
                            res,
                        );
                    }
                }
                i += 1;
            }
            res
        }
        dfs(&num, 0, target as i64, "".into(), 0, 0, Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(Solution::add_operators("123".into(), 6).len(), 2);
        assert_eq!(Solution::add_operators("232".into(), 8).len(), 2);
        assert_eq!(Solution::add_operators("00".into(), 0).len(), 3);
        assert_eq!(Solution::add_operators("3456237490".into(), 9191).len(), 0);
    }

    #[bench]
    fn single(b: &mut Bencher) {
        b.iter(|| Solution::add_operators("12345".into(), 9191));
    }
}
