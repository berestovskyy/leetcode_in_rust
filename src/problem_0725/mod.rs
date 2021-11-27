//
// Problem 725. Split Linked List in Parts (Medium)
// See [on LeetCode](https://leetcode.com/problems/split-linked-list-in-parts/).
//
// 0ms (100%)/2MB (81%)
// Space complexity: O(n)
// Runtime complexity: O(n²)
//
// TODO: list tests
//

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution {}

impl Solution {
    pub fn split_list_to_parts(
        mut head: Option<Box<ListNode>>,
        k: i32,
    ) -> Vec<Option<Box<ListNode>>> {
        let k = k as usize;
        let mut n = 0;
        let mut it = &head;
        while let Some(node) = it {
            it = &node.next;
            n += 1;
        }
        let mut res = vec![None; k];
        for i in 0..k {
            res[i] = head;
            let mut it = &mut res[i];
            for _ in 0..if i < n % k { n / k + 1 } else { n / k } {
                if let Some(node) = it {
                    it = &mut node.next;
                }
            }
            head = it.take();
        }
        res
    }
}
