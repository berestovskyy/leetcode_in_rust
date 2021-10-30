//
// Problem 899. Orderly Queue (Hard)
// https://leetcode.com/problems/orderly-queue/
//
// 0ms (100%)/2.1MB
// Space complexity: O(n)
// Runtime complexity: O(n)
//
// test problem_0899::tests::string_1k_1     ... bench:      98,339 ns/iter (+/- 17,299)
// test problem_0899::tests::string_1k_3     ... bench:       2,198 ns/iter (+/- 390)
// test problem_0899::tests::string_1k_same  ... bench:      42,978 ns/iter (+/- 2,545)
//

struct Solution {}

impl Solution {
    pub fn orderly_queue(s: String, k: i32) -> String {
        match k {
            1 => {
                let rep = s.bytes().chain(s.bytes()).collect::<Vec<_>>();
                let mut v = rep.windows(s.len()).collect::<Vec<_>>();
                v.sort_unstable();
                String::from_utf8(v[0].to_vec()).unwrap()
            }
            _ => {
                let mut v = Vec::from(s);
                v.sort_unstable();
                String::from_utf8(v).unwrap()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(Solution::orderly_queue("cba".into(), 1), "acb");
    }

    #[bench]
    fn string_1k_1(b: &mut Bencher) {
        let mut s = String::with_capacity(1_000);
        for i in 0..1_000 / 5 {
            s.push('a');
            s.push('b');
            s.push('c');
            s.push('d');
            s.push('e');
        }
        test::black_box(&s);
        b.iter(|| Solution::orderly_queue(s.clone(), 1));
    }

    #[bench]
    fn string_1k_3(b: &mut Bencher) {
        let mut s = String::with_capacity(1_000);
        for i in 0..1_000 / 5 {
            s.push('a');
            s.push('b');
            s.push('c');
            s.push('d');
            s.push('e');
        }
        test::black_box(&s);
        b.iter(|| Solution::orderly_queue(s.clone(), 3));
    }

    #[bench]
    fn string_1k_same(b: &mut Bencher) {
        let mut s = String::with_capacity(1_000);
        for i in 0..1_000 {
            s.push('a');
        }
        test::black_box(&s);
        b.iter(|| Solution::orderly_queue(s.clone(), 1));
    }
}
