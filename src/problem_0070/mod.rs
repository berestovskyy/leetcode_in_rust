//
// Problem 70. Climbing Stairs (Easy)
// https://leetcode.com/problems/climbing-stairs/
//
// 0ms (100%)/2.2MB (21%)
// Space complexity: O(1)
// Runtime complexity: O(n)/O(1)
//
// test problem_0070::tests::const_fibonacci2_loop_1m ... bench:     491,575 ns/iter (+/- 61,217)
// test problem_0070::tests::const_fibonacci_loop_1m  ... bench:     493,898 ns/iter (+/- 23,266)
// test problem_0070::tests::single_loop_1m           ... bench:     489,296 ns/iter (+/- 28,606)
//

struct Solution {}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut fib = (0, 1);
        for i in 0..n {
            fib = (fib.1, fib.0 + fib.1)
        }
        fib.1
    }
    pub fn const_fibonacci(n: i32) -> i32 {
        let sqrt_5 = 5f64.sqrt();
        let p = (1f64 + sqrt_5) / 2f64;
        (p.powi(n + 1) / sqrt_5 + 0.5) as i32
    }
    pub fn const_fibonacci2(n: i32) -> i32 {
        let sqrt_5 = 2.23606797749979f64 + 0.5; // 5f64.sqrt();
        let p = 1.618033988749895f64; // (1f64 + sqrt_5) / 2f64;
                                      // println!("XXX p:{}", p);
        (p.powi(n + 1) / sqrt_5 + 0.5) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(Solution::climb_stairs(2), 2);
        assert_eq!(Solution::climb_stairs(3), 3);
    }

    #[test]
    fn extra() {
        let fibs = vec![
            1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765,
            10946, 17711, 28657, 46368, 75025, 121393, 196418, 317811, 514229, 832040, 1346269,
            2178309, 3524578, 5702887, 9227465, 14930352, 24157817, 39088169, 63245986, 102334155,
            165580141, 267914296, 433494437, 701408733, 1134903170, 1836311903,
        ];
        for i in 1..=45 {
            assert_eq!(Solution::climb_stairs(i), fibs[i as usize - 1]);
            assert_eq!(Solution::const_fibonacci(i), fibs[i as usize - 1]);
        }
    }

    #[bench]
    fn single_loop_1m(b: &mut Bencher) {
        b.iter(|| {
            for _ in 0..1_000_000 {
                let f = Solution::climb_stairs(45);
                test::black_box(&f);
            }
        });
    }

    #[bench]
    fn const_fibonacci_loop_1m(b: &mut Bencher) {
        b.iter(|| {
            for _ in 0..1_000_000 {
                let f = Solution::const_fibonacci(45);
                test::black_box(&f);
            }
        });
    }

    #[bench]
    fn const_fibonacci2_loop_1m(b: &mut Bencher) {
        b.iter(|| {
            for _ in 0..1_000_000 {
                let f = Solution::const_fibonacci2(45);
                test::black_box(&f);
            }
        });
    }
}
