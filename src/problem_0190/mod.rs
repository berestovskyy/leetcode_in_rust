//
// Problem 190. Reverse Bits (Easy)
// https://leetcode.com/problems/reverse-bits/
//
// 0ms (100%)/1.9MB (100%)
// Space complexity: O(1)
// Runtime complexity: O(1)
//
// test problem_0190::tests::single_1k                ... bench:       4,271 ns/iter (+/- 251)
//

struct Solution {}

impl Solution {
    pub fn reverse_bits(mut x: u32) -> u32 {
        let mut res = 0;
        for _ in 0..8 {
            res <<= 4;
            res += match x & 0xf {
                0x0 => 0x0,
                0x1 => 0x8,
                0x2 => 0x4,
                0x3 => 0xc,
                0x4 => 0x2,
                0x5 => 0xa,
                0x6 => 0x6,
                0x7 => 0xe,
                0x8 => 0x1,
                0x9 => 0x9,
                0xa => 0x5,
                0xb => 0xd,
                0xc => 0x3,
                0xd => 0xb,
                0xe => 0x7,
                _ => 0xf,
            };
            x >>= 4;
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
            Solution::reverse_bits(0b00000010100101000001111010011100),
            0b00111001011110000010100101000000
        );
        assert_eq!(
            Solution::reverse_bits(0b11111111111111111111111111111101),
            0b10111111111111111111111111111111
        );
    }

    #[bench]
    fn single_1k(b: &mut Bencher) {
        b.iter(|| {
            for _ in 0..1_000 {
                Solution::reverse_bits(0b11111111111111111111111111111101);
            }
        });
    }
}
