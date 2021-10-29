//
// Problem 19. Remove Nth Node From End of List (Medium)
// https://leetcode.com/problems/remove-nth-node-from-end-of-list/
//
// 0ms (100%)/2.1MB
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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        fn bt(head: Option<Box<ListNode>>, n: i32) -> (Option<Box<ListNode>>, i32) {
            match head {
                None => (None, 1),
                Some(mut head) => {
                    let (rest, b) = bt(head.next, n);
                    if b != n {
                        head.next = rest;
                        (Some(head), b + 1)
                    } else {
                        (rest, b + 1)
                    }
                }
            }
        }
        bt(head, n).0
    }
}
