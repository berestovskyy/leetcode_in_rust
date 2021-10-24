//
// Problem 278. First Bad Version (Easy)
// https://leetcode.com/problems/first-bad-version/
//
// 0ms/2.1MB
// Space complexity: O(1)
// Runtime complexity: avg: O(log n)
//
// test problem_0278::tests::first_bad_version_1k ... bench:       3,421 ns/iter (+/- 470)
//

struct Solution {
    pub bad_version: i32,
}

impl Solution {
    #[allow(non_snake_case)]
    pub fn isBadVersion(&self, v: i32) -> bool {
        std::thread::yield_now();
        v == self.bad_version
    }
}

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let (mut l, mut r) = (1, n);
        while l < r {
            let m = l + (r - l) / 2;
            match self.isBadVersion(m) {
                true => r = m,
                false => l = m + 1,
            }
        }
        l
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn all() {
        let s = Solution { bad_version: 4 };
        assert_eq!(s.first_bad_version(5), 4);
        let s = Solution { bad_version: 1 };
        assert_eq!(s.first_bad_version(1), 1);
    }

    #[bench]
    fn first_bad_version_1k(b: &mut Bencher) {
        let s = Solution { bad_version: 500 };
        test::black_box(&s);
        b.iter(|| s.first_bad_version(1000));
    }
}
