//
// Problem 876. Middle of the Linked List (Easy)
// https://leetcode.com/problems/middle-of-the-linked-list/
//
// 0ms/2.1MB
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
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut normal, mut double) = (&head, &head);
        while double.is_some() {
            double = &double.as_ref().unwrap().next;
            if double.is_some() {
                double = &double.as_ref().unwrap().next;
                normal = &normal.as_ref().unwrap().next;
            }
        }
        normal.clone()
    }
}
