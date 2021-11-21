//
// Problem 1286. Iterator for Combination (Medium)
// See [on LeetCode](https://leetcode.com/problems/iterator-for-combination/).
//
// 3ms (87%)/3.3MB (77%)
// Space complexity: O(1)
// Runtime complexity: O(n²)?
//
// test problem_1286::tests::string_26                ... bench:         105 ns/iter (+/- 9)
//

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

/**
 * Your CombinationIterator object will be instantiated and called as such:
 * let obj = CombinationIterator::new(characters, combinationLength);
 * let ret_1: String = obj.next();
 * let ret_2: bool = obj.has_next();
 */
struct CombinationIterator {
    indexes: Vec<usize>,
    chars: Vec<char>,
    length: usize,
    has_next: bool,
}

impl CombinationIterator {
    fn new(characters: String, combination_length: i32) -> Self {
        let length = combination_length as usize;
        let chars = characters.chars().collect();
        let indexes = (0..length).collect();
        Self {
            indexes,
            chars,
            length,
            has_next: true,
        }
    }

    fn next(&mut self) -> String {
        let result = self.indexes.iter().map(|&i| self.chars[i]).collect();
        if let Some((change_index, &original_index)) = self
            .indexes
            .iter()
            .enumerate()
            .rev()
            .find(|(i, &index)| self.indexes.len() + index < self.chars.len() + i)
        {
            for i in change_index..self.indexes.len() {
                self.indexes[i] = original_index + 1 + i - change_index;
            }
        } else {
            self.has_next = false;
        }
        result
    }

    fn has_next(&self) -> bool {
        self.has_next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        let mut iter = CombinationIterator::new("abc".into(), 2);
        assert_eq!(iter.next(), "ab");
        assert_eq!(iter.has_next(), true);
        assert_eq!(iter.next(), "ac");
        assert_eq!(iter.has_next(), true);
        assert_eq!(iter.next(), "bc");
        assert_eq!(iter.has_next(), false);
    }

    #[bench]
    fn string_26(b: &mut Bencher) {
        let mut iter = CombinationIterator::new("abcdefghijklmnopqrstuvwxyz".into(), 26);
        test::black_box(&iter);
        b.iter(|| iter.next());
    }
}
