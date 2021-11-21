//
// Problem 203. Remove Linked List Elements (Easy)
// See [on LeetCode](https://leetcode.com/problems/reverse-linked-list/).
//
// 4ms (83%)/2.6MB (100%)
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
    pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut ptr = &mut head;
        loop {
            match ptr {
                None => break,
                Some(node) if node.val == val => {
                    *ptr = node.next.take();
                }
                Some(node) => {
                    ptr = &mut node.next;
                }
            }
        }
        head
    }
}
