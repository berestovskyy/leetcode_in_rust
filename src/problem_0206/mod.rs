//
// Problem 206. Reverse Linked List (Easy)
// https://leetcode.com/problems/reverse-linked-list/
//
// 0ms (100%)/2.5MB
// Space complexity: O(1)
// Runtime complexity: O(n)
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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut cur = head;
        let mut prev = None;
        while let Some(mut node) = cur {
            cur = node.next;
            node.next = prev;
            prev = Some(node);
        }
        prev
    }
}
